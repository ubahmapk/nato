# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-04-22

### Added

- `char_to_nato()`: maps a single character (A–Z, 0–9) to its NATO phonetic word,
  case-insensitively. Digits use ICAO standard words (9 → "Niner"). Returns `None`
  for unrecognised characters.
- `convert()`: converts a full string to `Vec<NatoEntry>`, skipping whitespace.
  Returns structured data so any front-end can format it independently.
- CLI (`nato`): accepts text as a positional argument or reads one line from stdin.
  Formats output as `X - Word` per line; unknown characters print `(no NATO equivalent)`.
- 7 unit tests covering: basic words, case-insensitivity, digits, the "Niner"
  convention, space-skipping, empty input, and unknown characters.

[Unreleased]: https://github.com/ubahmapk/nato/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ubahmapk/nato/releases/tag/v0.1.0
