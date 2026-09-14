# Changelog

All notable changes to this project are documented here.

## [Unreleased]

### Added

- Bounded, authenticated AES-256-GCM transfer frames with a per-file HKDF session key.
- Password mode with a per-transfer PBKDF2 salt, plus direct 32-byte hex key mode.
- Path, size, frame-index, truncation, corruption, wrong-password, and symlink-parent checks.
- Verified publication: a received file is published only after the exact byte count and BLAKE3
  digest are confirmed, and an existing destination is never overwritten.
- Loopback integration coverage for nested, empty, and multi-chunk files.
- A reproducible loopback demo harness at [scripts/demo_loopback.sh](scripts/demo_loopback.sh) with
  recorded evidence under [docs/validation/](docs/validation/).

### Changed

- The CLI help text, runtime messages, and error output are now English throughout. Previously the
  command surface was Turkish while the README, protocol errors, and logs were English.

## Scope and limitations

This is a pre-1.0 prototype. It does not claim certificate identity, TLS, forward secrecy, durable
replay prevention across receiver restarts, disk-quota enforcement, or complete
hostile-local-filesystem TOCTOU resistance. Passing tests are scenario evidence, not a general
production security audit.
