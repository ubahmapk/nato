/// A single input character paired with its NATO phonetic word.
/// `word` is None for characters with no NATO equivalent.
#[derive(Debug, Clone)]
pub struct NatoEntry {
    pub character: char,
    pub word: Option<&'static str>,
}

/// Maps one character to its NATO phonetic word.
/// Input is treated case-insensitively. Returns None for unrecognized characters.
pub fn char_to_nato(c: char) -> Option<&'static str> {
    match c.to_ascii_lowercase() {
        'a' => Some("Alpha"),
        'b' => Some("Bravo"),
        'c' => Some("Charlie"),
        'd' => Some("Delta"),
        'e' => Some("Echo"),
        'f' => Some("Foxtrot"),
        'g' => Some("Golf"),
        'h' => Some("Hotel"),
        'i' => Some("India"),
        'j' => Some("Juliet"),
        'k' => Some("Kilo"),
        'l' => Some("Lima"),
        'm' => Some("Mike"),
        'n' => Some("November"),
        'o' => Some("Oscar"),
        'p' => Some("Papa"),
        'q' => Some("Quebec"),
        'r' => Some("Romeo"),
        's' => Some("Sierra"),
        't' => Some("Tango"),
        'u' => Some("Uniform"),
        'v' => Some("Victor"),
        'w' => Some("Whiskey"),
        'x' => Some("X-ray"),
        'y' => Some("Yankee"),
        'z' => Some("Zulu"),
        '0' => Some("Zero"),
        '1' => Some("One"),
        '2' => Some("Two"),
        '3' => Some("Three"),
        '4' => Some("Four"),
        '5' => Some("Five"),
        '6' => Some("Six"),
        '7' => Some("Seven"),
        '8' => Some("Eight"),
        '9' => Some("Niner"),
        _ => None,
    }
}

/// Converts a string to a list of NatoEntry values.
/// Spaces are silently skipped — no entry is produced for them.
/// All other characters produce an entry; unknown ones have `word: None`.
pub fn convert(input: &str) -> Vec<NatoEntry> {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| NatoEntry {
            word: char_to_nato(c),
            character: c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_word() {
        let entries = convert("hello");
        let words: Vec<_> = entries.iter().map(|e| e.word.unwrap()).collect();
        assert_eq!(words, ["Hotel", "Echo", "Lima", "Lima", "Oscar"]);
    }

    #[test]
    fn case_insensitive() {
        let entries = convert("SOS");
        let words: Vec<_> = entries.iter().map(|e| e.word.unwrap()).collect();
        assert_eq!(words, ["Sierra", "Oscar", "Sierra"]);
    }

    #[test]
    fn digits() {
        let entries = convert("007");
        let words: Vec<_> = entries.iter().map(|e| e.word.unwrap()).collect();
        assert_eq!(words, ["Zero", "Zero", "Seven"]);
    }

    #[test]
    fn digit_nine_is_niner() {
        assert_eq!(char_to_nato('9'), Some("Niner"));
    }

    #[test]
    fn spaces_skipped() {
        let entries = convert("hi there");
        assert_eq!(entries.len(), 7);
        assert_eq!(entries[2].word.unwrap(), "Tango");
    }

    #[test]
    fn empty_input() {
        assert!(convert("").is_empty());
    }

    #[test]
    fn unknown_char_gives_none() {
        let entries = convert("@");
        assert_eq!(entries.len(), 1);
        assert!(entries[0].word.is_none());
    }
}
