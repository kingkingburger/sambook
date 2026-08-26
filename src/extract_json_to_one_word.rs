use std::error::Error;

use crate::load_json::load_json;

pub fn extract_json_to_one_word() -> Result<Vec<String>, Box<dyn Error + 'static>> {
    let raw_json = load_json(String::from("raw_data/1_5000_20260719.json"))?;
    let raw_hangle_array = raw_json
        .get("LexicalResource")
        .and_then(|x| x.get("Lexicon"))
        .and_then(|x| x.get("LexicalEntry"))
        .unwrap()
        .as_array()
        .unwrap();

    let mut word_result: Vec<String> = Vec::new();

    for (index, _raw_hagnle) in raw_hangle_array.iter().enumerate() {
        let one_raw_hangle = raw_json
            .get("LexicalResource")
            .and_then(|x| x.get("Lexicon"))
            .and_then(|x| x.get("LexicalEntry"))
            .and_then(|x| x.get(index))
            .and_then(|x| x.get("Lemma"))
            .and_then(|x| x.get("feat"))
            .and_then(|x| x.get("val"))
            .and_then(|x| x.as_str());

        if let Some(one_raw_hangle) = one_raw_hangle {
            if one_raw_hangle != "null" {
                word_result.push(one_raw_hangle.to_string());
            }
        }
    }

    Ok(word_result)
}
