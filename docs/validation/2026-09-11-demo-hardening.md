# deltasafe validation — 2026-09-11 harness hardening

This record captures the hardened loopback demo harness and its raw evidence. It is evidence for the
listed environment and scenarios, not a production security certification.

## Identity

- Host: macOS Apple Silicon (darwin arm64)
- Toolchain: `rustc 1.94.1 (e408947bf 2026-03-25)` / `cargo 1.94.1 (29ea6fb6a 2026-03-24)`
- Rust source SHA: `90d131b` (unchanged; `git diff --stat 90d131b 0d73ed1 -- src tests Cargo.toml Cargo.lock` is empty)
- Harness revision: `scripts/demo_loopback.sh` at commit `0d73ed1`, file SHA-256
  `054e2fb5f5d590a4ce89f6c772f7b51e73d9c7299ab04593c186bd927900c58a`
- Built binary SHA-256: `63a8b9c45dbb1c60feb93e86fb7b3a0dcaff9f42f998c844aaa512dd1ce23c46`
  (identical to the 2026-09-10 record, confirming the Rust source did not change)
- Network: ephemeral loopback TCP listeners only

## Commands

~~~bash
cargo fmt --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked -- --test-threads=1

bash scripts/demo_loopback.sh                       # clean run 1
bash scripts/demo_loopback.sh                       # clean run 2

# Controlled startup failures (both expected to exit non-zero)
DELTASAFE_DEMO_ADDR=192.0.2.1:12345 \
  DELTASAFE_DEMO_EVIDENCE_DIR=/tmp/evidence-unbindable \
  bash scripts/demo_loopback.sh
# Occupied port: hold 127.0.0.1:<port> with a separate python listener first
DELTASAFE_DEMO_ADDR=127.0.0.1:<held-port> \
  DELTASAFE_DEMO_EVIDENCE_DIR=/tmp/evidence-occupied \
  bash scripts/demo_loopback.sh
~~~

## Automated checks

All pass with the committed lockfile: 14 library tests, 5 integration tests, 5 helper tests, and 1
doctest (0 failed). The integration suite remains the primary coverage for partial/corrupt transfers,
path traversal, and destination collisions.

## Clean runs

Two independent clean runs (`2026-09-11-run1.txt`, `2026-09-11-run2.txt`) both exited 0 with
`DEMO PASS`, reported `dirty: 0 path(s)`, and used the same binary SHA-256. Each run:

1. Built with `cargo build --locked`.
2. Created an owned `mktemp -d` root with separate source and receiver directories.
3. Required all three readiness proofs before sending anything:
   - the receiver is our own live child process (`kill -0`),
   - the receiver log contains `Receiver listening on <addr>` for the exact address passed in,
   - that address accepts a TCP connection.
4. Sent a wrong password with a valid length and required sender exit 1 with no files or symlinks
   anywhere under the receiver root.
5. Sent the correct password and verified (`cmp` plus SHA-256) the empty file, the small text file, and
   the nested 20000-byte multi-frame binary, then checked that the receiver tree contains exactly those
   three fixtures.
6. Stopped only its own receiver process and removed only its own temporary root.

## Wrong-password cleanup contract

The harness walks the whole receiver root (`find ... -type f -o -type l`) instead of checking a single
path. Both runs show zero files and symlinks after the wrong password, with one directory remaining:

~~~text
wrong password rejected (sender exit 1); receiver tree has no files or symlinks
note: receiver tree has directories (allowed by the cleanup contract):
.../receiver/received_files
~~~

This matches the implementation contract:

- `receive_file` writes into a `NamedTempFile` created in the destination parent and drops it on any
  error, so no temporary or partial destination file may remain.
- `prepare_destination` creates destination directories before frame decryption, so parent directories
  may legitimately remain after an authentication failure. The harness rejects files and symlinks but
  tolerates directories, and reports which directories remain.

## Negative scenarios

Both controlled startup failures exited non-zero and preserved raw logs:

- Unbindable address `192.0.2.1:12345` (`2026-09-11-negative-unbindable.txt`): receiver logged
  `Could not bind receiver to 192.0.2.1:12345`, the harness reported
  `FAIL: receiver exited before becoming ready`, and exited 1.
- Genuinely occupied port (`2026-09-11-negative-occupied-port.txt`): a separate python listener held
  the port; the receiver logged `Could not bind receiver to 127.0.0.1:<port>`, the harness exited 1,
  and the holder process was still alive afterwards (checked with `kill -0`), proving the harness only
  terminates processes it started.

Raw failure logs are printed before cleanup and copied to `DELTASAFE_DEMO_EVIDENCE_DIR` when set; the
preserved receiver logs are included as `2026-09-11-negative-*-server.log`.

## Evidence hygiene

The raw harness transcripts were copied here verbatim except for one substitution applied to every
evidence file: this machine's absolute repository path was replaced with `<repo>` (and any remaining
`/Users/<user>` prefix with `/Users/<redacted>`). No commands, exit codes, hashes, or log text were
otherwise changed. No credentials, keys, or tokens appear in any evidence file.

## Boundaries

- Loopback only; both endpoints ran on one host. This is not a cross-device test.
- The harness picks a free port by bind-and-close before the receiver binds it; a local race is
  theoretically possible, in which case readiness fails and the harness exits non-zero rather than
  passing falsely.
- Passing tests and this demo are evidence for the listed scenarios only. They do not claim
  certificate-based device identity, TLS, forward secrecy, durable replay protection across receiver
  restarts, or elimination of TOCTOU races against a hostile local process.
