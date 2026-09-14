# deltasafe validation — 2026-09-14 English CLI and regenerated negative evidence

This record documents the language change to the command surface and re-establishes the loopback
evidence against the translated binary. It is evidence for the listed environment and scenarios, not a
production security certification.

## Why this record exists

Before this change the command surface was Turkish while the README, the protocol errors, and the
receiver/listener logs were English. The 2026-09-11 captured receiver logs still contained the Turkish
startup line `Sunucu başlatılıyor: <addr>`, which no longer matches the binary. Those captures were
replaced rather than annotated so that the committed evidence matches the committed source.

The captures were first taken at revision `bd6614c`. Code changed afterwards (the `connect`/`watch`
removal, the discovery rewrite, and the audit fixes), so they were regenerated at the revision named
below. Evidence that describes a superseded revision is not evidence for the current one.

## Identity

- Host: macOS Apple Silicon (darwin arm64)
- Toolchain: `rustc 1.94.1 (e408947bf 2026-03-25)` / `cargo 1.94.1 (29ea6fb6a 2026-03-24)`
- Source revision: `410c306cf03ceb74167e363a41e3efd2581831f5` — the last commit that changed `src/`,
  `tests/`, or `Cargo.*`. Every later commit on this branch touches only documentation and
  changelogs, so the tested code is unchanged.
- Built binary SHA-256: `5e613612b64e70e09d5f21a7976bd7a1b6d52ac53c1266af03a7adadca522da6`
- Network: ephemeral loopback TCP listeners only
- Working tree at capture time: `dirty: 0 path(s)`

## Commands

~~~bash
cargo fmt --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked -- --test-threads=1

bash scripts/demo_loopback.sh                       # clean run, 2026-09-14-run1.txt

# Controlled startup failure: address that cannot be bound.
DELTASAFE_DEMO_ADDR=192.0.2.1:12345 \
  DELTASAFE_DEMO_EVIDENCE_DIR=/tmp/evidence-unbindable \
  bash scripts/demo_loopback.sh

# Controlled startup failure: port genuinely held by a separate python listener.
DELTASAFE_DEMO_ADDR=127.0.0.1:<held-port> \
  DELTASAFE_DEMO_EVIDENCE_DIR=/tmp/evidence-occupied \
  bash scripts/demo_loopback.sh
~~~

## Automated checks

All pass with the committed lockfile: 14 library tests, 5 CLI-surface tests, 5 integration tests, 5
helper tests, and 1 doctest (0 failed), 30 total.

## Clean run

`2026-09-14-run1.txt` exited 0 with `DEMO PASS` and `dirty: 0 path(s)`. It reproduced the same
contract as the 2026-09-11 runs:

1. Built with `cargo build --locked`.
2. Required all three readiness proofs before sending anything: the receiver is our own live child
   process, its log contains `Receiver listening on <addr>` for the exact address passed in, and that
   address accepts a TCP connection.
3. Sent a wrong password of valid length and required sender exit 1 with no files or symlinks anywhere
   under the receiver root.
4. Sent the correct password and verified byte/SHA-256 equality for the empty file, the small text
   file, and the nested 20000-byte multi-frame binary, then checked the receiver inventory.
5. Stopped only its own receiver process and removed only its own temporary root.

## Regenerated negative scenarios

Both controlled startup failures exited non-zero, preserved the raw receiver log, and now show
English output:

- Unbindable address `192.0.2.1:12345` (`2026-09-14-negative-unbindable.txt`,
  `2026-09-14-negative-unbindable-server.log`): the receiver logged
  `Starting server: 192.0.2.1:12345` followed by
  `[❌] Error: Could not bind receiver to 192.0.2.1:12345`, and the harness reported
  `FAIL: receiver exited before becoming ready` and exited 1.
- Genuinely occupied port (`2026-09-14-negative-occupied-port.txt`,
  `2026-09-14-negative-occupied-port-server.log`): a separate python listener held the port, the
  receiver logged `Starting server: 127.0.0.1:<port>` and
  `[❌] Error: Could not bind receiver to 127.0.0.1:<port>`, the harness exited 1, and the holder
  process was still alive afterwards (`kill -0`), proving the harness only terminates processes it
  started.

## Evidence hygiene

The raw harness transcripts were copied verbatim except for three substitutions applied to every
evidence file: this machine's absolute repository path became `<repo>`, any `/Users/<user>` prefix
became `/Users/<redacted>`, and the per-user temporary root under `/var/folders/…/T/` became `/tmp/`.
The temporary-root substitution was added after review pointed out that macOS puts a machine-specific
identifier in that path, which is the same class of local-environment trace the other two remove. No
commands, exit codes, hashes, or log text were otherwise changed. No credentials, keys, or tokens
appear in any evidence file.

## Boundaries

- Loopback only; both endpoints ran on one host. This is not a cross-device test.
- The harness picks a free port by bind-and-close before the receiver binds it; a local race is
  theoretically possible, in which case readiness fails and the harness exits non-zero rather than
  passing falsely.
- Passing tests and this demo are evidence for the listed scenarios only. They do not claim
  certificate-based device identity, TLS, forward secrecy, durable replay protection across receiver
  restarts, or elimination of TOCTOU races against a hostile local process.
