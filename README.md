# nato

Convert text to the NATO phonetic alphabet from the command line.

(Blatently vibe-coded with Claude)

[![CI](https://github.com/ubahmapk/nato/actions/workflows/ci.yml/badge.svg)](https://github.com/ubahmapk/nato/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## What it does

Spell out any text letter-by-letter using the standard NATO phonetic alphabet:

```
$ nato hello
H - Hotel
E - Echo
L - Lima
L - Lima
O - Oscar
```

Digits use the ICAO standard words (0 → Zero, 9 → Niner). Spaces are silently
skipped; any other unrecognised character prints `(no NATO equivalent)`.

## Install

**From source:**

```sh
git clone https://github.com/ubahmapk/nato
cd nato
cargo install --path .
```

**From crates.io (once published):**

```sh
cargo install nato
```

## Usage

Pass text as an argument:

```sh
nato SOS
nato "call sign bravo"
nato 007
```

Or pipe text from stdin — useful in scripts:

```sh
echo "alpha bravo" | nato
printf "hello" | nato
```

When no argument is given the tool reads one line from stdin.

## How it works

The conversion logic lives entirely in `src/lib.rs` as a public library, separate
from the CLI in `src/main.rs`. The core function `convert()` returns structured
data (`Vec<NatoEntry>`) rather than a pre-formatted string, so future interfaces
— a web API, a GUI, a WASM module — can call the same engine and format the
output however they need.

## Roadmap

- [ ] Additional output modes (`--format json`, `--format compact`)
- [ ] Multi-line stdin support (process all lines, not just the first)
- [ ] Web interface (WASM or small HTTP server)
- [ ] GUI front-end
- [ ] Full punctuation support (ITU/ICAO special characters)

## License

This project is licensed under the [MIT License](LICENSE).
