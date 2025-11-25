use crate::error::FplError;
use crate::urls;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::LOCATION;
use reqwest::StatusCode;
use secrecy::{ExposeSecret, SecretString};

pub fn auth_request(
    pkce_challenge: &str,
    initial_state: &str,
    client: &Client,
) -> Result<(StatusCode, SecretString, String), Box<dyn std::error::Error>> {
    let url = format!("{}{}", urls::BASE_ACCOUNT_URL, urls::AUTH_PATH);

    let params = [
        ("client_id", urls::CLIENT_ID),
        ("redirect_uri", urls::BASE_FANTASY_URL),
        ("response_type", "code"),
        ("state", initial_state),
        ("code_challenge", pkce_challenge),
        ("code_challenge_method", "S256"),
    ];

    let response = client.get(url).query(&params).send()?.error_for_status()?;

    let status = response.status();
    let html = response.text()?;
    let access_token = extract_html_access_token(&html)?;
    let access_token_secret = SecretString::new(access_token.into_boxed_str());
    let state = extract_html_state(&html)?;

    Ok((status, access_token_secret, state))
}

fn extract_html_access_token(html: &str) -> Result<String, FplError> {
    let re = Regex::new(r#""accessToken":"([^"]+)""#)?;
    re.captures(html)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or( FplError::AccessTokenNotFound)
}

fn extract_html_state(html: &str) -> Result<String, FplError> {
    let re = Regex::new(r#"<input[^>]+name="state"[^>]+value="([^"]+)""#)?;
    re.captures(html)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or( FplError::StateNotFound)
}

pub fn access_request(
    dv_response: String,
    new_state: String,
) -> Result<(StatusCode, SecretString), Box<dyn std::error::Error>> {
    let no_redirect_client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let url = format!("{}{}", urls::BASE_ACCOUNT_URL, urls::RESUME_PATH);

    let params = [("dvResponse", dv_response), ("state", new_state)];

    let response: reqwest::blocking::Response =
        no_redirect_client.post(url).form(&params).send()?;

    let status = response.status();

    let location = response
        .headers()
        .get(LOCATION)
        .ok_or("Location header not found")?
        .to_str()?;

    let auth_code = extract_html_auth_code(location)?;
    let auth_code_secret = SecretString::new(auth_code.into_boxed_str());
    Ok((status, auth_code_secret))
}

fn extract_html_auth_code(location: &str) -> Result<String, FplError> {
    let re = Regex::new(r"[?&]code=([^&]+)")?;
    re.captures(location)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or( FplError::AuthCodeNotFound)
}

pub fn access_token_exchange(
    auth_code: &SecretString,
    verifier: &str,
    client: &Client,
) -> Result<(StatusCode, SecretString), Box<dyn std::error::Error>> {
    let url: String = format!("{}{}", urls::BASE_ACCOUNT_URL, urls::TOKEN_PATH);

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", urls::BASE_FANTASY_URL),
        ("code", auth_code.expose_secret()),
        ("code_verifier", verifier),
        ("client_id", urls::CLIENT_ID),
    ];

    let response = client.post(url).form(&params).send()?.error_for_status()?;

    let status = response.status();
    let response_json = response.error_for_status()?.json::<serde_json::Value>()?;
    let access_token = response_json["access_token"]
        .as_str()
        .ok_or("access_token not found")?
        .to_string();
    let access_token_secret = SecretString::new(access_token.into_boxed_str());

    Ok((status, access_token_secret))
}
