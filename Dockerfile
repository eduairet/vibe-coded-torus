# Build stage
FROM rust:1.77-slim AS builder

WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

COPY --from=builder /app/target/release/vibe-coded-torus /usr/local/bin/vibe-coded-torus

ENTRYPOINT ["vibe-coded-torus"]
