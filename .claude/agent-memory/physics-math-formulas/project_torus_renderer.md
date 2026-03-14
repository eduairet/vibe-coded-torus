---
name: ASCII Torus Renderer — Project Context
description: Core facts about the vibe-coded-torus project: geometry, rendering pipeline, and parameter choices
type: project
---

Rust ASCII torus renderer with lib/binary split (`src/lib.rs` + `src/main.rs`). Full math spec in `docs/math_specification.md`.

Parameters: R1=1 (minor/tube radius), R2=2 (major/ring radius), K2=5 (viewer distance), screen 80x24, ~30fps (33ms sleep).

The torus symmetry axis is the Y-axis. Rotation composition is Rz(B)·Rx(A), with dA=0.07, dB=0.03 per frame.

K1 = W*K2/(8*(R1+R2)) — intentionally smaller than the spec's (W/2)*K2/(R1+R2) to provide a 25% safety margin against edge clipping.

Luminance = ny2 - nz2 (using unnormalised L=(0,1,-1)), expanded algebraically in the implementation — verified correct.

**How to apply:** Spec and implementation are now aligned on all parameters. When verifying changes, check that the Trig struct fields match the expected sin/cos pairs.
