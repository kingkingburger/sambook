use std::error::Error;

use crate::get_3_word::get_3_word;

mod extract_json_to_one_word;
mod get_3_word;
mod load_json;
mod refine_word;

fn main() -> Result<(), Box<dyn Error>> {
    let picked = get_3_word()?;
    for word in picked {
        println!("{}", word);
    }

    Ok(())
}
