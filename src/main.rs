use std::{
    error::Error,
    println,
    time::{Duration, Instant},
};

use tokio::time;

use crate::{get_3_word::get_3_word, send_discord_alert::send_discord_alert};

mod get_3_word;
mod send_discord_alert;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut interval = time::interval(Duration::from_hours(1));

    loop {
        interval.tick().await;

        let _ = dotenvy::dotenv();
        let start = Instant::now();
        // 3단어를 , 로 묶어서 1번에 보내기
        let picked = get_3_word()?;
        let elapsed = start.elapsed();
        println!("실행시간: {:?}", elapsed);

        let words = picked
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<&str>>()
            .join(",");

        println!("{}", words);

        let send_start = Instant::now();
        send_discord_alert(words).await?;
        let send_elapsed = send_start.elapsed();
        println!("실행시간: {:?}", send_elapsed);
    }
    // Ok(())
}
