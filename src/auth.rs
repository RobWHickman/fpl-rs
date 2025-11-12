use crate::fpl_constants;
use crate::urls;
use reqwest::blocking::Client;
use reqwest::StatusCode;

pub fn auth_request(
    pkce_challenge: String,
    initial_state: String,
    client: &Client,
) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
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

    Ok((status, html))
}
