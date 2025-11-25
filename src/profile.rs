use crate::error::FplError;
use crate::urls;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
use secrecy::{ExposeSecret, SecretString};
use std::fmt;

#[derive(Debug)]
pub struct Profile {
    id: i64,
    name: String,
    email: Option<String>,
    sso_id: String,
    access_token: SecretString,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Profile:")?;
        writeln!(f, "  id: {}", self.id)?;
        writeln!(f, "  name: {}", self.name)?;
        writeln!(f, "  email: {:?}", self.email)?;
        writeln!(f, "  sso_id: {}", self.sso_id)?;
        write!(f, "  access_token: {:?}", self.access_token) // secret string redacted
    }
}

impl Profile {
    #[must_use]
    pub fn access_token(&self) -> &str {
        self.access_token.expose_secret()
    }
}

pub fn profile_request(
    access_token: SecretString,
    client: &Client,
) -> Result<(StatusCode, Profile), FplError> {
    let url = format!("{}{}", urls::BASE_FANTASY_URL, urls::ME_PATH);
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-API-Authorization",
        HeaderValue::from_str(&format!("Bearer {}", access_token.expose_secret()))?,
    );

    let response = client.get(&url).headers(headers.clone()).send()?;

    let status = response.status();
    let response_json = response.error_for_status()?.json::<serde_json::Value>()?;

    let profile = Profile {
        id: response_json["player"]["entry"]
            .as_i64()
            .ok_or_else(|| FplError::JsonField("id".to_string()))? as i64,
        name: format!(
            "{} {}",
            response_json["player"]["first_name"].as_str().unwrap_or(""),
            response_json["player"]["last_name"].as_str().unwrap_or("")
        ),
        email: response_json["player"]["email"]
            .as_str()
            .map(std::string::ToString::to_string),
        sso_id: response_json["player"]["sso_id"]
            .as_str()
            .ok_or_else(|| FplError::JsonField("sso_id".to_string()))?
            .to_string(),
        access_token,
    };

    Ok((status, profile))
}
