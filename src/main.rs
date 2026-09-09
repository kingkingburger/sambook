use std::error::Error;

use crate::{get_3_word::get_3_word, send_discord_alert::send_discord_alert};

mod get_3_word;
mod send_discord_alert;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();

    // 3단어를 , 로 묶어서 1번에 보내기
    let picked = get_3_word()?;

    let words = picked
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect::<Vec<&str>>()
        .join(",");

    println!("{}", words);
    send_discord_alert(words).await?;

    Ok(())
}
