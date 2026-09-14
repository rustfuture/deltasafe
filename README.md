# deltasafe

[![CI](https://github.com/rustfuture/deltasafe/actions/workflows/ci.yml/badge.svg)](https://github.com/rustfuture/deltasafe/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

`deltasafe` is a Rust command-line tool for authenticated file transfer over a trusted LAN. A sender streams regular files from a directory to a receiver; the receiver validates every encrypted frame, verifies the complete BLAKE3 digest, and publishes each file only after successful verification.

This is a pre-1.0 portfolio project. The supported scope and security boundaries are intentionally explicit. See [CHANGELOG.md](CHANGELOG.md) for the change history.

## What is implemented

- AES-256-GCM authenticated encryption for every data and control frame.
- Per-file session identifiers and HKDF-derived session keys.
- Deterministic, direction-separated nonces derived from frame indexes.
- Password mode with a per-transfer PBKDF2 salt, or direct 32-byte hex keys.
- Bounded JSON headers and frame sizes.
- Exact byte-count checks, BLAKE3 verification, and authenticated final status.
- Temporary-file receive path; incomplete or corrupt transfers are removed and never published.
- Relative-path validation, symlink-parent rejection, and no-overwrite publication.
- Multi-file, nested-directory, empty-file, wrong-password, corruption, truncation, and timeout-oriented tests.
- Best-effort LAN discovery by TCP port scan over the first 10 hosts of the local `/24` and ports 12340–12349, bounded by `--timeout`. mDNS is not implemented, and an open port is not a verified peer identity; `--target` is the deterministic path.

## Security boundaries

The protocol authenticates possession of the shared password or key and protects file contents against tampering in transit. It does not provide a certificate-based device identity, TLS, forward secrecy, durable replay prevention across receiver restarts, disk-quota enforcement, or protection against a local administrator who can alter the receive directory during a transfer. Use it on a network and filesystem you control; do not expose the listener directly to the public internet.

The receiver rejects paths that are absolute, contain parent/root/prefix components, escape the canonical receive root, or overwrite an existing destination. The standard-library path checks cannot eliminate every operating-system-specific TOCTOU race against a hostile local process during a transfer; that limitation is documented rather than hidden.

## Requirements and build

- Rust 1.85 or newer. The crate uses edition 2021; the committed `Cargo.lock` is the reproducibility source for dependency versions.
- A local network address reachable by both peers.

~~~bash
git clone https://github.com/rustfuture/deltasafe.git
cd deltasafe
cargo build --locked --release
~~~

## Usage

Start a receiver with a password:

~~~bash
cargo run --locked -- server --address 127.0.0.1:12345 --password "MySecret123"
~~~

Send a directory to that receiver from another terminal:

~~~bash
cargo run --locked -- sync \
  --source ./my_folder \
  --target 127.0.0.1:12345 \
  --password "MySecret123"
~~~

For direct key mode, pass the same 64-character hexadecimal key to both commands:

~~~bash
cargo run --locked -- server \
  --address 127.0.0.1:12345 \
  --key 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
~~~

The receiver writes verified files under `received_files/` and refuses to replace an existing destination. Use `--auto`/`--auto-select` only when best-effort LAN discovery is acceptable; `--target` is the deterministic option.

The command surface is `sync`, `discover`, and `server`. `connect` and `watch` were advertised in earlier revisions but were never implemented; they were removed rather than left as stubs, and they now fail as unknown subcommands. See [CHANGELOG.md](CHANGELOG.md).

## Protocol outline

For each file, the sender sends a bounded JSON header containing the protocol version, a random session ID, relative path, declared size, BLAKE3 digest, and optional password salt. The receiver validates it and returns an authenticated `READY` frame. Data frames carry a sequential index and AES-GCM ciphertext. An authenticated empty `FINISH` frame covers empty files and terminates the file. The receiver checks the exact size and digest, atomically publishes the temporary file without overwriting an existing path, and returns an authenticated `COMPLETE` or `ERROR` frame.

See [docs/architecture.md](docs/architecture.md) for the state machine and implementation boundaries.

## Verification

Run the same local checks used by the portfolio review:

~~~bash
cargo fmt --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked -- --test-threads=1
~~~

The integration suite uses ephemeral loopback ports and temporary directories. It covers multiple nested files, empty and multi-chunk files, password salt exchange, wrong passwords, existing destinations, corrupted ciphertext, truncated frames, path traversal, and symlink-parent rejection. Passing tests are evidence for these scenarios only; they are not a general production security audit.

A localhost demo harness is kept at [scripts/demo_loopback.sh](scripts/demo_loopback.sh). It proves receiver ownership before transfer, verifies byte/SHA-256 equality for every fixture, checks that a valid-length wrong password publishes no file anywhere under the receiver root, and preserves raw failure logs. The current harnessed runs and controlled startup-failure results are recorded in [docs/validation/2026-09-14-english-cli.md](docs/validation/2026-09-14-english-cli.md); the earlier hardening record is kept at [docs/validation/2026-09-11-demo-hardening.md](docs/validation/2026-09-11-demo-hardening.md).

## Project layout

- `src/sync.rs` — source traversal, header construction, encrypted sender, final-status handling.
- `src/server.rs` — bounded receiver, safe destination preparation, temporary-file publication.
- `src/protocol.rs` — framing, HKDF session keys, AES-GCM and authenticated control messages.
- `src/crypto.rs` — password/key parsing and PBKDF2 compatibility layer.
- `src/discovery.rs` — best-effort TCP port-scan discovery. mDNS is not implemented.
- `src/utils.rs` — CLI key and target resolution.

## Versioning and support

deltasafe follows `0.x` semantics: the version number is a statement about scope, not a
compatibility promise. While the major version is 0, a breaking change to the CLI, the wire
protocol, or the receive-directory layout bumps the minor version, and a compatible fix bumps the
patch version. Every change is recorded in [CHANGELOG.md](CHANGELOG.md).

| Platform | Status |
| --- | --- |
| Linux | Verified by CI on Rust 1.85 (the minimum supported version) and stable. |
| macOS | Verified locally against the committed source; not part of the CI matrix. |
| Windows | Not supported or verified. |

The minimum supported Rust version is 1.85; raising it is a minor-version change. A `1.0` would
mean the existing command surface, protocol version, and documented boundaries have stopped moving,
not that every idea in the issue tracker has been implemented.

## Security

Report suspected vulnerabilities privately as described in [SECURITY.md](SECURITY.md). The
protocol's guarantees and its explicit non-goals are listed under
[Security boundaries](#security-boundaries).

## License

MIT. See [LICENSE](LICENSE).
