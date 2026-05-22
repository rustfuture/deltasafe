---
name: deltasafe
description: Deltasafe LAN file sync tool conventions. Use when editing src/, tests/, or working on TCP transfer protocol, AES encryption, PBKDF2 passwords, server/client sync, or discovery.
paths:
  - "src/**/*.rs"
  - "tests/**/*.rs"
  - "Cargo.toml"
---

# Deltasafe Project Skill

## Architecture

- **CLI** (`cli.rs`, `main.rs`): `server`, `sync`, `discover`, `connect`, `watch`
- **Client** (`sync.rs`): walks source dir, sends JSON `FileHeader`, then framed encrypted chunks
- **Server** (`server.rs`): accepts multiple files per TCP connection in a loop
- **Crypto** (`crypto.rs`): PBKDF2 password derivation, hex keys
- **Discovery** (`discovery.rs`): port scan (mDNS stub only)
- **Utils** (`utils.rs`): key/address resolution, `KeyRole::Client` vs `KeyRole::Server`

## Transfer Protocol

1. Client sends `u32 BE header_len` + JSON `FileHeader`
2. Server replies `1` byte ack (`1` = ok, `0` = reject)
3. For each chunk: `u32 BE payload_len` + `IV (16 bytes)` + AES-256-CBC ciphertext (PKCS7)
4. Server decrypts until plaintext bytes == `header.file_size`
5. Repeat from step 1 for next file on same connection

## Password Mode

- **Client** (`KeyRole::Client`): generates random 16-byte PBKDF2 salt per sync session, puts hex in `FileHeader.pbkdf2_salt`
- **Server** (`KeyRole::Server`): must start with `--password`; re-derives key from header salt + stored password
- **Hex key mode**: `pbkdf2_salt` is `None`; both sides use the same `--key`

## Conventions

- Received files go to `received_files/` preserving `relative_path`
- Chunk size constant: `CHUNK_SIZE = 4096` (plaintext read size; encrypted payload is larger)
- Prefer `anyhow::Result` over panics in server decrypt path
- Share `FileHeader` and `calculate_file_hash` from `sync.rs`; do not duplicate in `server.rs`
- Run `cargo test` before finishing changes

## Common Tasks

| Task | Guidance |
|------|----------|
| Fix transfer bugs | Verify framed length prefix on both client and server |
| Add file metadata | Extend `FileHeader`; keep backward-compatible serde defaults |
| Improve discovery | Implement real mDNS in `discover_via_mdns`, or document port-scan limits |
| Async refactor | Keep protocol logic testable; avoid blocking tokio runtime in CLI paths |
