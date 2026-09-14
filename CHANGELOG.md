# Changelog

All notable changes to this project are documented here.

## [Unreleased]

## [0.1.1] - 2026-09-14

### Fixed

- `discover --timeout 0` was accepted and then silently raised to one second, so the run ignored the
  value the caller passed. The parser now rejects zero and the scan uses the budget it is given.
- `tests/unit_tests.rs` wrote fixed filenames into the repository root and its key-validation test
  asserted string lengths and called `hex::decode` directly instead of the application's
  `parse_hex_key`. The tests now use a temporary directory and exercise the real validator, including
  the empty, short, non-hex, and round-trip cases.

### Changed

- The validation evidence under `docs/validation/` was regenerated at the current source revision.
  The earlier captures named revision `bd6614c`, which stopped describing the code once the command
  surface and discovery changed. The evidence now records `410c306` and the binary it was produced
  from, and the sanitization also redacts the per-user temporary root.
- `tests/cli_surface.rs` documents its scope as the parser surface. It previously implied that every
  accepted subcommand was proven to do real work, which parser tests cannot show; integration coverage
  and the loopback harness are where that lives.

## [0.1.0] - 2026-09-14

### Added

- Bounded, authenticated AES-256-GCM transfer frames with a per-file HKDF session key.
- Password mode with a per-transfer PBKDF2 salt, plus direct 32-byte hex key mode.
- Path, size, frame-index, truncation, corruption, wrong-password, and symlink-parent checks.
- Verified publication: a received file is published only after the exact byte count and BLAKE3
  digest are confirmed, and an existing destination is never overwritten.
- Loopback integration coverage for nested, empty, and multi-chunk files.
- A reproducible loopback demo harness at [scripts/demo_loopback.sh](scripts/demo_loopback.sh) with
  recorded evidence under [docs/validation/](docs/validation/).
- CLI surface regression tests in [tests/cli_surface.rs](tests/cli_surface.rs).
- [SECURITY.md](SECURITY.md) with the private reporting channel, in-scope and out-of-scope
  categories, and an explicit statement that the project has not been audited.

### Changed

- The CLI help text, runtime messages, and error output are now English throughout. Previously the
  command surface was Turkish while the README, protocol errors, and logs were English.
- `discover --timeout` is now honoured. The port scan previously ignored the value and always ran
  with a fixed per-connection timeout; the sweep is now bounded by the budget the caller passes. The
  per-connection connect timeout is a named constant, raised from 100 ms to 200 ms so that a slow but
  reachable host is not missed while the overall budget still bounds the sweep.
- The port scan is documented as a heuristic: an open port is not a verified peer identity.

### Removed

- The `connect` and `watch` subcommands. They were listed in `--help` but only printed a "still
  under development" notice and exited 0, so a script could not tell that nothing had happened.
  They now fail as unknown subcommands.
- The mDNS discovery stub. It slept for 100 ms and always returned no servers while reporting
  `Found 0 server(s) via mDNS`, which implied support that did not exist. The `mdns-sd` dependency
  and the unused `DiscoveryMethod` enum went with it, since only one discovery path remains.

## Scope and limitations

This is a pre-1.0 prototype. It does not claim certificate identity, TLS, forward secrecy, durable
replay prevention across receiver restarts, disk-quota enforcement, or complete
hostile-local-filesystem TOCTOU resistance. Passing tests are scenario evidence, not a general
production security audit.
