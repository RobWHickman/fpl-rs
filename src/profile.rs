use crate::urls;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
use std::fmt;

#[derive(Debug)]
pub struct Profile {
    id: i32,
    name: String,
    email: Option<String>,
    sso_id: String,
    access_token: String,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Profile:")?;
        writeln!(f, "  id: {}", self.id)?;
        writeln!(f, "  name: {}", self.name)?;
        writeln!(f, "  email: {:?}", self.email)?;
        writeln!(f, "  sso_id: {}", self.sso_id)?;
        write!(f, "  access_token: {}", self.access_token)
    }
}

pub fn profile_request(
    access_token: &str,
    client: &Client,
) -> Result<(StatusCode, Profile), Box<dyn std::error::Error>> {
    let url = format!("{}{}", urls::BASE_FANTASY_URL, urls::ME_PATH);
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-API-Authorization",
        HeaderValue::from_str(&format!("Bearer {}", access_token))?,
    );

    let response = client.get(&url).headers(headers.clone()).send()?;

    let status = response.status();
    let response_json = response.error_for_status()?.json::<serde_json::Value>()?;

    let profile = Profile {
        id: response_json["player"]["entry"]
            .as_i64()
            .ok_or("id not found")? as i32,
        name: format!(
            "{} {}",
            response_json["player"]["first_name"].as_str().unwrap_or(""),
            response_json["player"]["last_name"].as_str().unwrap_or("")
        ),
        email: response_json["player"]["email"]
            .as_str()
            .map(|s| s.to_string()),
        sso_id: response_json["player"]["sso_id"]
            .as_str()
            .ok_or("sso_id not found")?
            .to_string(),
        access_token: access_token.to_string(),
    };

    Ok((status, profile))
}
