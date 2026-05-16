# Contributing

## Scope

`rudolint` is early-stage. Please discuss large feature work before opening a
pull request. Small parser fixes, rule fixtures, diagnostics improvements, and
documentation fixes are welcome.

## Setup

Install Rust through `rustup`. The repository pins its toolchain in
`rust-toolchain.toml`, including `clippy` and `rustfmt`.

```bash
rustup show
cargo --version
```

## Repository Layout

The Rust workspace lives under `crates/`. Each crate should have one clear job.
Read [crates/README.md](crates/README.md) and
[docs/architecture.md](docs/architecture.md) before adding new modules.

## Validation

Run the same checks as CI before sending a pull request:

```bash
cargo fmt --all -- --check
cargo check --all-targets --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --all-features --locked
```

During iteration, prefer targeted tests:

```bash
cargo test -p rudolint-dockerfile parser
cargo test -p rudolint --test cli
```

## Tests

- Add tests for behavior changes.
- Prefer integration tests for CLI and rule output.
- Prefer `insta` snapshots for structured JSON and SARIF diagnostics.
- Keep external oracle tests ignored unless the required binary is pinned and
  explicitly installed.

## Rules

Rules should be deterministic and should not perform network access. Avoid
filesystem access inside rule implementations unless the CLI explicitly enables
repository-wide analysis for that mode.

Compatibility-oriented rules use `RDL` or `RSC` families. BuildKit-native rules
use `RDK`.

## Dependencies

Keep dependency additions small and justified. Do not update the full lockfile
unless the change requires it. Prefer scoped updates:

```bash
cargo update --package <name> --precise <version>
```

## Releases

Do not cut releases from local machines yet. Release automation should produce
checksummed artifacts and should run from GitHub Actions once the release
workflow exists.
