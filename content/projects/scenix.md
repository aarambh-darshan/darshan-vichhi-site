---
id: scenix
slug: scenix
name: Scenix
order: 4
featured: true

summary: Modular Rust-native 3D scene infrastructure with separate paths for CPU authoring, asset loading, rendering, animation, and browser targets.

technologies:
  - Rust
  - Scene graphs
  - 3D rendering
  - WebGPU and WASM

links:
  github:
    label: View Scenix on GitHub
    url: https://github.com/AarambhDevHub/scenix
  documentation:
    label: Read the Scenix API documentation
    url: https://docs.rs/scenix
---

Scenix organizes scene graphs, cameras, geometry, materials, lights, textures,
ray casting, helpers, asset loading, rendering, post-processing, animation, and
browser support into focused crates.

CPU-side scene authoring remains lightweight by default. Applications opt into
GPU rendering, asset formats, post-processing, Animato integration, or WASM only
when those systems are required.
