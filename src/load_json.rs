use crate::Error;
use serde_json::Value;

pub fn load_json(path: String) -> Result<Value, Box<dyn Error>> {
    let s = std::fs::read_to_string(path)?;
    let value: Value = serde_json::from_str(&s)?;

    Ok(value)
}
