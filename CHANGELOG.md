# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased]

## [0.1.3] - 2026-05-13

### Added
- AWS IAM policy document and creation script (`infrastructure/iam-caddy-route53.json`,
  `infrastructure/create-iam-policy.sh`) for granting Caddy the Route53 permissions needed
  for DNS-01 Let's Encrypt certificate issuance and renewal

### Changed
- README updated with IAM setup instructions under the "Web interface (Docker)" section

## [0.1.2] - 2026-05-11

### Added
- Dynamic CSP hash calculation: script hash is now recomputed on every build, eliminating manual updates after each frontend change

### Fixed
- Corrected 308 redirect when running on localhost

### Changed
- GitHub Actions upgraded: `actions/checkout` v4→v6 and `actions/cache` v4→v5 for Node.js 24 compatibility ahead of June 2 mandatory cutover
- Docker image builds now only trigger on tagged pushes, not on every push to `main`
- Updated installation and usage instructions in documentation

### Removed
- Defunct `Caddyfile` removed from repository

## [0.1.1] - 2026-05-11

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
[Unreleased]: https://github.com/ubahmapk/nato/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/ubahmapk/nato/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/ubahmapk/nato/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/ubahmapk/nato/releases/tag/v0.1.1
[0.1.0]: https://github.com/ubahmapk/nato/releases/tag/v0.1.0
