# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased]

### Added
- Cargo workspace: split into `crates/nato` (library), `crates/nato-cli` (binary), and `crates/nato-wasm` (WebAssembly)
- `nato-wasm`: WebAssembly bindings via `wasm-bindgen`; exposes `convert_to_nato()` returning a JS array of `{character, word}` objects
- Web frontend (SvelteKit 2 + Svelte 5 + TypeScript) under `web/`; runs entirely in-browser via WASM, no server-side rendering
- Three output formats: human-readable (`C – Charlie`), compact (space-joined words), and JSON
- Clipboard copy button for converted output
- In-session conversion history (Enter to save; clicking an entry repopulates the input)
- Docker deployment: three-stage build (Rust→WASM, Svelte→static, Caddy serving static files)
- Automatic TLS via Let's Encrypt DNS-01 challenge (AWS Route53), supporting private-network deployments
- GitHub Actions release workflow and GHCR Docker image hosting

### Fixed
- Copy button no longer overlaps output in compact format mode

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

<!-- next-url -->

[Unreleased]: https://github.com/ubahmapk/nato/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ubahmapk/nato/releases/tag/v0.1.0
