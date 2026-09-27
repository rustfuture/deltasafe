# deltasafe

A Rust command-line tool for developers and operators who need authenticated, tamper-evident file transfer across a trusted local network.

[![CI](https://github.com/rustfuture/deltasafe/actions/workflows/ci.yml/badge.svg)](https://github.com/rustfuture/deltasafe/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Status**: Experimental CLI prototype (v0.1.2). Tested on Linux and macOS; not audited for production use. Scope and security boundaries are explicit; see [CHANGELOG.md](CHANGELOG.md).

- **Encrypted streaming**: Streams regular files and nested directories over TCP using AES-256-GCM authenticated frames with per-file HKDF-derived session keys.
- **Shared-secret authentication**: Supports shared passwords (PBKDF2 with per-transfer salt) or direct 32-byte hex keys.
- **Staged atomic publication**: Buffers transfers in temporary files and verifies exact byte counts and BLAKE3 digests before atomic publication (no-overwrite).
- **Filesystem boundary validation**: Rejects path traversal (`..`), absolute paths, and symlink parents to ensure all files stay within the receive root.
- **Heuristic peer discovery**: Offers an optional, bounded TCP port-scan sweep (`--timeout`) across local addresses to find listening peers.

## Quick start

### Build

Requires Rust 1.85+ (see [Cargo.toml](Cargo.toml)):

~~~bash
cargo build --locked --release
~~~

### Start receiver

Run the receiver on a local port with a password (minimum 8 characters):

~~~bash
cargo run --locked -- server --address 127.0.0.1:12345 --password "MySecret123"
~~~

### Send files

In another terminal, create a directory and synchronize it to the receiver:

~~~bash
mkdir -p ./my_folder && echo "hello" > ./my_folder/hello.txt
cargo run --locked -- sync \
  --source ./my_folder \
  --target 127.0.0.1:12345 \
  --password "MySecret123"
~~~

The receiver verifies and publishes received files under `received_files/`.

### Direct key mode

To authenticate using a 32-byte hex key instead of a password:

~~~bash
# Receiver
cargo run --locked -- server \
  --address 127.0.0.1:12345 \
  --key 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef

# Sender
cargo run --locked -- sync \
  --source ./my_folder \
  --target 127.0.0.1:12345 \
  --key 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
~~~

The command surface consists of `sync`, `discover`, and `server`. Legacy stubs (`connect`, `watch`) have been removed; see [CHANGELOG.md](CHANGELOG.md).

## Protocol outline

For each file, the sender transmits a bounded JSON header containing the protocol version, random session ID, relative path, declared size, BLAKE3 digest, and optional PBKDF2 salt. The receiver validates the header and responds with an authenticated `READY` frame. Data frames carry a sequential index and AES-GCM ciphertext. An authenticated empty `FINISH` frame terminates the transfer. The receiver validates the byte count and BLAKE3 digest, atomically publishes the temporary file without overwriting existing files, and replies with an authenticated `COMPLETE` or `ERROR` frame.

See [docs/architecture.md](docs/architecture.md) for the state machine and crypto details.

## Scope and limitations

<a id="security-boundaries"></a>

`deltasafe` is designed for trusted local networks and controlled filesystems. It explicitly defines the following boundaries:

- **Trusted LAN only**: Authenticates possession of the shared password or key, not device or certificate identities. Does not provide TLS or PKI; do not expose the listener to the public internet.
- **No forward secrecy**: Compromise of the pre-shared secret allows decrypting previously captured traffic that used that secret.
- **No persistent replay cache**: Replay protection is enforced within an active session, but does not persist across receiver restarts.
- **Local filesystem TOCTOU**: Path validation prevents path traversal and symlink escapes, but standard library checks cannot eliminate OS-specific TOCTOU races against concurrent hostile local processes modifying the receive directory.
- **Resource limits**: Framing and header sizes are bounded in memory, but disk storage quotas are not enforced.
- **Discovery limitations**: LAN discovery is a best-effort TCP port scan over the local `/24` and ports 12340–12349, bounded by `--timeout`. An open port does not guarantee peer authenticity; `--target` is the deterministic path.
- **Platform support**: Verified on Linux (via CI) and macOS (local); Windows is not supported.

## Verification

Run the test suite and checks locally:

~~~bash
cargo fmt --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked -- --test-threads=1
~~~

Committed verification artifacts in this repository:
- **Unit and parser tests**: CLI surface parsing ([tests/cli_surface.rs](tests/cli_surface.rs)) and crypto/hash validation ([tests/unit_tests.rs](tests/unit_tests.rs)).
- **Integration tests**: Ephemeral loopback transfers, empty files, multi-chunk transfers, wrong passwords, corrupt frames, and path safety ([tests/integration_tests.rs](tests/integration_tests.rs)).
- **Loopback demo harness**: End-to-end verification script ([scripts/demo_loopback.sh](scripts/demo_loopback.sh)).
- **Validation records**: Historical and recent run logs and negative test evidence ([docs/validation/2026-09-14-english-cli.md](docs/validation/2026-09-14-english-cli.md) and [docs/validation/2026-09-11-demo-hardening.md](docs/validation/2026-09-11-demo-hardening.md)).

## Project layout

- `src/sync.rs` — source traversal, header construction, encrypted sender, final-status handling.
- `src/server.rs` — bounded receiver, safe destination preparation, temporary-file publication.
- `src/protocol.rs` — framing, HKDF session keys, AES-GCM and authenticated control messages.
- `src/crypto.rs` — password/key parsing and PBKDF2 compatibility layer.
- `src/discovery.rs` — best-effort TCP port-scan discovery. mDNS is not implemented.
- `src/utils.rs` — CLI key and target resolution.

## Versioning and support

deltasafe follows `0.x` semantics: the version number represents scope rather than a long-term stability guarantee. Breaking changes bump the minor version; compatible fixes bump the patch version. Changes are recorded in [CHANGELOG.md](CHANGELOG.md).

| Platform | Status |
| --- | --- |
| Linux | Verified in CI on Rust 1.85 and stable. |
| macOS | Verified locally on Apple Silicon against committed source. |
| Windows | Not supported or verified. |

## Security

Report suspected vulnerabilities privately as described in [SECURITY.md](SECURITY.md). Guarantees and non-goals are detailed in [Scope and limitations](#security-boundaries).

## License

MIT. See [LICENSE](LICENSE).
