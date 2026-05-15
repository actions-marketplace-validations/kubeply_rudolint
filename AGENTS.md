# AGENTS.md

## Purpose

`rudolint` is a BuildKit-native Dockerfile linter written in Rust. Keep the
runtime as a single binary: no shell, Python, Node, Docker, or external linter
dependency in the normal `rudolint check` path.

## Working Rules

- Keep parsing, diagnostics, and rule evaluation separated.
- Treat compatibility tests as oracle tests, not as runtime behavior.
- Prefer source-span preserving parsers and deterministic diagnostics.
- Do not copy third-party rule descriptions wholesale into source or docs.
- Add new rules with focused fixtures and JSON/SARIF output coverage.

## Validation

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```
