use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// NatoEntry uses Option<&'static str> which cannot cross the WASM boundary.
// This owned DTO is serialized to a JS value via serde-wasm-bindgen.
#[derive(Serialize, Deserialize)]
struct NatoEntryDto {
    pub character: String,
    pub word: Option<String>,
}

impl From<nato::NatoEntry> for NatoEntryDto {
    fn from(e: nato::NatoEntry) -> Self {
        NatoEntryDto {
            character: e.character.to_string(),
            word: e.word.map(|s| s.to_string()),
        }
    }
}

/// Converts input text to NATO phonetic alphabet.
/// Returns a JS array of `{character, word}` objects.
/// `word` is `null` for unrecognized characters.
#[wasm_bindgen]
pub fn convert_to_nato(input: &str) -> Result<JsValue, JsValue> {
    let entries: Vec<NatoEntryDto> = nato::convert(input)
        .into_iter()
        .map(NatoEntryDto::from)
        .collect();
    serde_wasm_bindgen::to_value(&entries).map_err(|e| JsValue::from_str(&e.to_string()))
}
