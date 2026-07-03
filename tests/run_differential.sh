#!/usr/bin/env bash
# run_differential.sh — differential testing between the two compilers.
#
# Every program in tests/differential/ is compiled by BOTH the Rust reference
# compiler (./target/release/ernos) and the self-hosted compiler (./epc); both
# binaries are run and their stdout + exit codes compared. The two compilers
# must agree exactly — on success AND on rejection. Any divergence is a bug in
# one of them (this suite found a `not (a > b)` miscompilation, a grammar
# drift on single-line closures, and three type-checker acceptance gaps).
#
# Exit code: 0 only if every program agrees (or is rejected by both).
set -uo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
DIR="tests/differential"
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

if [ ! -x ./target/release/ernos ]; then echo "build the Rust compiler first (cargo build --release)"; exit 1; fi
if [ ! -x ./epc ]; then echo "build epc first (./target/release/ernos epc.ep)"; exit 1; fi

pass=0; div=0; crash=0
for f in "$DIR"/*.ep; do
  name=$(basename "$f" .ep)
  mkdir -p "$WORK/$name/r" "$WORK/$name/e"
  cp "$f" "$WORK/$name/r/$name.ep"; cp "$f" "$WORK/$name/e/$name.ep"
  (cd "$WORK/$name/r" && "$ROOT/target/release/ernos" "$name.ep" >compile.log 2>&1); rrc=$?
  (cd "$WORK/$name/e" && "$ROOT/epc" "$name.ep" >compile.log 2>&1); erc=$?
  if [ $rrc -ne 0 ] && [ $erc -ne 0 ]; then
    echo "BOTH-REJECT  $name"; pass=$((pass+1)); continue
  fi
  if [ $rrc -ne 0 ] || [ $erc -ne 0 ]; then
    echo "COMPILE-DIVERGENCE $name: rust_rc=$rrc epc_rc=$erc"; div=$((div+1)); continue
  fi
  (cd "$WORK/$name/r" && perl -e 'alarm 20; exec @ARGV' "./$name" >run.out 2>run.err); rrun=$?
  (cd "$WORK/$name/e" && perl -e 'alarm 20; exec @ARGV' "./$name" >run.out 2>run.err); erun=$?
  if [ $rrun -ge 128 ] || [ $erun -ge 128 ]; then
    echo "CRASH $name: rust_run=$rrun epc_run=$erun"; crash=$((crash+1)); continue
  fi
  if [ $rrun -ne $erun ] || ! diff -q "$WORK/$name/r/run.out" "$WORK/$name/e/run.out" >/dev/null; then
    echo "RUN-DIVERGENCE $name: rust_rc=$rrun epc_rc=$erun"
    diff "$WORK/$name/r/run.out" "$WORK/$name/e/run.out" | head -6 | sed 's/^/    /'
    div=$((div+1)); continue
  fi
  echo "AGREE        $name"; pass=$((pass+1))
done
total=$((pass+div+crash))
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  differential: $pass/$total agree, $div divergence, $crash crash"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
[ $div -eq 0 ] && [ $crash -eq 0 ]
