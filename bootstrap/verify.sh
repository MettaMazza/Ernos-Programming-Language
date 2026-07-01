#!/usr/bin/env bash
# verify.sh — prove the toolchain is fully self-contained (Rust retirable).
#
# Chain, using ONLY clang:
#   boot0 = clang(bootstrap/epc_bootstrap.c)
#   boot1 = boot0 compiling epc.ep
#   boot2 = boot1 compiling epc.ep     (must be byte-identical to a 3rd stage)
#   boot3 = boot2 compiling epc.ep
#   assert boot2 == boot3   (fixpoint reached from the frozen C, no Rust)
# Then run the epc parity suite with the freshly bootstrapped compiler.
set -uo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
CC="${CC:-clang}"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

echo "[verify] clang-only build of the frozen bootstrap ..."
"$CC" -O2 bootstrap/epc_bootstrap.c -o "$TMP/boot0" -lpthread || { echo "FAIL: bootstrap C did not compile"; exit 1; }

echo "[verify] boot0 -> epc.ep -> boot1 ..."
"$TMP/boot0" epc.ep >"$TMP/l1.log" 2>&1 || { echo "FAIL: boot0 cannot compile epc.ep"; grep -iE error "$TMP/l1.log" | head; exit 1; }
cp epc "$TMP/boot1"
"$TMP/boot1" epc.ep >"$TMP/l2.log" 2>&1 || { echo "FAIL: boot1 cannot compile epc.ep"; grep -iE error "$TMP/l2.log" | head; exit 1; }
cp epc "$TMP/boot2"
"$TMP/boot2" epc.ep >"$TMP/l3.log" 2>&1 || { echo "FAIL: boot2 cannot compile epc.ep"; grep -iE error "$TMP/l3.log" | head; exit 1; }
cp epc "$TMP/boot3"
cmp -s "$TMP/boot2" "$TMP/boot3" || { echo "FAIL: no fixpoint from the frozen C (boot2 != boot3)"; exit 1; }
echo "[verify] fixpoint reached from frozen C — Rust not used."

# Freshness: the committed bootstrap must match what today's epc.ep produces,
# so the artifact can never silently drift from source.
"$TMP/boot2" epc.ep >/dev/null 2>&1
if ! cmp -s epc_compiled.c bootstrap/epc_bootstrap.c; then
    echo "FAIL: bootstrap/epc_bootstrap.c is stale — regenerate it:"
    echo "      ./epc epc.ep && cp epc_compiled.c bootstrap/epc_bootstrap.c"
    exit 1
fi
echo "[verify] committed bootstrap is fresh (matches epc.ep)."

echo "[verify] running epc parity suite with the bootstrapped compiler ..."
bash tests/run_epc_parity.sh | grep -E "epc parity|rejection"
echo "[verify] OK — fully self-contained."
