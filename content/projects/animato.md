---
id: animato
slug: animato
name: Animato
order: 3
featured: true

summary: A renderer-agnostic Rust animation toolkit for computing reusable motion values across applications, engines, browsers, and UI frameworks.

technologies:
  - Rust
  - Tweens and timelines
  - Springs and motion paths
  - WASM and framework integrations

links:
  github:
    label: View Animato on GitHub
    url: https://github.com/AarambhDevHub/animato
  documentation:
    label: Read the Animato API documentation
    url: https://docs.rs/animato
---

Animato separates animation state from rendering. An application advances a
tween, spring, timeline, path, gesture, or driver, reads the computed value, and
applies it to its own renderer.

The crate family covers portable interpolation, easing, timelines, physics,
colour spaces, motion paths, browser helpers, GPU batching, development tools,
and integrations for Rust UI and game ecosystems. The design keeps the core
usable without forcing a particular framework or rendering backend.
