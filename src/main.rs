use std::{error::Error, println};

use crate::load_json::load_json;

mod load_json;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_json = load_json(String::from("raw_data/1_5000_20260719.json"))?;
    println!("{}", serde_json::to_string_pretty(&raw_json)?);
    
    Ok(())
}
