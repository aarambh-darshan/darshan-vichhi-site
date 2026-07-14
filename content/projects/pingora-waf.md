---
id: pingora-waf
slug: pingora-waf
name: Pingora WAF
order: 7
featured: true

summary: A configurable Rust web application firewall and reverse proxy built on Pingora with request inspection, policy enforcement, and observability.

technologies:
  - Rust
  - Pingora
  - Web security
  - Prometheus metrics

links:
  github:
    label: View Pingora WAF on GitHub
    url: https://github.com/AarambhDevHub/pingora-waf
  documentation:
    label: Read the Pingora WAF documentation
    url: https://github.com/AarambhDevHub/pingora-waf#readme
---

Pingora WAF places security checks in front of an upstream service. Its rules
cover common injection patterns, path traversal, command injection, rate limits,
IP policies, bot handling, request bodies, and header validation.

The project also includes structured logging, Prometheus metrics, configuration
reloads, deployment examples, and benchmark documentation. It is an engineering
project for studying safe, observable request filtering rather than a claim to
replace a fully audited commercial security product.
