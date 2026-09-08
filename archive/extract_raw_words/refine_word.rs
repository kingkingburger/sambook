use std::error::Error;

use crate::extract_json_to_one_word::extract_json_to_one_word;

pub fn refine_word() -> Result<(), Box<dyn Error + 'static>> {
    let mut hangle_collection: Vec<String> = Vec::new();
    hangle_collection.push(String::from("1_5000_20260719"));
    hangle_collection.push(String::from("2_5000_20260719"));
    hangle_collection.push(String::from("3_5000_20260719"));
    hangle_collection.push(String::from("4_5000_20260719"));
    hangle_collection.push(String::from("5_5000_20260719"));
    hangle_collection.push(String::from("6_5000_20260719"));
    hangle_collection.push(String::from("7_5000_20260719"));
    hangle_collection.push(String::from("8_5000_20260719"));
    hangle_collection.push(String::from("9_5000_20260719"));
    hangle_collection.push(String::from("10_5000_20260719"));
    hangle_collection.push(String::from("11_3671_20260719"));

    let mut all_words: Vec<String> = Vec::new();

    for file_name in hangle_collection {
        let word_array = extract_json_to_one_word(format!("raw_data/{file_name}.json"))?;
        all_words.extend(word_array);
        // println!("{}", serde_json::to_string_pretty(&word_array)?);
    }

    let result = serde_json::json!({
        "words": all_words
    });

    let json = serde_json::to_string_pretty(&result)?;
    std::fs::write("data/words.json", json)?;

    Ok(())
}
