---
id: typebridge
slug: typebridge
name: TypeBridge
order: 5
featured: true

summary: A Rust-first type synchronization toolkit that generates matching definitions and schemas for other languages from Rust source types.

technologies:
  - Rust
  - Procedural macros
  - Code generation
  - Schema tooling

links:
  github:
    label: View TypeBridge on GitHub
    url: https://github.com/aarambh-darshan/typewriter
  documentation:
    label: Read the TypeBridge API documentation
    url: https://docs.rs/typebridge
---

TypeBridge treats Rust structs and enums as the source of truth for generated
TypeScript, Python, Go, Swift, Kotlin, GraphQL, JSON Schema, and other output
formats.

The project combines derive macros, generators, and a command-line interface so
teams can update a Rust model once and regenerate compatible contracts instead
of maintaining several copies by hand. The repository is named `typewriter`;
the released tool and primary binary are named `typebridge`.
