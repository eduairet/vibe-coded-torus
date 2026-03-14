# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ASCII torus renderer — a Rust CLI application that displays a rotating 3D torus in the terminal using perspective projection, z-buffering, and Lambertian ASCII shading. Runs inside Docker.

## Build & Run Commands

```bash
# Build and run via Docker
docker compose up --build

# Run tests
docker run --rm -v "$(pwd):/app" -w /app rust:1.77-slim cargo test
```

The Docker setup uses a multi-stage build (`rust:1.77-slim` → `debian:bookworm-slim`). The compose file sets `stdin_open` and `tty` for interactive terminal output.

## Architecture

Library crate (`src/lib.rs`) with rendering logic, thin binary entrypoint (`src/main.rs`). The render loop:

1. Samples the torus surface parametrically (theta × phi ≈ 28k points/frame)
2. Applies rotation matrices Rz(B)·Rx(A) to each point
3. Perspective-projects to screen coordinates with aspect-ratio correction
4. Computes Lambertian luminance (dot product of rotated surface normal with light direction)
5. Z-buffers and maps luminance to 12-level ASCII palette `.,-~:;=!*#$@`
6. Writes full frame to stdout using ANSI cursor reset (`\x1B[H`)

Key constants (in `lib.rs`): `R1=1` (minor radius), `R2=2` (major radius), `K2=5` (viewer distance), screen 80×24, rotation increments A+=0.07 B+=0.03 per frame, ~30fps.

The full mathematical derivation is in `docs/math_specification.md`.

## Dependencies

- `ctrlc` crate — graceful Ctrl+C shutdown (the only external dependency)
