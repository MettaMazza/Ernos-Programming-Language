#!/usr/bin/env bash
# Native backend smoke + safety gate.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

COMPILER=./target/release/ernos
VALID=tests/test_native_basic.ep
INVALID=tests/test_arg_type_mismatch.ep
OWNERSHIP_INVALID=forensic/test_safety_move.ep
BIN=tests/test_native_basic
trap 'rm -f "$BIN" tests/test_native_basic_native.s tests/test_native_basic_native.o tests/test_native_basic_runtime.c tests/test_native_basic_runtime.o' EXIT

[[ -x "$COMPILER" ]] || { echo "build the release compiler first" >&2; exit 2; }

"$COMPILER" "$VALID" --native >/tmp/ernos_native_compile.log 2>&1
actual=$("$BIN")
expected=$(cat tests/test_native_basic.expected)
[[ "$actual" == "$expected" ]] || {
    echo "native backend output mismatch" >&2
    diff <(printf '%s\n' "$expected") <(printf '%s\n' "$actual")
    exit 1
}

if "$COMPILER" "$INVALID" --native >/tmp/ernos_native_reject.log 2>&1; then
    echo "native backend accepted an invalid typed program" >&2
    exit 1
fi
if "$COMPILER" "$OWNERSHIP_INVALID" --native >/tmp/ernos_native_ownership_reject.log 2>&1; then
    echo "native backend accepted a use-after-move" >&2
    exit 1
fi

echo "native gate: PASS (valid program runs; type and ownership errors rejected)"
