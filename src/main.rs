use std::error::Error;

use rand::seq::IndexedRandom;
use serde_json::Value;

mod extract_json_to_one_word;
mod load_json;
mod refine_word;

fn main() -> Result<(), Box<dyn Error>> {
    let s = std::fs::read_to_string("data/words.json")?;
    let value: Value = serde_json::from_str(&s)?;
    let words_object = &value.get("words").unwrap().as_array().unwrap();
    let mut rng = rand::rng();

    let picked = words_object.sample(&mut rng, 3);
    for word in picked {
        println!("{}", word);
    }

    Ok(())
}
