// Simple non-exhaustive tests that the pkce generation is working
// somewhat as expected

use data_encoding::BASE64URL_NOPAD;
use fpl_rs::pkce::pkce_init;

#[test]
fn test_pkce_init() {
    let (verifier, challenge, state) = pkce_init();

    assert!(!verifier.is_empty(), "Verifier should not be empty");
    assert!(!challenge.is_empty(), "Challenge should not be empty");
    assert!(!state.is_empty(), "State should not be empty");
}

#[test]
fn test_pkce_verifier_valid() {
    let (verifier, _challenge, _state) = pkce_init();

    assert!(
        BASE64URL_NOPAD.decode(verifier.as_bytes()).is_ok(),
        "Verifier should be valid base64url encoding"
    );
}

#[test]
fn test_pkce_challenge_valid() {
    let (_verifier, challenge, _state) = pkce_init();

    assert!(
        BASE64URL_NOPAD.decode(challenge.as_bytes()).is_ok(),
        "Challenge should be valid base64url encoding"
    );
}

#[test]
fn test_pkce_uniqueness() {
    let (verifier1, challenge1, state1) = pkce_init();
    let (verifier2, challenge2, state2) = pkce_init();

    assert_ne!(verifier1, verifier2, "Verifiers should be unique");
    assert_ne!(challenge1, challenge2, "Challenges should be unique");
    assert_ne!(state1, state2, "States should be unique");
}
