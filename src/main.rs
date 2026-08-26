use std::{error::Error, println};

use serde_json::Value;

mod extract_json_to_one_word;
mod load_json;
mod refine_word;

fn main() -> Result<(), Box<dyn Error>> {
    let s = std::fs::read_to_string("data/words.json")?;
    let value: Value = serde_json::from_str(&s)?;
    let manf = value.get("words").unwrap();
    let words = serde_json::to_string_pretty(manf)?;

    println!("{}", words.len());

    Ok(())
}
