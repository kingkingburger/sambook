use std::{error::Error, fs::File, println};

use serde_json::Value;

fn decode_json() -> Result<(), Box<dyn Error>>{
    
    let file = File::open("raw_data/1_5000_20260719.json")?;
    let value: Value = serde_json::from_reader(file)?;

    println!("{}", serde_json::to_string_pretty(&value)?);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    decode_json()?;
    println!("Hello, world!");
    Ok(())
}
