# deltasafe

`deltasafe` is a Rust command-line tool for authenticated file transfer over a trusted LAN. A sender streams regular files from a directory to a receiver; the receiver validates every encrypted frame, verifies the complete BLAKE3 digest, and publishes each file only after successful verification.

This is a pre-1.0 portfolio project. The supported scope and security boundaries are intentionally explicit.

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
- Optional LAN discovery. mDNS is currently a placeholder; port scanning is best-effort and manual `--target` is the reproducible path.

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

`connect` and `watch` are reserved CLI surfaces and currently report that they are not implemented. They are not presented as working features.

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

## Project layout

- `src/sync.rs` — source traversal, header construction, encrypted sender, final-status handling.
- `src/server.rs` — bounded receiver, safe destination preparation, temporary-file publication.
- `src/protocol.rs` — framing, HKDF session keys, AES-GCM and authenticated control messages.
- `src/crypto.rs` — password/key parsing and PBKDF2 compatibility layer.
- `src/discovery.rs` — best-effort LAN discovery.
- `src/utils.rs` — CLI key and target resolution.

## License

MIT. See [LICENSE](LICENSE).
