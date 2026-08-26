use std::error::Error;

use crate::refine_word::refine_word;

mod extract_json_to_one_word;
mod load_json;
mod refine_word;

fn main() -> Result<(), Box<dyn Error>> {
    refine_word()?;

    Ok(())
}
