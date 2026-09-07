use std::error::Error;

use rand::seq::IndexedRandom;
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
