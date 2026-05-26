#!/usr/bin/env bash
#
# run_tests.sh — Compile and run each tests/*.ep file, checking:
#   1. Compilation succeeds (exit 0 from cargo run)
#   2. Execution succeeds (exit 0 from the binary)
#   3. Output matches expected values from # expected: comments
#
# Usage: ./run_tests.sh [--verbose]
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

VERBOSE=0
[[ "${1:-}" == "--verbose" ]] && VERBOSE=1

PASS=0
FAIL=0
SKIP=0
FAILURES=""

for TEST_FILE in tests/test_*.ep conformance/test_*.ep; do
    [[ -f "$TEST_FILE" ]] || continue
    NAME=$(basename "$TEST_FILE" .ep)
    BINARY="./$NAME"

    # ── Check for expected compile error ──
    EXPECT_COMPILE_ERROR=0
    if head -5 "$TEST_FILE" | grep -q '# expected_compile_error'; then
        EXPECT_COMPILE_ERROR=1
    fi

    # ── Compile ──
    if ! COMPILE_OUT=$(cargo run -- "$TEST_FILE" 2>&1); then
        if [[ $EXPECT_COMPILE_ERROR -eq 1 ]]; then
            echo "PASS  $NAME  (expected compile error)"
            PASS=$((PASS + 1))
        else
            echo "FAIL  $NAME  (compilation failed)"
            [[ $VERBOSE -eq 1 ]] && echo "$COMPILE_OUT" | sed 's/^/      /'
            FAIL=$((FAIL + 1))
            FAILURES="$FAILURES  $NAME (compile)\n"
        fi
        continue
    fi

    if [[ $EXPECT_COMPILE_ERROR -eq 1 ]]; then
        echo "FAIL  $NAME  (expected compile error but succeeded)"
        FAIL=$((FAIL + 1))
        FAILURES="$FAILURES  $NAME (should have failed)\n"
        rm -f "$BINARY" "${NAME}_compiled.c"
        continue
    fi

    if [[ ! -x "$BINARY" ]]; then
        echo "FAIL  $NAME  (binary not produced)"
        FAIL=$((FAIL + 1))
        FAILURES="$FAILURES  $NAME (no binary)\n"
        continue
    fi

    # ── Run ──
    ACTUAL=$("$BINARY" 2>/dev/null) || EXIT_CODE=$?
    EXIT_CODE=${EXIT_CODE:-0}

    if [[ $EXIT_CODE -ne 0 ]]; then
        echo "FAIL  $NAME  (exit code $EXIT_CODE)"
        [[ $VERBOSE -eq 1 ]] && echo "      output: $ACTUAL"
        FAIL=$((FAIL + 1))
        FAILURES="$FAILURES  $NAME (exit $EXIT_CODE)\n"
        rm -f "$BINARY" "${NAME}_compiled.c"
        continue
    fi

    # ── Check expected output ──
    # Extract "# expected: <line>" comments from the top of the file
    EXPECTED=""
    while IFS= read -r line; do
        if [[ "$line" =~ ^#\ expected:\ (.+)$ ]]; then
            VAL="${BASH_REMATCH[1]}"
            if [[ "$VAL" == "(no output)" ]]; then
                EXPECTED=""
            else
                if [[ -n "$EXPECTED" ]]; then
                    EXPECTED="$EXPECTED"$'\n'"$VAL"
                else
                    EXPECTED="$VAL"
                fi
            fi
        elif [[ "$line" =~ ^#\ expected_last:\ (.+)$ ]]; then
            # Only check that the last line of output matches
            LAST_LINE=$(echo "$ACTUAL" | tail -1)
            EXPECTED_LAST="${BASH_REMATCH[1]}"
            if [[ "$LAST_LINE" != "$EXPECTED_LAST" ]]; then
                echo "FAIL  $NAME  (last line mismatch)"
                echo "      expected last: $EXPECTED_LAST"
                echo "      actual last:   $LAST_LINE"
                FAIL=$((FAIL + 1))
                FAILURES="$FAILURES  $NAME (output)\n"
                rm -f "$BINARY" "${NAME}_compiled.c"
                continue 2
            fi
            # Mark as checked, skip the full-output comparison
            EXPECTED="__LAST_CHECKED__"
        elif [[ "$line" =~ ^# ]]; then
            continue  # skip other comments
        else
            break  # stop at first non-comment line
        fi
    done < "$TEST_FILE"

    if [[ "$EXPECTED" == "__LAST_CHECKED__" ]]; then
        echo "PASS  $NAME"
        PASS=$((PASS + 1))
    elif [[ -z "$EXPECTED" && -z "$ACTUAL" ]]; then
        echo "PASS  $NAME"
        PASS=$((PASS + 1))
    elif [[ -n "$EXPECTED" && "$ACTUAL" == "$EXPECTED" ]]; then
        echo "PASS  $NAME"
        PASS=$((PASS + 1))
    elif [[ -z "$EXPECTED" && -n "$ACTUAL" ]]; then
        # No expected output specified but binary produced output — still pass on exit code
        echo "PASS  $NAME  (no expected output defined, exit 0)"
        PASS=$((PASS + 1))
    else
        echo "FAIL  $NAME  (output mismatch)"
        echo "      expected: $(echo "$EXPECTED" | head -3)"
        echo "      actual:   $(echo "$ACTUAL" | head -3)"
        FAIL=$((FAIL + 1))
        FAILURES="$FAILURES  $NAME (output)\n"
    fi

    # Clean up compiled artifacts
    rm -f "$BINARY" "${NAME}_compiled.c"
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Results: $PASS passed, $FAIL failed"
if [[ $FAIL -gt 0 ]]; then
    echo ""
    echo "  Failures:"
    echo -e "$FAILURES"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 1
else
    echo "  All tests passed ✓"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 0
fi
