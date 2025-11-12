use fpl_rs::auth::auth_request;
use fpl_rs::login::interaction_id_request;
use fpl_rs::pkce::pkce_init;
use reqwest::blocking::Client;

fn main() {
    let client: Client = Client::new();
    let (pkce_verifier, pkce_challenge, initial_state) = pkce_init();
    println!(
        "PKCE INIT:\n{}, {}, {}",
        pkce_verifier, pkce_challenge, initial_state
    );

    let (fpl_auth_code, fpl_auth_token, returned_state) =
        auth_request(pkce_challenge, initial_state, &client).unwrap();
    println!(
        "AUTH REQUEST:\n{}",
        format!("{}: {}", fpl_auth_code, fpl_auth_token)
    );

    let (fpl_interaction_code, fpl_interaction_id) =
        interaction_id_request(fpl_auth_token, &client).unwrap();
    println!(
        "INTERACTION REQUEST:\n{}",
        format!("{}: {}", fpl_interaction_code, fpl_interaction_id)
    );
}
