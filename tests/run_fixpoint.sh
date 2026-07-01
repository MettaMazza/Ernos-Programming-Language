#!/usr/bin/env bash
# run_fixpoint.sh — the self-hosting fixpoint gate.
#
#   gen1 = epc built by the Rust compiler
#   gen2 = epc built by gen1        (must succeed)
#   gen3 = epc built by gen2        (must succeed)
#   gen2 and gen3 must be byte-identical.
#
# Every stage's exit code is checked: a stale binary passing `cmp` must never
# masquerade as a fixpoint.
set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$SCRIPT_DIR"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

./target/release/ernos epc.ep >"$TMP/g1.log" 2>&1 || { echo "FIXPOINT FAIL: Rust compiler cannot build epc.ep"; tail -5 "$TMP/g1.log"; exit 1; }
cp epc "$TMP/gen1"
"$TMP/gen1" epc.ep >"$TMP/g2.log" 2>&1 || { echo "FIXPOINT FAIL: gen1 epc cannot compile epc.ep"; grep -iE "error" "$TMP/g2.log" | head -5; exit 1; }
cp epc "$TMP/gen2"
"$TMP/gen2" epc.ep >"$TMP/g3.log" 2>&1 || { echo "FIXPOINT FAIL: gen2 epc cannot compile epc.ep"; grep -iE "error" "$TMP/g3.log" | head -5; exit 1; }
cp epc "$TMP/gen3"
cmp -s "$TMP/gen2" "$TMP/gen3" || { echo "FIXPOINT FAIL: gen2 and gen3 binaries differ"; exit 1; }
./epc tests/test_basic_math.ep >/dev/null 2>&1 && ./tests/test_basic_math >/dev/null 2>&1 || ./test_basic_math >/dev/null 2>&1 || { echo "FIXPOINT FAIL: fixpoint epc cannot compile+run test_basic_math"; exit 1; }
rm -f test_basic_math test_basic_math_compiled.c tests/test_basic_math 2>/dev/null
echo "FIXPOINT OK: 3-stage byte-identical, basic-math smoke passes"
