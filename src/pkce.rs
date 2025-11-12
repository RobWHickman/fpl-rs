use data_encoding::BASE64URL_NOPAD;
use getrandom::fill;
use sha2::{Digest, Sha256};
use uuid::Uuid;

fn pkce_verifier() -> String {
    let mut bytes = [0u8; 96];
    fill(&mut bytes).unwrap();
    BASE64URL_NOPAD.encode(&bytes)
}

fn pkce_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let digest = hasher.finalize();

    BASE64URL_NOPAD.encode(&digest)
}

pub fn pkce_init() ->(String, String, String) {
    let state = Uuid::new_v4().simple().to_string();
    let verifier = pkce_verifier();
    let challenge = pkce_challenge(&verifier);
    (verifier, challenge, state)
}
