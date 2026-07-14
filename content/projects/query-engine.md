---
id: query-engine
slug: query-engine
name: Query Engine
order: 6
featured: true

summary: A modular SQL query engine in Rust with parsing, logical planning, optimization, vectorized execution, multiple data sources, and protocol experiments.

technologies:
  - Rust
  - SQL
  - Apache Arrow
  - Query planning and execution

links:
  github:
    label: View Query Engine on GitHub
    url: https://github.com/AarambhDevHub/query-engine
  documentation:
    label: Read the Query Engine documentation
    url: https://github.com/AarambhDevHub/query-engine#readme
---

Query Engine follows the path from SQL text to an executable result: parsing,
logical planning, optimization, physical execution, and vectorized processing
with Apache Arrow data batches.

The workspace also explores CSV, Parquet, and in-memory sources, indexes,
caching, joins, aggregates, window functions, a command-line REPL, PostgreSQL
wire-protocol compatibility, and coordinator/worker execution boundaries.
