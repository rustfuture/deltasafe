#!/usr/bin/env bash
#
# Reproducible two-endpoint loopback demo for deltasafe.
#
# Builds the current checkout, starts one receiver on a free loopback port in an
# owned temporary directory, performs a password-mode transfer, verifies
# byte/hash equality, checks that a wrong password of valid length fails
# without publishing, and cleans up only the resources it created.
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

echo "== deltasafe loopback demo =="
echo "repo:    $REPO_ROOT"
echo "commit:  $(git rev-parse HEAD)"
dirty_count="$(git status --porcelain | wc -l | tr -d ' ')"
echo "dirty:   $dirty_count path(s)"
echo "command: cargo build --locked"

cargo build --locked
BIN="$REPO_ROOT/target/debug/deltasafe"
BIN_SHA="$(shasum -a 256 "$BIN" | awk '{print $1}')"
echo "binary:  $BIN"
echo "sha256:  $BIN_SHA"

WORK="$(mktemp -d "${TMPDIR:-/tmp}/deltasafe-demo.XXXXXX")"
SRC="$WORK/source"
RECV_ROOT="$WORK/receiver"
mkdir -p "$SRC/nested" "$RECV_ROOT"
echo "workdir: $WORK"

SERVER_PID=""
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

# Reserve a free loopback port, then release it for the receiver to bind.
# DELTASAFE_DEMO_ADDR overrides the address (used to prove that a controlled
# startup failure makes this harness exit non-zero).
if [[ -n "${DELTASAFE_DEMO_ADDR:-}" ]]; then
  ADDR="$DELTASAFE_DEMO_ADDR"
  PORT="${ADDR##*:}"
else
  PORT="$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1]); s.close()')"
  ADDR="127.0.0.1:$PORT"
fi
SERVER_LOG="$WORK/server.log"
CLIENT_LOG="$WORK/client.log"

# The receiver writes under received_files/ relative to its own working dir, so
# run it from the owned receive root instead of the repository root.
( cd "$RECV_ROOT" && exec "$BIN" server --address "$ADDR" --password "demo-password-123" ) \
  >"$SERVER_LOG" 2>&1 &
SERVER_PID=$!

ready=0
for _ in $(seq 1 100); do
  if ! kill -0 "$SERVER_PID" 2>/dev/null; then
    echo "FAIL: receiver exited before becoming ready" >&2
    cat "$SERVER_LOG" >&2
    exit 1
  fi
  if (exec 3<>"/dev/tcp/127.0.0.1/$PORT") 2>/dev/null; then
    exec 3>&- 2>/dev/null || true
    ready=1
    break
  fi
  sleep 0.1
done
if [[ $ready -ne 1 ]]; then
  echo "FAIL: receiver did not accept connections in time" >&2
  cat "$SERVER_LOG" >&2
  exit 1
fi
echo "receiver ready on $ADDR (pid $SERVER_PID)"

# Wrong password with a valid length: must fail and publish nothing.
echo "== wrong password (valid length) =="
set +e
"$BIN" sync --source "$SRC" --target "$ADDR" --password "wrong-password-456" >"$CLIENT_LOG" 2>&1
wrong_status=$?
set -e
if [[ $wrong_status -eq 0 ]]; then
  echo "FAIL: sender accepted a wrong password" >&2
  cat "$CLIENT_LOG" >&2
  exit 1
fi
if [[ -e "$RECV_ROOT/received_files/hello.txt" ]]; then
  echo "FAIL: wrong password published a destination file" >&2
  exit 1
fi
echo "wrong password rejected (sender exit $wrong_status); no file published"

# Correct password: verify byte-for-byte equality for every source file.
echo "== correct password transfer =="
"$BIN" sync --source "$SRC" --target "$ADDR" --password "demo-password-123" >"$CLIENT_LOG" 2>&1
for rel in empty.bin hello.txt nested/large.bin; do
  if ! cmp -s "$SRC/$rel" "$RECV_ROOT/received_files/$rel"; then
    echo "FAIL: $rel differs or is missing" >&2
    exit 1
  fi
  echo "verified: $rel ($(wc -c < "$SRC/$rel" | tr -d ' ') bytes)"
done

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
