use super::*;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};

fn config() -> Config {
    let salt = SaltString::encode_b64(b"test-salt-value!!").unwrap();
    let hash = Argon2::default()
        .hash_password(b"strong test password", &salt)
        .unwrap()
        .to_string();
    Config::new(hash, "a".repeat(48), "https://rust-dd.com".into()).unwrap()
}

#[test]
fn signed_sessions_expire_and_reject_forgery() {
    let cfg = config();
    let token = cfg.sign(100, "testnonce12345678");
    assert!(cfg.verify(&token, 101));
    assert!(!cfg.verify(&token, 100 + SESSION_TTL));
    assert!(!cfg.verify(&token, 99));
    assert!(!cfg.verify(&format!("{token}x"), 101));
    assert!(!cfg.verify("not.a.session", 101));
}

#[test]
fn secret_and_password_rotation_revoke_sessions() {
    let mut cfg = config();
    let token = cfg.sign(100, "testnonce12345678");
    assert!(cfg.verify(&token, 101));
    cfg.secret = "b".repeat(48);
    assert!(!cfg.verify(&token, 101));
    cfg.secret = "a".repeat(48);
    cfg.hash.push('x');
    assert!(!cfg.verify(&token, 101));
}

#[test]
fn configuration_fails_closed() {
    assert!(Config::new("plaintext".into(), "a".repeat(48), "https://rust-dd.com".into()).is_none());
    let cfg = config();
    assert!(Config::new(cfg.hash.clone(), "short".into(), cfg.origin.clone()).is_none());
    assert!(Config::new(cfg.hash, "a".repeat(48), "http://rust-dd.com".into()).is_none());
}

#[test]
fn cookies_are_private_and_duplicates_are_rejected() {
    let cookie = config().cookie("token", SESSION_TTL);
    for attribute in ["HttpOnly", "Secure", "SameSite=Strict", "Path=/", "Max-Age=2592000"] {
        assert!(cookie.contains(attribute));
    }
    let mut headers = http::HeaderMap::new();
    headers.insert(http::header::COOKIE, "other=one; rd_admin=token".parse().unwrap());
    assert_eq!(cookie_token(&headers).as_deref(), Some("token"));
    headers.append(http::header::COOKIE, "rd_admin=second".parse().unwrap());
    assert!(cookie_token(&headers).is_none());
}

#[test]
fn mutations_require_the_exact_origin() {
    assert!(allowed_origin(Some("https://rust-dd.com"), "https://rust-dd.com"));
    for origin in [
        None,
        Some("null"),
        Some("https://evil.example"),
        Some("https://rust-dd.com.evil.example"),
    ] {
        assert!(!allowed_origin(origin, "https://rust-dd.com"));
    }
}

#[test]
fn sixth_login_attempt_is_blocked_for_fifteen_minutes() {
    let mut attempts = Attempts::default();
    let now = Instant::now();
    for _ in 0..5 {
        assert!(attempts.admit("127.0.0.1", now));
    }
    assert!(!attempts.admit("127.0.0.1", now));
    assert!(attempts.admit("127.0.0.2", now));
    assert!(attempts.admit("127.0.0.1", now + std::time::Duration::from_secs(901)));
}
