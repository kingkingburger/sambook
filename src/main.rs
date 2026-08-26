use std::error::Error;

use crate::extract_json_to_one_word::extract_json_to_one_word;

mod extract_json_to_one_word;
mod load_json;

fn main() -> Result<(), Box<dyn Error>> {
    let word_array = extract_json_to_one_word()?;

    println!("{}", serde_json::to_string_pretty(&word_array)?);
    Ok(())
}
