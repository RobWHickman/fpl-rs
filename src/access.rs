use crate::fpl_constants;
use crate::urls;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::StatusCode;

pub fn auth_request(
    pkce_challenge: String,
    initial_state: String,
    client: &Client,
) -> Result<(StatusCode, String, String), Box<dyn std::error::Error>> {
    let url = format!("{}{}", urls::BASE_ACCOUNT_URL, urls::AUTH_PATH);

    let params = [
        ("client_id", fpl_constants::CLIENT_ID),
        ("redirect_uri", urls::BASE_FANTASY_URL),
        ("response_type", "code"),
        ("state", &initial_state),
        ("code_challenge", &pkce_challenge),
        ("code_challenge_method", "S256"),
    ];

    let response = client.get(url).query(&params).send()?.error_for_status()?;

    let status = response.status();
    let html = response.text()?;
    let access_token = extract_html_access_token(&html)?;
    let state = extract_html_state(&html)?;

    Ok((status, access_token, state))
}

fn extract_html_access_token(html: &str) -> Result<String, String> {
    let re = Regex::new(r#""accessToken":"([^"]+)""#).unwrap();
    re.captures(html)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| "Access token not found".to_string())
}
