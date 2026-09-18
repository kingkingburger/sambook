use std::{error::Error, println};

use axum::{Router, routing::get};

use crate::{
    get_3_word::get_3_word_to_json,
    get_3_word_then_send_discord::get_3_word_then_send_discord_in_loop,
};

mod get_3_word;
mod get_3_word_then_send_discord;
mod send_discord_alert;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ===== 환경변수 가져오기 =====
    let _ = dotenvy::dotenv();
    std::env::var("DISCORD_WEBHOOK_URL")?;
    std::env::var("INTERNAL_TIME")?.parse::<u64>().unwrap();
    let port = std::env::var("PORT")?.parse::<u64>().unwrap();

    // ===== api 서버 띄우기 =====
    let app = Router::new().route("/word", get(get_3_word_to_json));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    let server = tokio::spawn(async move {
        println!("Server running on http://localhost:{port}");
        if let Err(e) = axum::serve(listener, app).await {
            eprintln!("Api 서버 오류: {e}");
        }
    });

    let worker = tokio::spawn(async move {
        if let Err(e) = get_3_word_then_send_discord_in_loop().await {
            eprintln!("단어보내기 오류 {e}");
        }
    });

    tokio::select! {
        _ = server => {}
        _ = worker => {}
    }

    Ok(())
}
