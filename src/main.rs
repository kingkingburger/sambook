use std::error::Error;

use crate::{get_3_word::get_3_word, send_discord_alert::send_discord_alert};

mod get_3_word;
mod send_discord_alert;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();

    let picked = get_3_word()?;
    for word in picked {
        send_discord_alert(word.to_string()).await?;
        println!("{}", word);
    }

    Ok(())
}
