# AGENTS.md

## Project overview

Deltasafe is a Rust CLI tool for authenticated LAN file transfer. The current protocol uses AES-256-GCM with per-file HKDF session keys. Single crate, no external services or databases required.

### Development commands

| Task | Command |
|------|---------|
| Build | `cargo build` |
| Test | `cargo test --locked -- --test-threads=1` |
| Lint | `cargo clippy --locked --all-targets -- -D warnings` |
| Run server | `cargo run -- server --address 127.0.0.1:12345 --password "test"` |
| Run sync client | `cargo run -- sync --source ./path --target 127.0.0.1:12345 --password "test"` |

### Review baseline

- `cargo fmt --check`, locked all-target compilation, strict Clippy, and the serialized test suite are the review baseline.
- Tests use ephemeral loopback listeners and temporary receive directories; do not reintroduce fixed ports or repository-root test artifacts.
- The README and `docs/architecture.md` are the source of truth for the protocol and its security boundaries.

### Environment notes

- The crate uses Rust edition 2021. The committed `Cargo.lock` must be used for reproducible checks.
- No Docker, databases, or external services are needed.
- The project uses Tokio for async networking but can be tested entirely on localhost (loopback).
