#!/usr/bin/env bash
#
# Reproducible two-endpoint loopback demo for deltasafe.
#
# Builds the current checkout, starts one receiver on a free loopback port in an
# owned temporary directory, performs a password-mode transfer, verifies
# byte/hash equality for every fixture, checks that a wrong password of valid
# length fails without publishing a file anywhere under the receiver root, and
# cleans up only the resources it created. On failure the receiver and sender
# logs are printed before cleanup and, when DELTASAFE_DEMO_EVIDENCE_DIR is set,
# copied there so they survive cleanup.
#
# Readiness requires all three: the receiver process is alive, its log contains
# the `Receiver listening on <addr>` marker, and the listener accepts a TCP
# connection. A TCP connect alone is never treated as ownership proof.
#
# This is a localhost demonstration. It is not a cross-device test and not a
# production security audit. The loopback path exercises the same protocol as a
# real transfer, but both endpoints run on one host.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if ! command -v python3 >/dev/null 2>&1; then
  echo "FAIL: python3 is required to pick a free loopback port" >&2
  exit 1
fi

hash_file() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
  else
    sha256sum "$1" | awk '{print $1}'
  fi
}

echo "== deltasafe loopback demo =="
echo "repo:    $REPO_ROOT"
echo "commit:  $(git rev-parse HEAD)"
dirty_count="$(git status --porcelain | wc -l | tr -d ' ')"
echo "dirty:   $dirty_count path(s)"
echo "command: cargo build --locked"

cargo build --locked
BIN="$REPO_ROOT/target/debug/deltasafe"
BIN_SHA="$(hash_file "$BIN")"
echo "binary:  $BIN"
echo "sha256:  $BIN_SHA"

WORK="$(mktemp -d "${TMPDIR:-/tmp}/deltasafe-demo.XXXXXX")"
SRC="$WORK/source"
RECV_ROOT="$WORK/receiver"
mkdir -p "$SRC/nested" "$RECV_ROOT"
echo "workdir: $WORK"

EVIDENCE_DIR="${DELTASAFE_DEMO_EVIDENCE_DIR:-}"
SERVER_PID=""
SERVER_LOG=""
CLIENT_LOG=""
FORCED_KILL=0
cleanup() {
  local status=$?
  if [[ -n "$SERVER_PID" ]] && kill -0 "$SERVER_PID" 2>/dev/null; then
    kill "$SERVER_PID" 2>/dev/null || true
    for _ in $(seq 1 50); do
      kill -0 "$SERVER_PID" 2>/dev/null || break
      sleep 0.1
    done
    if kill -0 "$SERVER_PID" 2>/dev/null; then
      FORCED_KILL=1
      kill -9 "$SERVER_PID" 2>/dev/null || true
    fi
  fi
  [[ -n "$SERVER_PID" ]] && wait "$SERVER_PID" 2>/dev/null || true
  if [[ $status -ne 0 ]]; then
    echo "== failure evidence: receiver log ($SERVER_LOG) ==" >&2
    [[ -f "$SERVER_LOG" ]] && cat "$SERVER_LOG" >&2 || true
    echo "== failure evidence: sender log ($CLIENT_LOG) ==" >&2
    [[ -f "$CLIENT_LOG" ]] && cat "$CLIENT_LOG" >&2 || true
    if [[ -n "$EVIDENCE_DIR" ]]; then
      mkdir -p "$EVIDENCE_DIR"
      [[ -f "$SERVER_LOG" ]] && cp "$SERVER_LOG" "$EVIDENCE_DIR/failure-server.log" || true
      [[ -f "$CLIENT_LOG" ]] && cp "$CLIENT_LOG" "$EVIDENCE_DIR/failure-client.log" || true
      echo "failure logs copied to $EVIDENCE_DIR" >&2
    fi
  fi
  rm -rf "$WORK"
  if [[ $FORCED_KILL -eq 1 ]]; then
    echo "cleanup: receiver required SIGKILL" >&2
  fi
  exit "$status"
}
trap cleanup EXIT INT TERM

# Source tree: empty file, small text file, and a nested multi-frame binary.
: > "$SRC/empty.bin"
printf 'Hello, Deltasafe!\n' > "$SRC/hello.txt"
head -c 20000 /dev/urandom > "$SRC/nested/large.bin"
FIXTURES=(empty.bin hello.txt nested/large.bin)

# Reserve a free loopback port, then release it for the receiver to bind.
# DELTASAFE_DEMO_ADDR overrides the address (used to prove that a controlled
# startup failure makes this harness exit non-zero).
if [[ -n "${DELTASAFE_DEMO_ADDR:-}" ]]; then
  ADDR="$DELTASAFE_DEMO_ADDR"
else
  PORT="$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')"
  ADDR="127.0.0.1:$PORT"
fi
PORT="${ADDR##*:}"
SERVER_LOG="$WORK/server.log"
CLIENT_LOG="$WORK/client.log"

# The receiver writes under received_files/ relative to its own working dir, so
# run it from the owned receive root instead of the repository root.
( cd "$RECV_ROOT" && exec "$BIN" server --address "$ADDR" --password "demo-password-123" ) \
  >"$SERVER_LOG" 2>&1 &
SERVER_PID=$!

# Readiness must prove ownership: our own child process is alive, its log shows
# the listening marker for the exact address it was given, and that address
# accepts a connection. A TCP connect to an unrelated listener is not enough.
LISTEN_MARKER="Receiver listening on $ADDR"
ready=0
for _ in $(seq 1 100); do
  if ! kill -0 "$SERVER_PID" 2>/dev/null; then
    echo "FAIL: receiver exited before becoming ready" >&2
    exit 1
  fi
  if grep -Fq "$LISTEN_MARKER" "$SERVER_LOG" 2>/dev/null \
    && (exec 3<>"/dev/tcp/127.0.0.1/$PORT") 2>/dev/null; then
    ready=1
    break
  fi
  sleep 0.1
done
if [[ $ready -ne 1 ]]; then
  echo "FAIL: receiver did not become ready (alive process + log marker + TCP accept)" >&2
  exit 1
fi
echo "receiver ready on $ADDR (pid $SERVER_PID)"
echo "readiness proof: process alive; log marker '$LISTEN_MARKER'; TCP connect accepted"

# Wrong password with a valid length: must fail and publish nothing anywhere
# under the receiver root. The protocol's cleanup contract is that receive_file
# writes into a NamedTempFile in the destination parent and drops it on error,
# so no temporary or destination file may remain. Parent directories are
# created before the file body is received and may legitimately remain, so this
# check rejects files and symlinks but tolerates directories.
echo "== wrong password (valid length) =="
set +e
"$BIN" sync --source "$SRC" --target "$ADDR" --password "wrong-password-456" >"$CLIENT_LOG" 2>&1
wrong_status=$?
set -e
if [[ $wrong_status -eq 0 ]]; then
  echo "FAIL: sender accepted a wrong password" >&2
  exit 1
fi
leftover_files="$(find "$RECV_ROOT" -mindepth 1 \( -type f -o -type l \) -print 2>/dev/null | sort)"
if [[ -n "$leftover_files" ]]; then
  echo "FAIL: wrong password left files or symlinks under the receiver root:" >&2
  printf '%s\n' "$leftover_files" >&2
  exit 1
fi
leftover_dirs="$(find "$RECV_ROOT" -mindepth 1 -type d -print 2>/dev/null | sort)"
echo "wrong password rejected (sender exit $wrong_status); receiver tree has no files or symlinks"
if [[ -n "$leftover_dirs" ]]; then
  echo "note: receiver tree has directories (allowed by the cleanup contract):"
  printf '%s\n' "$leftover_dirs"
fi

# Correct password: verify byte-for-byte and SHA-256 equality for every source
# file, then confirm the receiver tree contains exactly the expected files.
echo "== correct password transfer =="
"$BIN" sync --source "$SRC" --target "$ADDR" --password "demo-password-123" >"$CLIENT_LOG" 2>&1
for rel in "${FIXTURES[@]}"; do
  if [[ ! -f "$RECV_ROOT/received_files/$rel" ]]; then
    echo "FAIL: $rel was not published" >&2
    exit 1
  fi
  if ! cmp -s "$SRC/$rel" "$RECV_ROOT/received_files/$rel"; then
    echo "FAIL: $rel differs byte-for-byte" >&2
    exit 1
  fi
  src_hash="$(hash_file "$SRC/$rel")"
  dst_hash="$(hash_file "$RECV_ROOT/received_files/$rel")"
  if [[ "$src_hash" != "$dst_hash" ]]; then
    echo "FAIL: $rel hash mismatch (source $src_hash, receiver $dst_hash)" >&2
    exit 1
  fi
  echo "verified: $rel ($(wc -c < "$SRC/$rel" | tr -d ' ') bytes, sha256 $src_hash)"
done

expected_inventory="$(printf '%s\n' "${FIXTURES[@]}" | sort)"
actual_inventory="$(cd "$RECV_ROOT/received_files" && find . -type f | sed 's#^\./##' | sort)"
if [[ "$actual_inventory" != "$expected_inventory" ]]; then
  echo "FAIL: receiver tree does not contain exactly the expected files" >&2
  echo "expected:" >&2
  printf '%s\n' "$expected_inventory" >&2
  echo "actual:" >&2
  printf '%s\n' "$actual_inventory" >&2
  exit 1
fi
echo "receiver inventory matches fixtures exactly"

echo "== stopping receiver =="
kill "$SERVER_PID" 2>/dev/null || true
for _ in $(seq 1 50); do
  kill -0 "$SERVER_PID" 2>/dev/null || break
  sleep 0.1
done
if kill -0 "$SERVER_PID" 2>/dev/null; then
  echo "note: receiver still running; cleanup will escalate" >&2
else
  SERVER_PID=""
fi

echo "DEMO PASS"
echo "commit:  $(git rev-parse HEAD)"
echo "binary:  $BIN_SHA"
echo "note: loopback only; wrong-password and integrity behaviors are also covered by tests/integration_tests.rs"
