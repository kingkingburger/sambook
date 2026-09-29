use std::error::Error;

use reqwest::Client;
use serde_json::json;

use crate::env_config::{EnvObject, get_env};

pub async fn send_discord_alert(message: String) -> Result<(), Box<dyn Error>> {
    let EnvObject {
        discord_webhook_url,
        ..
    } = get_env()?;
    
    let client = Client::new();

    let payload = json!({
        "content": message,
        "username": "sambook"
    });

    client
        .post(discord_webhook_url)
        .json(&payload)
        .send()
        .await?;

    Ok(())
}
