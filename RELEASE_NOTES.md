# Release notes

## Unreleased — authenticated transfer prototype

- Added bounded, authenticated AES-256-GCM transfer frames.
- Derived per-file session keys with HKDF and authenticated control statuses.
- Added path, size, frame-index, truncation, corruption, wrong-password, and symlink-parent checks.
- Publish received files only after integrity verification; existing destinations are not overwritten.
- Added loopback integration coverage for nested, empty, and multi-chunk files.

## Scope and limitations

This is a pre-1.0 prototype. It does not claim certificate identity, TLS, forward secrecy, durable replay prevention across receiver restarts, disk-quota enforcement, or complete hostile-local-filesystem TOCTOU resistance. Passing tests are scenario evidence, not a general production security audit.
