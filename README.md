# vibe-coded-torus

A rotating 3D torus rendered as ASCII art in your terminal, written in Rust.

Inspired by Andy Sloane's [donut.c](https://www.a1k0n.net/2011/07/20/donut-math.html) and [Joma Tech's video](https://www.youtube.com/watch?v=DEqXNfs_HhY) explaining the math behind it.

```
                                    @@@@@@@
                                $$$#####****###
                              ##***!!!!**=*!!!=!*
                             **!*!!!==;;:::;;;====
                            !!!!==;::~-,,,,,,~~:;;
                            !==;;:~-,        .,-~:
                            ;;;::-,.      ....,,-~
                            :::~~-,..~=!##!,,,----
                            ~~~~~-,,-=*#$$*=;:~~-
                             ,----~~:=!!!!!!;:~
                               .,,-~:;==!==~,
```

## Requirements

- [Docker](https://docs.docker.com/get-docker/)

## Quick Start

```bash
git clone https://github.com/your-user/vibe-coded-torus.git
cd vibe-coded-torus
docker compose up --build
```

Press `Ctrl+C` to stop.

## Running Tests

```bash
docker run --rm -v "$(pwd):/app" -w /app rust:1.77-slim cargo test
```

## How It Works

The renderer samples ~28,000 points on a torus surface each frame using parametric equations, applies 3D rotation matrices (X and Z axes), projects to 2D with perspective division, and shades using the dot product of the surface normal with a light direction vector. Luminance is mapped to 12 ASCII characters from dimmest to brightest: `.,-~:;=!*#$@`

The full mathematical derivation is in [`docs/math_specification.md`](docs/math_specification.md).
