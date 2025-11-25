// Simple smoke tests for various FPL endpoints

use fpl_rs::urls;
use reqwest::blocking::Client;

#[test]
fn test_base_account_url_is_reachable() {
    let client = Client::new();
    let url = format!("{}{}", urls::BASE_ACCOUNT_URL, urls::AUTH_PATH);

    let response = client.get(&url).send();

    assert!(response.is_ok());

    let response = response.unwrap();
    assert!(
        response.status().is_client_error()
            || response.status().is_success()
            || response.status().is_redirection(),
    );
}

#[test]
fn test_base_fantasy_url_is_reachable() {
    let client = Client::new();
    let response = client.get(urls::BASE_FANTASY_URL).send();

    assert!(response.is_ok());

    let response = response.unwrap();
    assert!(response.status().is_success() || response.status().is_redirection());
}

#[test]
fn test_fantasy_api_me_endpoint_exists() {
    let client = Client::new();
    let url = format!("{}{}", urls::BASE_FANTASY_URL, urls::ME_PATH);

    let response = client.get(&url).send();

    assert!(response.is_ok());

    let response = response.unwrap();
    assert!(!response.status().is_server_error(),);
}

#[test]
fn test_account_token_endpoint_exists() {
    let client = Client::new();
    let url = format!("{}{}", urls::BASE_ACCOUNT_URL, urls::TOKEN_PATH);

    let response = client.post(&url).send();

    assert!(response.is_ok(),);

    let response = response.unwrap();
    assert!(!response.status().is_server_error(),);
}

#[test]
fn test_account_resume_endpoint_exists() {
    let client = Client::new();
    let url = format!("{}{}", urls::BASE_ACCOUNT_URL, urls::RESUME_PATH);

    let response = client.post(&url).send();

    assert!(response.is_ok(),);

    let response = response.unwrap();
    assert!(!response.status().is_server_error(),);
}

#[test]
fn test_davinci_start_endpoint_exists() {
    let client = Client::new();
    let url = format!("{}{}", urls::BASE_ACCOUNT_URL, urls::START_PATH);

    let response = client.post(&url).send();

    assert!(response.is_ok(),);

    let response = response.unwrap();
    assert!(!response.status().is_server_error());
}
