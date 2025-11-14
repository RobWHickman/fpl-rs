use env_logger;
use fpl_rs::auth::{access_request, access_token_exchange, auth_request};
use fpl_rs::login::login_requests;
use fpl_rs::pkce::pkce_init;
use fpl_rs::profile::profile_request;
use log::{debug, info};
use reqwest::blocking::Client;
use dotenv::dotenv;

fn main() {
    dotenv().ok();  
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    let client: Client = Client::new();
    let (pkce_verifier, pkce_challenge, initial_state) = pkce_init();

    let (_fpl_auth_code, fpl_auth_token, returned_state) =
        auth_request(pkce_challenge, initial_state, &client).unwrap();
    debug!(
        "[AUTH] Status: {}, Token: {:?}",
        _fpl_auth_code, fpl_auth_token
    );

    let (_fpl_login_statuses, fpl_dv_response) = login_requests(&fpl_auth_token, &client).unwrap();
    debug!(
        "[LOGIN] Statuses: {:?}, DV Response: {}",
        _fpl_login_statuses, fpl_dv_response
    );

    let (_fpl_access_status, fpl_auth_code) =
        access_request(fpl_dv_response, returned_state).unwrap();
    debug!(
        "[ACCESS] Status: {}, Auth Code: {:?}",
        _fpl_access_status, fpl_auth_code
    );

    let (_fpl_exchange_status, fpl_access_code) =
        access_token_exchange(fpl_auth_code, pkce_verifier, &client).unwrap();
    debug!(
        "[EXCHANGE] Status: {}, Access Token: {:?}",
        _fpl_exchange_status, fpl_access_code
    );

    let (_fpl_profile_status, fpl_profile) = profile_request(fpl_access_code, &client).unwrap();
    debug!("[PROFILE] Status: {}", _fpl_profile_status);

    info!("{}", fpl_profile);
    info!("\nAccess Token: {}", fpl_profile.access_token());
}
