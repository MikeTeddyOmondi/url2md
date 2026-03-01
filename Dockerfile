# ── Build stage ──────────────────────────────────────────────────────────────
FROM rust:alpine AS builder

# cmake + C toolchain required by aws-lc-sys (rustls crypto backend)
RUN apk add --no-cache cmake make musl-dev gcc g++

WORKDIR /app

# Cache dependency compilation separately from source changes
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main(){}' > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src
# Touch main.rs so cargo knows it changed vs the dummy above
RUN touch src/main.rs && cargo build --release

# ── Runtime stage ─────────────────────────────────────────────────────────────
FROM alpine:3

# ca-certificates is needed for HTTPS requests
RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/url2md /usr/local/bin/url2md

ENTRYPOINT ["url2md"]
