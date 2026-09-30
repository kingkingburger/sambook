use std::{error::Error, println, time::Duration};

use chrono::Local;
use tokio::time;

use crate::{
    env_config::{EnvObject, get_env},
    get_3_word::get_3_word,
    send_discord_alert::send_discord_alert,
};

pub async fn get_3_word_then_send_discord_in_loop() -> Result<(), Box<dyn Error>> {
    let EnvObject { internal_time, .. } = get_env()?;

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

        // 3단어를 하나의 String으로 나타내기 위함
        let words = picked
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>() // 컴파일러 자동 추론
            .join(",");

        if let Err(e) = send_discord_alert(words).await {
            eprintln!("디스코드 전송 실패: {e}")
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
