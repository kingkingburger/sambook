use std::error::Error;

use reqwest::Client;
use serde_json::json;

pub async fn send_discord_alert(message: String) -> Result<(), Box<dyn Error>> {
    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL")?;
    let client = Client::new();

    let payload = json!({
        "content": message,
        "username": "sambook"
    });

    client.post(webhook_url).json(&payload).send().await?;

    Ok(())
}
