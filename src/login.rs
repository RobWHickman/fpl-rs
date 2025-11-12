use crate::urls;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde_json;
use serde_json::json;

pub fn login_requests(
    access_token: &str,
    client: &Client,
) -> Result<(Vec<StatusCode>, String), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let mut json_body = json!({
        "id": serde_json::Value::Null,
        "parameters": {
            "buttonValue": "SIGNON",
            "username": std::env::var("EMAIL")?,
            "password": std::env::var("PASSWORD")?,
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

        response_json = response.error_for_status()?.json::<serde_json::Value>()?;

        headers.insert(
            "interactionId",
            HeaderValue::from_str(response_json["interactionId"].as_str().unwrap())?,
        );
        json_body["id"] = response_json["id"].clone();
        request_url = format!(
            "{}{}/{}/capabilities/customHTMLTemplate",
            urls::BASE_ACCOUNT_URL,
            urls::LOGIN_PATH,
            response_json["connectionId"].as_str().unwrap()
        );
    }

    let dv_response = response_json["dvResponse"]
        .as_str()
        .ok_or("dvResponse not found")?
        .to_string();

    Ok((status_responses, dv_response))
}
