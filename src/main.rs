use std::error::Error;

use crate::extract_json_to_one_word::extract_json_to_one_word;

mod extract_json_to_one_word;
mod load_json;

fn main() -> Result<(), Box<dyn Error>> {
    extract_json_to_one_word()?;

    Ok(())
}
