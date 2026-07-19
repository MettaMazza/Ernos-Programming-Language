#!/usr/bin/env bash
# run_stdlib_gate.sh — stdlib coherence gate.
#
# Guards against two regression classes that both shipped at some point:
#
#   1. Two stdlib modules defining the same top-level function. Importing
#      both then fails with "Function is defined multiple times" under epc
#      (and the Rust compiler silently drops one copy). Past collisions:
#      fs.ep vs os.ep on get_cwd, os.ep vs datetime.ep on sleep_ms.
#   2. A single module drifting out of the shared grammar or runtime so that
#      importing it fails at all. Past cases: select.ep used the removed
#      `receive x from ch` statement form, toml.ep used bare `and` as a
#      boolean operator (Rust-only grammar; the portable form is `and also`).
#
# Part 1  Textual audit: no top-level function name may be defined twice
#         across stdlib/*.ep.
# Part 2  One program importing every linkable stdlib module is compiled by
#         BOTH compilers, run, and the outputs must match exactly.
# Part 3  Modules excluded from Part 2 for external C library dependencies
#         still get full check-only coverage from both compilers.
#
# Exit code: 0 only if every part passes.
set -uo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if [ ! -x ./target/release/ernos ]; then echo "build the Rust compiler first (cargo build --release)"; exit 1; fi
if [ ! -x ./epc ]; then echo "build epc first (./target/release/ernos epc.ep)"; exit 1; fi

WORK=$(mktemp -d)
# The compilers must run from the repo root so bare stdlib imports resolve,
# so the program and its build artifacts live in ROOT under this stem.
# NOTE: the stem must not end in "gui", "sql", or "crypto" — the compilers
# key external link flags off an ends_with("<module>.ep") test on every
# parsed path, including the program's own.
STEM="stdlib_gate_prog"
trap 'rm -rf "$WORK" "$ROOT/$STEM" "$ROOT/${STEM}.ep" "$ROOT/${STEM}_compiled.c" "$ROOT/${STEM}_check.ep"' EXIT

fail=0

# ───────────────── Part 1: duplicate top-level function names ─────────────────
dups=$(awk '
  /^define |^async define / {
    name = $0
    sub(/^async /, "", name); sub(/^define /, "", name)
    sub(/[ :(].*/, "", name)
    count[name]++
    files[name] = files[name] " " FILENAME
  }
  END { for (n in count) if (count[n] > 1) print n ":" files[n] }
' stdlib/*.ep)
if [ -n "$dups" ]; then
  echo "DUPLICATE-DEFINE: the same function is defined in more than one stdlib module."
  echo "Pick one canonical owner per name (see stdlib/os.ep for get_cwd/sleep_ms precedent):"
  echo "$dups" | sed 's/^/    /'
  fail=1
else
  echo "audit        no duplicate top-level function names across $(ls stdlib/*.ep | wc -l | tr -d ' ') modules"
fi

# ───────────────── Part 2: import every linkable module at once ─────────────────
# A tiny compile+link probe decides whether optional C libraries exist on
# this machine; modules whose library is missing are excluded here but still
# checked in Part 3.
probe_lib() {
  printf 'int main(void){return 0;}' | clang -x c - -o "$WORK/probe" "$@" >/dev/null 2>&1
}

skip_link=""   # modules excluded from the link test, with reasons for the log
link_modules=""
for f in stdlib/*.ep; do
  m=$(basename "$f" .ep)
  case "$m" in
    gui)
      probe_lib -lraylib || { skip_link="$skip_link $m(no-raylib)"; continue; } ;;
    sql)
      probe_lib -lsqlite3 || { skip_link="$skip_link $m(no-sqlite3)"; continue; } ;;
    crypto)
      probe_lib -L/opt/homebrew/opt/openssl/lib -lcrypto || { skip_link="$skip_link $m(no-libcrypto)"; continue; } ;;
  esac
  link_modules="$link_modules $m"
done

{
  for m in $link_modules; do echo "import \"$m\""; done
  printf '\ndefine main:\n    display "stdlib import gate ok"\n'
} > "$ROOT/${STEM}.ep"

[ -n "$skip_link" ] && echo "link-skip    not link-tested:$skip_link"
echo "link-test    importing:$(echo "$link_modules" | tr -s ' ')"

if ! ./target/release/ernos "${STEM}.ep" > "$WORK/rust_compile.log" 2>&1; then
  echo "RUST-COMPILE-FAIL: the Rust compiler rejected the all-imports program"
  tail -15 "$WORK/rust_compile.log" | sed 's/^/    /'
  fail=1
else
  "./$STEM" > "$WORK/rust_run.out" 2>&1 || { echo "RUST-RUN-FAIL"; fail=1; }
fi

if ! ./epc "${STEM}.ep" > "$WORK/epc_compile.log" 2>&1; then
  echo "EPC-COMPILE-FAIL: the self-hosted compiler rejected the all-imports program"
  tail -15 "$WORK/epc_compile.log" | sed 's/^/    /'
  fail=1
else
  "./$STEM" > "$WORK/epc_run.out" 2>&1 || { echo "EPC-RUN-FAIL"; fail=1; }
fi

if [ $fail -eq 0 ]; then
  if diff -q "$WORK/rust_run.out" "$WORK/epc_run.out" >/dev/null; then
    echo "link-test    both compilers built and ran it, identical output"
  else
    echo "OUTPUT-DIVERGENCE between the two compilers:"
    diff "$WORK/rust_run.out" "$WORK/epc_run.out" | head -6 | sed 's/^/    /'
    fail=1
  fi
fi

# ───────────────── Part 3: --check coverage for link-excluded modules ─────────────────
# Both check modes resolve imports and perform semantic + ownership checks
# without invoking clang, so grammar or safety drift is still caught.
skipped_names=$(echo "$skip_link" | tr ' ' '\n' | sed 's/(.*//' | grep -v '^$' || true)
if [ -n "$skipped_names" ]; then
  {
    for m in $skipped_names; do echo "import \"$m\""; done
    printf '\ndefine main:\n    display "ok"\n'
  } > "$ROOT/${STEM}_check.ep"
  if ./target/release/ernos --check "${STEM}_check.ep" > "$WORK/check.log" 2>&1; then
    echo "check-only   ernos ok:$(echo " $skipped_names" | tr '\n' ' ')"
  else
    echo "CHECK-FAIL: a link-excluded module no longer parses/typechecks:"
    tail -15 "$WORK/check.log" | sed 's/^/    /'
    fail=1
  fi
  if ./epc check "${STEM}_check.ep" > "$WORK/epc_check.log" 2>&1; then
    echo "check-only   epc ok:$(echo " $skipped_names" | tr '\n' ' ')"
  else
    echo "EPC-CHECK-FAIL: a link-excluded module no longer passes static validation:"
    tail -15 "$WORK/epc_check.log" | sed 's/^/    /'
    fail=1
  fi
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $fail -eq 0 ]; then
  echo "  stdlib gate: PASS"
else
  echo "  stdlib gate: FAIL"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
exit $fail
