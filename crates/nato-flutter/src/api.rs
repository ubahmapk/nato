use flutter_rust_bridge::frb;

pub struct NatoEntryDto {
    pub character: String,
    pub word: Option<String>,
}

impl From<nato::NatoEntry> for NatoEntryDto {
    fn from(e: nato::NatoEntry) -> Self {
        NatoEntryDto {
            character: e.character.to_string(),
            word: e.word.map(str::to_string),
        }
    }
}

#[frb(sync)]
pub fn convert_nato(input: String) -> Vec<NatoEntryDto> {
    nato::convert(&input)
        .into_iter()
        .map(NatoEntryDto::from)
        .collect()
}
