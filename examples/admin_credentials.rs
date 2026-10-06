use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use std::io::{self, Read};

fn random<const N: usize>() -> [u8; N] {
    let mut bytes = [0u8; N];
    getrandom::fill(&mut bytes).expect("secure randomness is unavailable");
    bytes
}

fn main() {
    let generated = std::env::args().any(|arg| arg == "--generate");
    let password = if generated {
        URL_SAFE_NO_PAD.encode(random::<24>())
    } else {
        let mut password = String::new();
        io::stdin()
            .read_to_string(&mut password)
            .expect("could not read password from stdin");
        password.trim_end_matches(['\r', '\n']).to_string()
    };
    if password.len() < 12 || password.len() > 4096 {
        eprintln!("Use a password between 12 and 4096 bytes.");
        std::process::exit(1);
    }
    let salt = SaltString::encode_b64(&random::<16>()).unwrap();
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let mut result = serde_json::json!({
        "ADMIN_PASSWORD_HASH": hash,
        "ADMIN_SESSION_SECRET": URL_SAFE_NO_PAD.encode(random::<48>()),
    });
    if generated {
        result["password"] = password.into();
    }
    println!("{result}");
}
