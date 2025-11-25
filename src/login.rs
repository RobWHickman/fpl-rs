use crate::error::FplError;
use crate::secrets::LoginSecrets;
use crate::urls;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use secrecy::{ExposeSecret, SecretString};
use serde_json;
use serde_json::json;

pub fn login_requests(
    access_token: &SecretString,
    client: &Client,
) -> Result<(Vec<StatusCode>, String), FplError> {
    let secrets = LoginSecrets::from_env()?;
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token.expose_secret()))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let mut json_body = json!({
        "id": serde_json::Value::Null,
        "parameters": {
            "buttonValue": "SIGNON",
            "username": secrets.email(),
            "password": secrets.password().expose_secret(),
        },
        "eventName": "continue",
    });

    let mut request_url = format!("{}{}", urls::BASE_ACCOUNT_URL, urls::START_PATH);
    let mut response_json = serde_json::Value::Null;
    let mut status_responses = Vec::new();

    for _ in 0..4 {
        let response = client
            .post(&request_url)
            .headers(headers.clone())
            .json(&json_body)
            .send()?;

        let status = response.status();
        status_responses.push(status);

        let status = response.status();
        status_responses.push(status);

        response_json = response.error_for_status()?.json::<serde_json::Value>()?;

        let interaction_id = response_json["interactionId"]
            .as_str()
            .ok_or_else(|| FplError::JsonField("interactionId".to_string()))?;

        headers.insert("interactionId", HeaderValue::from_str(interaction_id)?);

        let response_id: &str = response_json["id"]
            .as_str()
            .ok_or_else(|| FplError::JsonField("id".to_string()))?;
        json_body["id"] = serde_json::Value::String(response_id.to_string());

        let connection_id = response_json["connectionId"]
            .as_str()
            .ok_or_else(|| FplError::JsonField("connectionId".to_string()))?;

        request_url = format!(
            "{}{}/{}/capabilities/customHTMLTemplate",
            urls::BASE_ACCOUNT_URL,
            urls::LOGIN_PATH,
            connection_id
        );
    }

    let dv_response = response_json["dvResponse"]
        .as_str()
        .ok_or_else(|| FplError::JsonField("dvResponse".to_string()))?
        .to_string();

    Ok((status_responses, dv_response))
}
