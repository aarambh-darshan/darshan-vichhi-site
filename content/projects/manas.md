---
id: manas
slug: manas
name: Manas
order: 2
featured: true

summary: An experimental local-first neural-memory system that learns incrementally and stores retrieved knowledge in changed neural weights.

technologies:
  - Rust
  - Local-first AI
  - Continual learning
  - Neural memory

links:
  github:
    label: View Manas on GitHub
    url: https://github.com/AarambhDevHub/manas
  documentation:
    label: Read the Manas documentation
    url: https://github.com/AarambhDevHub/manas#readme
---

Manas explores whether a small neural system can learn facts one at a time,
preserve older knowledge, grow capacity when needed, and answer locally from
learned weights after the original source text has been removed.

The project is deliberately presented as research, not as a production memory
platform or a replacement for a large language model. Its main value is the
inspectable experiment: learning, protection against forgetting, persistence,
retrieval, growth, freshness, and local CPU execution are tested as explicit
parts of the design.
