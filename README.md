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

## Why

For fun.

## Install

**From source:**

```sh
git clone https://github.com/ubahmapk/nato
cd nato
cargo install --path .
```

## Web interface (Docker)

A pre-built image is published to the GitHub Container Registry and serves the
web interface (Svelte + WASM) via Caddy.

**Local testing** — no domain or TLS configuration required:

```sh
docker run -p 8080:80 ghcr.io/ubahmapk/nato
```

Open http://localhost:8080. Caddy automatically uses its internal CA.

**Production deployment** — custom domain with Let's Encrypt TLS via Route53
DNS-01:

```sh
SITE_DOMAIN=nato.example.com \
AWS_ACCESS_KEY_ID=AKIA... \
AWS_SECRET_ACCESS_KEY=... \
AWS_REGION=us-east-1 \
docker compose -f web/docker-compose.yml up -d
```

| Variable | Required | Description |
|---|---|---|
| `SITE_DOMAIN` | Yes | Public hostname; omit or set to `localhost` for local testing |
| `AWS_ACCESS_KEY_ID` | Yes (prod) | IAM key for Route53 DNS-01 challenge |
| `AWS_SECRET_ACCESS_KEY` | Yes (prod) | IAM secret |
| `AWS_REGION` | No | Defaults to `us-east-1` |
| `AWS_HOSTED_ZONE_ID` | No | Set to allow a stricter IAM policy (skips zone-discovery API calls) |

The `caddy_data` volume holds TLS certificates — do not delete it between
restarts or Let's Encrypt rate limits may apply.

### IAM setup

Caddy uses the Route53 DNS-01 challenge to obtain and renew certificates. The AWS identity
must have permission to write `_acme-challenge` TXT records in your hosted zone.

Create the least-privilege policy with the provided script (requires the AWS CLI):

```sh
./infrastructure/create-iam-policy.sh Z1D633PJN98FT9   # replace with your hosted zone ID
```

Then attach the printed policy ARN to the IAM role or user that the container authenticates as.

**Prefer an IAM role** (EC2 instance role or ECS task role) over static keys — no credentials
need to appear in the environment. If you must use static keys, pass them via `AWS_ACCESS_KEY_ID`
and `AWS_SECRET_ACCESS_KEY` in the compose env block.

Once the policy is attached, set `AWS_HOSTED_ZONE_ID` in your environment (uncomment the line in
`web/docker-compose.yml`) to scope the policy to a single hosted zone and skip the
zone-discovery API calls (`ListHostedZonesByName`, `ListHostedZones`).

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
- [x] Web interface (WASM or small HTTP server)
- [ ] GUI front-end
- [ ] Full punctuation support (ITU/ICAO special characters)

## License

This project is licensed under the [MIT License](LICENSE).
