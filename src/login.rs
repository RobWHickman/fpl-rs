use crate::urls;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json;

pub fn interaction_id_request(
    access_token: String,
    client: &Client,
) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
    let url = format!("{}{}", urls::BASE_ACCOUNT_URL, urls::START_PATH);

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/json")
        .send()?;

    let status = response.status();
    let json = response.error_for_status()?.json::<serde_json::Value>()?;
    let interaction_id = json["interactionId"]
        .as_str()
        .ok_or("interactionId not found")?
        .to_string();

    Ok((status, interaction_id))
}
