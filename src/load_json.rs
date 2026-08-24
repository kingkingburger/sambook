use std::fs::File;
use crate::Error;
use serde_json::Value;

pub fn load_json(path: String) -> Result<Value, Box<dyn Error>>{
    
    // let file = File::open("raw_data/1_5000_20260719.json")?;
    let file = File::open(path)?;
    let value: Value = serde_json::from_reader(file)?;
    
    Ok(value)
}
