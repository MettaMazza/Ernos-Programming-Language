#!/usr/bin/env bash
# run_key_input_gate.sh — the raw-keyboard builtins behave identically under
# both compilers, driven over a pipe (stdin here is never a terminal, so
# read_key reads bytes and terminal_columns/terminal_rows give the 80x24
# fallback — fully deterministic).
#
#   read_key()          one key code per call, -1 at end of input
#   terminal_columns()  80 when the output is not a terminal
#   terminal_rows()     24 when the output is not a terminal
set -uo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

if [ ! -x ./target/release/ernos ]; then echo "build the Rust compiler first"; exit 1; fi
if [ ! -x ./epc ]; then echo "build epc first"; exit 1; fi

cat > "$TMP/key_probe.ep" <<'EOF'
define main:
    display terminal_columns()
    display terminal_rows()
    set k1 to read_key()
    set k2 to read_key()
    set k3 to read_key()
    set k4 to read_key()
    display k1
    display k2
    display k3
    display k4
    return 0
EOF

# What the probe must print when fed the two bytes "A" and newline:
# 80, 24 (pipe fallback), 65 ('A'), 10 (newline), then -1 -1 at end of input.
cat > "$TMP/expected.txt" <<'EOF'
80
24
65
10
-1
-1
EOF

FAIL=0
for compiler in "./target/release/ernos" "./epc"; do
    name=$(basename "$compiler")
    mkdir -p "$TMP/$name"
    cp "$TMP/key_probe.ep" "$TMP/$name/key_probe.ep"
    ( cd "$TMP/$name" && "$ROOT/$compiler" key_probe.ep >compile.log 2>&1 ) || { echo "FAIL: $name cannot compile the key probe"; FAIL=1; continue; }
    printf 'A\n' | "$TMP/$name/key_probe" > "$TMP/$name/out.txt" 2>&1
    if diff -q "$TMP/expected.txt" "$TMP/$name/out.txt" >/dev/null; then
        echo "  key input gate ($name): PASS"
    else
        echo "  key input gate ($name): FAIL"
        diff "$TMP/expected.txt" "$TMP/$name/out.txt" | head -8 | sed 's/^/    /'
        FAIL=1
    fi
done

if [ "$FAIL" -ne 0 ]; then exit 1; fi
echo "  key input gate: PASS"
