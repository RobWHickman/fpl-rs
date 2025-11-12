use data_encoding::BASE64URL_NOPAD;
use getrandom::fill;
use sha2::{Digest, Sha256};

pub fn pkce_verifier() -> String {
    let mut bytes = [0u8; 96];
    fill(&mut bytes).unwrap();
    BASE64URL_NOPAD.encode(&bytes)
}

pub fn pkce_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let digest = hasher.finalize();

    BASE64URL_NOPAD.encode(&digest)
}
