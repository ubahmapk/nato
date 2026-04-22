use clap::Parser;
use nato::convert;
use std::io::{self, BufRead};

/// Convert text to NATO phonetic alphabet.
#[derive(Parser)]
#[command(name = "nato", version, about)]
struct Args {
    /// Text to convert. If omitted, reads one line from stdin.
    input: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let input = match args.input {
        Some(text) => text,
        None => {
            let stdin = io::stdin();
            stdin.lock().lines().next().unwrap_or(Ok(String::new()))?
        }
    };

    for entry in convert(&input) {
        match entry.word {
            Some(word) => println!("{} - {}", entry.character.to_uppercase(), word),
            None => println!("{} - (no NATO equivalent)", entry.character),
        }
    }

    Ok(())
}
