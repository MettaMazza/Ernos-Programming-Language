#!/usr/bin/env bash
# build.sh — build the ErnosPlain self-hosted compiler from the frozen C
# bootstrap, using ONLY a C compiler. No Rust, no cargo.
#
#   bootstrap/epc_bootstrap.c  is epc compiled by epc (the fixpoint output).
#   Compiling it yields a working ./epc, which can then compile epc.ep (and any
#   .ep program) directly.
#
# Usage:  bash bootstrap/build.sh   ->  produces ./epc in the repo root
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
CC="${CC:-clang}"
echo "[bootstrap] compiling epc from frozen C with $CC ..."
"$CC" -O2 bootstrap/epc_bootstrap.c -o epc-bootstrapped -lpthread
echo "[bootstrap] rebuilding epc.ep with the bootstrapped compiler ..."
./epc-bootstrapped epc.ep
echo "[bootstrap] done — ./epc is ready (Rust was not used)."
