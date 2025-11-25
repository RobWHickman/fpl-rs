use dotenv::dotenv;
use fpl_rs::auth::{access_request, access_token_exchange, auth_request};
use fpl_rs::error::FplError;
use fpl_rs::login::login_requests;
use fpl_rs::pkce::pkce_init;
use fpl_rs::profile::profile_request;
use log::{debug, info};
use reqwest::blocking::Client;

fn main() -> Result<(), FplError> {
    dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let client: Client = Client::new();
    let (pkce_verifier, pkce_challenge, initial_state) = pkce_init();

    let (fpl_auth_code, fpl_auth_token, returned_state) =
        auth_request(&pkce_challenge, &initial_state, &client)?;
    debug!("[AUTH] Status: {fpl_auth_code}, Token: {fpl_auth_token:?}");

    let (fpl_login_statuses, fpl_dv_response) = login_requests(&fpl_auth_token, &client)?;
    debug!("[LOGIN] Statuses: {fpl_login_statuses:?}, DV Response: {fpl_dv_response}");

    let (fpl_access_status, fpl_auth_code) = access_request(fpl_dv_response, returned_state)?;
    debug!("[ACCESS] Status: {fpl_access_status}, Auth Code: {fpl_auth_code:?}");

    let (fpl_exchange_status, fpl_access_code) =
        access_token_exchange(&fpl_auth_code, &pkce_verifier, &client)?;
    debug!("[EXCHANGE] Status: {fpl_exchange_status}, Access Token: {fpl_access_code:?}");

    let (fpl_profile_status, fpl_profile) = profile_request(fpl_access_code, &client)?;
    debug!("[PROFILE] Status: {fpl_profile_status}");

    info!("{fpl_profile}");
    info!("\nAccess Token: {}", fpl_profile.access_token());
    Ok(())
}
