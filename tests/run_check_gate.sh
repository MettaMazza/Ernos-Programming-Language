#!/usr/bin/env bash
# Check-only mode must perform full semantic and ownership validation.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

RUST=./target/release/ernos
EPC=./epc
VALID=tests/test_native_basic.ep
INVALID=forensic/test_safety_move.ep

[[ -x "$RUST" ]] || { echo "build the release compiler first" >&2; exit 2; }
[[ -x "$EPC" ]] || { echo "build the self-hosted compiler first" >&2; exit 2; }

"$RUST" check "$VALID" >/dev/null
"$EPC" check "$VALID" >/dev/null

if "$RUST" check "$INVALID" >/dev/null 2>&1; then
    echo "ernos check accepted a use-after-move" >&2
    exit 1
fi
if "$EPC" check "$INVALID" >/dev/null 2>&1; then
    echo "epc check accepted a use-after-move" >&2
    exit 1
fi

echo "check gate: PASS (both compilers validate ownership without codegen)"
