use std::error::Error;

use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct EnvObject {
    discord_webhook_url: String,
    internal_time: u64,
    port: u64,
}

pub fn get_env() -> Result<Json<EnvObject>, Box<dyn Error>> {
    let _ = dotenvy::dotenv();

    let discord_webhook_url = std::env::var("DISCORD_WEBHOOK_URL")?;
    let internal_time = std::env::var("INTERNAL_TIME")?.parse::<u64>().unwrap();
    let port = std::env::var("PORT")?.parse::<u64>().unwrap();

    Ok(Json(EnvObject {
        discord_webhook_url: discord_webhook_url,
        internal_time: internal_time,
        port: port,
    }))
}
