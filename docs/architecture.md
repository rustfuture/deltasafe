# Architecture and protocol state machine

## Runtime components

The CLI resolves a target and a base key, then `sync` traverses regular files in deterministic path order. `server` accepts TCP connections and handles one or more file transactions per connection. The transport is synchronous per connection and uses bounded read/write timeouts; separate connections are handled by separate threads.

## One-file state machine

```text
sender                         receiver
  |                                |
  |-- bounded JSON header -------->| validate version, size, hash, path
  |                                | derive per-file session key
  |<-- authenticated READY --------|
  |-- encrypted DATA(index=n) ---->| decrypt, check sequence and size
  |-- encrypted FINISH ------------>| verify exact size and BLAKE3
  |                                | flush temporary file
  |                                | publish with no-overwrite rename
  |<-- authenticated COMPLETE -----|
```

An error at any step removes the temporary file and returns an authenticated `ERROR` status when the session key is available. The sender reports success only after `COMPLETE`; an early connection close is an error.

## Cryptographic construction

The password/key resolves to a base 32-byte key. Each file header carries a random 16-byte session ID. HKDF-SHA256 derives a separate session key from the base key and that ID. AES-256-GCM authenticates each frame. Nonces contain a direction prefix and the frame index, so client and server control frames occupy disjoint nonce spaces. The exact raw header, direction, frame kind, and index are authenticated as additional data.

This design protects confidentiality and integrity for peers that already share a key. It does not establish a certificate identity or forward secrecy, and it does not maintain a durable replay database across receiver restarts.

## Filesystem boundary

Only normalized relative paths are accepted. Parent directories are created one component at a time; symlink parents, non-directories, destinations outside the canonical receive root, and existing final destinations are rejected. `tempfile::NamedTempFile` keeps partial content in the destination directory and removes it on any error. The final publication uses no-overwrite persistence so a verified transfer cannot silently replace a file.

The remaining local TOCTOU limitation is documented in the README: a hostile local process with permission to mutate the receive tree concurrently would require descriptor-relative, platform-specific APIs to defeat completely.
