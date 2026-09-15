use std::{error::Error, println, time::Duration};

use chrono::Local;
use tokio::time;

use crate::{get_3_word::get_3_word, send_discord_alert::send_discord_alert};

mod get_3_word;
mod send_discord_alert;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    std::env::var("DISCORD_WEBHOOK_URL")?;
    let internal_time = std::env::var("INTERNAL_TIME")?.parse::<u64>().unwrap();

    let mut interval = time::interval(Duration::from_secs(internal_time));
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        interval.tick().await;

        // 3단어를 , 로 묶어서 1번에 보내기
        let picked = match get_3_word() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("단어 뽑기 실패: {e}");
                continue;
            }
        };

        let words = picked
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<&str>>()
            .join(",");

        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("[{now}] {words}");

        if let Err(e) = send_discord_alert(words).await {
            eprintln!("디스코드 전송 실패: {e}")
        }
    }
}
