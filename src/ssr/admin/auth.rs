use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use dioxus::fullstack::{FullstackContext, ServerFnError};
use dioxus::prelude::Result;
use hmac::{Hmac, Mac};
use http::{header, HeaderMap};
use sha2::Sha256;

pub const SESSION_TTL: u64 = 30 * 24 * 60 * 60;
const ATTEMPT_WINDOW: Duration = Duration::from_secs(15 * 60);
static ATTEMPTS: LazyLock<Mutex<Attempts>> = LazyLock::new(|| Mutex::new(Attempts::default()));
static HASH_SLOTS: tokio::sync::Semaphore = tokio::sync::Semaphore::const_new(2);

pub struct Config {
    pub hash: String,
    pub secret: String,
    pub origin: String,
}

impl Config {
    pub fn new(hash: String, secret: String, origin: String) -> Option<Self> {
        let parsed = PasswordHash::new(&hash).ok()?;
        if parsed.algorithm.as_str() != "argon2id" || parsed.hash.is_none() || secret.len() < 32 {
            return None;
        }
        let origin = origin.trim_end_matches('/').to_string();
        let uri = origin.parse::<http::Uri>().ok()?;
        let loopback = matches!(uri.host(), Some("localhost" | "127.0.0.1" | "[::1]"));
        if uri.host().is_none()
            || origin.contains('@')
            || uri.query().is_some()
            || uri.path() != "/"
            || !(uri.scheme_str() == Some("https") || (loopback && uri.scheme_str() == Some("http")))
        {
            return None;
        }
        Some(Self { hash, secret, origin })
    }

    pub fn from_env() -> Option<Self> {
        Self::new(
            std::env::var("ADMIN_PASSWORD_HASH").ok()?,
            std::env::var("ADMIN_SESSION_SECRET").ok()?,
            std::env::var("ADMIN_ORIGIN").unwrap_or_else(|_| "https://rust-dd.com".into()),
        )
    }

    fn mac(&self, payload: &str) -> Hmac<Sha256> {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes()).expect("HMAC accepts any key length");
        mac.update(self.hash.as_bytes());
        mac.update(b"\0");
        mac.update(payload.as_bytes());
        mac
    }

    pub fn sign(&self, now: u64, nonce: &str) -> String {
        let payload = format!("{now}.{nonce}");
        let signature = URL_SAFE_NO_PAD.encode(self.mac(&payload).finalize().into_bytes());
        format!("{payload}.{signature}")
    }

    pub fn verify(&self, token: &str, now: u64) -> bool {
        let Some((payload, signature)) = token.rsplit_once('.') else {
            return false;
        };
        let Some((issued, nonce)) = payload.split_once('.') else {
            return false;
        };
        let Ok(issued) = issued.parse::<u64>() else {
            return false;
        };
        if now < issued || now - issued >= SESSION_TTL || !(16..=64).contains(&nonce.len()) {
            return false;
        }
        let Ok(signature) = URL_SAFE_NO_PAD.decode(signature) else {
            return false;
        };
        self.mac(payload).verify_slice(&signature).is_ok()
    }

    pub fn cookie(&self, token: &str, max_age: u64) -> String {
        let secure = if self.origin.starts_with("https://") {
            "; Secure"
        } else {
            ""
        };
        format!("rd_admin={token}; Path=/; HttpOnly; SameSite=Strict; Max-Age={max_age}{secure}")
    }
}

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn error(code: u16, message: impl Into<String>) -> dioxus::CapturedError {
    ServerFnError::ServerError {
        code,
        message: message.into(),
        details: None,
    }
    .into()
}

fn context() -> Result<FullstackContext> {
    FullstackContext::current().ok_or_else(|| error(401, "Please sign in to the admin panel."))
}

pub fn cookie_token(headers: &HeaderMap) -> Option<String> {
    let mut token = None;
    for value in headers.get_all(header::COOKIE) {
        for cookie in value.to_str().ok()?.split(';') {
            let Some((name, value)) = cookie.trim().split_once('=') else {
                continue;
            };
            if name == "rd_admin" {
                if token.is_some() {
                    return None;
                }
                token = Some(value.to_string());
            }
        }
    }
    token
}

pub fn allowed_origin(origin: Option<&str>, expected: &str) -> bool {
    origin == Some(expected)
}

pub fn require_admin() -> Result<Config> {
    let config = Config::from_env().ok_or_else(|| error(503, "Admin sign-in is not configured."))?;
    let headers = context()?.parts_mut().headers.clone();
    if !cookie_token(&headers).is_some_and(|token| config.verify(&token, now())) {
        return Err(error(401, "Your session has expired. Sign in again to continue."));
    }
    Ok(config)
}

pub fn require_origin(config: &Config) -> Result<()> {
    let headers = context()?.parts_mut().headers.clone();
    if !allowed_origin(
        headers.get(header::ORIGIN).and_then(|value| value.to_str().ok()),
        &config.origin,
    ) {
        return Err(error(403, "This request did not come from the admin panel."));
    }
    Ok(())
}

#[derive(Default)]
pub struct Attempts {
    entries: HashMap<String, (Instant, usize)>,
    global: Option<(Instant, usize)>,
}

impl Attempts {
    pub fn admit(&mut self, key: &str, now: Instant) -> bool {
        self.entries
            .retain(|_, (start, _)| now.duration_since(*start) < ATTEMPT_WINDOW);
        if self
            .global
            .is_none_or(|(start, _)| now.duration_since(start) >= ATTEMPT_WINDOW)
        {
            self.global = Some((now, 0));
        }
        let (_, global_count) = self.global.as_mut().unwrap();
        if *global_count >= 50 || (self.entries.len() >= 4096 && !self.entries.contains_key(key)) {
            return false;
        }
        let (_, count) = self.entries.entry(key.into()).or_insert((now, 0));
        if *count >= 5 {
            return false;
        }
        *count += 1;
        *global_count += 1;
        true
    }
}

pub async fn login(password: String) -> Result<()> {
    let config = Config::from_env().ok_or_else(|| error(503, "Admin sign-in is not configured."))?;
    require_origin(&config)?;
    let ctx = context()?;
    let ip = ctx
        .extension::<super::middleware::ClientIp>()
        .map(|ip| ip.0)
        .unwrap_or_else(|| "unknown".into());
    if !ATTEMPTS.lock().unwrap().admit(&ip, Instant::now()) {
        return Err(error(429, "Too many sign-in attempts. Try again in 15 minutes."));
    }
    if password.is_empty() || password.len() > 4096 {
        return Err(error(401, "Incorrect password."));
    }
    let _slot = HASH_SLOTS
        .try_acquire()
        .map_err(|_| error(429, "Sign-in is busy. Try again shortly."))?;
    let hash = config.hash.clone();
    let valid = tokio::task::spawn_blocking(move || {
        PasswordHash::new(&hash)
            .ok()
            .is_some_and(|hash| Argon2::default().verify_password(password.as_bytes(), &hash).is_ok())
    })
    .await
    .map_err(|_| error(500, "Sign-in failed. Try again."))?;
    if !valid {
        return Err(error(401, "Incorrect password."));
    }
    ATTEMPTS.lock().unwrap().entries.remove(&ip);
    let mut nonce = [0u8; 24];
    getrandom::fill(&mut nonce).map_err(|_| error(500, "Sign-in failed. Try again."))?;
    let token = config.sign(now(), &URL_SAFE_NO_PAD.encode(nonce));
    ctx.add_response_header(
        header::SET_COOKIE,
        config.cookie(&token, SESSION_TTL).parse::<http::HeaderValue>().unwrap(),
    );
    Ok(())
}

pub fn logout() -> Result<()> {
    let config = require_admin()?;
    require_origin(&config)?;
    context()?.add_response_header(
        header::SET_COOKIE,
        config.cookie("", 0).parse::<http::HeaderValue>().unwrap(),
    );
    Ok(())
}

#[cfg(test)]
#[path = "auth_tests.rs"]
mod tests;
