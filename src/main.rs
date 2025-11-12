use fpl_rs::auth::auth_request;
use fpl_rs::pkce::pkce_init;
use reqwest::blocking::Client;

fn main() {
    let client: Client = Client::new();
    let (pkce_verifier, pkce_challenge, initial_state) = pkce_init();

    let (auth_code, auth_token, auth_state) =
        auth_request(pkce_challenge, initial_state, &client).unwrap();
    println!("{}", format!("{}: {}", auth_code, auth_token));
}
