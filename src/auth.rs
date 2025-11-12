use crate::fpl_constants;
use crate::urls;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::LOCATION;
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

fn extract_html_state(html: &str) -> Result<String, String> {
    let re = Regex::new(r#"<input[^>]+name="state"[^>]+value="([^"]+)""#).unwrap();
    re.captures(html)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| "State not found".to_string())
}

pub fn access_request(
    dv_response: String,
    new_state: String,
) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
    let no_redirect_client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    
    let url = format!("{}{}", urls::BASE_ACCOUNT_URL, urls::RESUME_PATH);

    let params = [("dvResponse", dv_response), ("state", new_state)];

    let response: reqwest::blocking::Response = no_redirect_client
        .post(url)
        .form(&params)
        .send()?;

    let status = response.status();
    println!("Status: {}", status);
    println!("Headers: {:?}", response.headers());

    let location = response
        .headers()
        .get(LOCATION)
        .ok_or("Location header not found")?
        .to_str()?;

    let auth_code = extract_html_auth_code(location)?;
    Ok((status, auth_code))
}

fn extract_html_auth_code(location: &str) -> Result<String, Box<dyn std::error::Error>> {
    let re = Regex::new(r"[?&]code=([^&]+)")?;
    re.captures(location)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| "Auth code not found".into())
}
