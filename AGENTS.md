# AGENTS.md

## Cursor Cloud specific instructions

### Project overview

Deltasafe is a Rust CLI tool for secure LAN file synchronization using AES-256-CBC encryption. Single crate, no external services or databases required.

### Development commands

| Task | Command |
|------|---------|
| Build | `cargo build` |
| Test (unit) | `cargo test` |
| Lint | `cargo clippy` |
| Run server | `cargo run -- server --address 127.0.0.1:12345 --password "test"` |
| Run sync client | `cargo run -- sync --source ./path --target 127.0.0.1:12345 --password "test"` |

### Known issues

- The `test_basic_sync` integration test has a pre-existing race condition and may fail intermittently. Unit tests are reliable.
- `cargo clippy -- -D warnings` will fail due to 4 pre-existing lint warnings in `src/sync.rs`, `src/server.rs`, and `src/discovery.rs`. Use `cargo clippy` (without `-D warnings`) for a clean exit code.
- The server has a pre-existing `UnpadError` panic when receiving file content (decryption padding mismatch between client/server).

### Environment notes

- Rust edition 2024 requires Rust 1.85+. The VM update script runs `rustup update stable && rustup default stable` to ensure the correct version.
- No Docker, databases, or external services are needed.
- The project uses Tokio for async networking but can be tested entirely on localhost (loopback).
