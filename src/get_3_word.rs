use std::error::Error;

use axum::Json;
use rand::seq::IndexedRandom;
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::Value;

pub fn get_3_word() -> Result<Vec<Value>, Box<dyn Error>> {
    let mut rng = rand::rng();
    let word_original = std::fs::read_to_string("data/words.json")?;
    let word_original_to_string: Value = serde_json::from_str(&word_original)?;
    let words_object = &word_original_to_string
        .get("words")
        .unwrap()
        .as_array()
        .unwrap();

    let picked = words_object
        .sample(&mut rng, 3)
        .cloned()
        .collect::<Vec<Value>>();

    Ok(picked)
}

// response type
#[derive(Serialize)]
pub struct Words {
    word: String,
}

// router이자 service 합쳐놓은 것.
pub async fn get_3_word_to_json() -> Result<Json<Words>, (StatusCode, String)> {
    let picked = get_3_word().map_err(|e| {
        eprintln!("문자 가져오기 실패: {e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error occurred".to_string(),
        )
    })?;

    let words = picked
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect::<Vec<&str>>()
        .join(",");

    Ok(Json(Words { word: words }))
}
