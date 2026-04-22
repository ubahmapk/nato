# Build context: repo root  →  docker build -f web/Dockerfile -t nato-web .

# ── Stage 1: Rust → WASM ─────────────────────────────────────────────────────
FROM rust:1-slim AS wasm-builder

# Install wasm-pack BEFORE copying project files so this layer is cached
# independently of source changes. Subsequent builds skip this step entirely.
RUN cargo install wasm-pack \
    && rustup target add wasm32-unknown-unknown

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

RUN cd crates/nato-wasm \
    && wasm-pack build --target web --out-dir /wasm-pkg

# ── Stage 2: Svelte → static files ───────────────────────────────────────────
FROM oven/bun:1-alpine AS web-builder

WORKDIR /app

# Copy manifest files first so bun install is cached when only source changes.
COPY web/package.json web/bun.lock ./

# Place the built WASM package where package.json expects it ("file:./pkg").
COPY --from=wasm-builder /wasm-pkg ./pkg/

RUN bun install --frozen-lockfile

# Copy remaining source (pkg/ is excluded by .dockerignore so it won't overwrite above).
COPY web/ ./

RUN bun run build

# ── Stage 3: Build custom Caddy with Route53 DNS-01 plugin ───────────────────
FROM caddy:builder AS caddy-builder

RUN xcaddy build \
    --with github.com/caddy-dns/route53

# ── Stage 4: Caddy serves static files + manages TLS ────────────────────────
FROM caddy:alpine AS runtime

COPY --from=caddy-builder /usr/bin/caddy /usr/bin/caddy
COPY --from=web-builder /app/build /usr/share/caddy
COPY web/Caddyfile /etc/caddy/Caddyfile

EXPOSE 80 443

# /data holds Let's Encrypt certificates. Mount as a named volume in production
# so certs survive container restarts. Losing this volume forces re-issuance
# and burns Let's Encrypt rate-limit quota (5 certs/domain/week).
VOLUME ["/data"]
