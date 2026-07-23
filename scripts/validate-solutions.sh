#!/usr/bin/env bash
# validate-solutions.sh — Tier 2 reference-solution compile/behavior gate.
#
# For every exercise in campaign.toml that has a hints/solution.rs this script:
#   1. Creates an isolated shadow workspace in target/solution-validation/
#   2. Copies each exercise directory with src/lib.rs replaced by hints/solution.rs
#   3. Runs `cargo test --package <id>` against the shadow workspace
#
# The learner's working-tree src/lib.rs is NEVER touched.
# Diagnostics are printed per-exercise with full cargo output on failure.
#
# Usage:
#   ./scripts/validate-solutions.sh              # validate all present solutions
#   ./scripts/validate-solutions.sh ex01-format-scoreboard  # single exercise
#
# Run in CI with:
#   ./scripts/validate-solutions.sh
#
# Requires: cargo, rsync (for workspace copy).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SHADOW_ROOT="$WORKSPACE_ROOT/target/solution-validation"
BUILD_DIR="$WORKSPACE_ROOT/target/solution-validation-build"

# Optional single-exercise filter passed as first argument.
TARGET_EXERCISE="${1:-}"

cd "$WORKSPACE_ROOT"

echo "═══════════════════════════════════════════════════════"
echo " learn-rust: Reference Solution Compile/Behavior Gate"
echo "═══════════════════════════════════════════════════════"
echo ""

# ── Discover exercises from cargo workspace metadata ──────────────────────
# Metadata-driven: reads the Cargo workspace, no hardcoded IDs or counts.

echo "Discovering exercises via cargo metadata..."
EXERCISE_LIST=$(cargo metadata --format-version=1 --no-deps 2>/dev/null | \
    python3 -c "
import json, sys, os
data = json.load(sys.stdin)
for pkg in sorted(data['packages'], key=lambda p: p['name']):
    name = pkg['name']
    manifest = pkg['manifest_path']
    # Exercise packages match: exNN-<name> (at least 5 chars, ex + two digits)
    if (len(name) >= 5
            and name[:2] == 'ex'
            and name[2:4].isdigit()
            and '-' in name[4:]):
        ex_dir = os.path.dirname(manifest)
        world_dir = os.path.dirname(ex_dir)
        world_id = os.path.basename(world_dir)
        print(world_id + ':' + name)
")

if [ -z "$EXERCISE_LIST" ]; then
    echo "ERROR: no exercises found via cargo metadata" >&2
    exit 1
fi

TOTAL=$(echo "$EXERCISE_LIST" | wc -l | tr -d ' ')
echo "Found $TOTAL exercises in workspace."
echo ""

# ── Create shadow workspace ───────────────────────────────────────────────

echo "Creating shadow workspace at target/solution-validation/ ..."
rm -rf "$SHADOW_ROOT"
# Copy workspace source (exclude target/ and .git/ to keep it lean).
rsync -a \
    --exclude='target/' \
    --exclude='.git/' \
    --exclude='.direnv/' \
    "$WORKSPACE_ROOT/" "$SHADOW_ROOT/"

# ── Validate each exercise ────────────────────────────────────────────────

PASS=0
FAIL=0
SKIP=0
FAILURES=()

while IFS=':' read -r world_id ex_id; do
    # Filter to a single exercise if one was requested.
    if [ -n "$TARGET_EXERCISE" ] && [ "$ex_id" != "$TARGET_EXERCISE" ]; then
        continue
    fi

    solution="$WORKSPACE_ROOT/exercises/$world_id/$ex_id/hints/solution.rs"

    if [ ! -f "$solution" ]; then
        printf "  ⏭  SKIP  %s  (no hints/solution.rs yet)\n" "$ex_id"
        SKIP=$((SKIP + 1))
        continue
    fi

    # Substitute solution for the learner stub in the shadow copy.
    shadow_lib="$SHADOW_ROOT/exercises/$world_id/$ex_id/src/lib.rs"
    cp "$solution" "$shadow_lib"

    printf "  🔨 TEST  %s ... " "$ex_id"

    # Run cargo test for this package only; redirect build artifacts to a
    # dedicated directory so they can be reused across validation runs.
    if cargo test \
            --manifest-path "$SHADOW_ROOT/Cargo.toml" \
            --package "$ex_id" \
            --target-dir "$BUILD_DIR" \
            2>&1 | sed 's/^/    /'; then
        printf "  ✅ PASS  %s\n" "$ex_id"
        PASS=$((PASS + 1))
    else
        printf "  ❌ FAIL  %s\n" "$ex_id"
        FAIL=$((FAIL + 1))
        FAILURES+=("$ex_id")
    fi

    echo ""

done <<< "$EXERCISE_LIST"

# ── Summary ───────────────────────────────────────────────────────────────

echo "═══════════════════════════════════════════════════════"
echo " Results: ${PASS} passed  ${FAIL} failed  ${SKIP} skipped"
echo "═══════════════════════════════════════════════════════"

if [ "${#FAILURES[@]}" -gt 0 ]; then
    echo ""
    echo "Failed exercises:"
    for f in "${FAILURES[@]}"; do
        echo "  ❌  $f"
    done
    exit 1
fi

if [ "$PASS" -eq 0 ] && [ "$SKIP" -gt 0 ]; then
    echo ""
    echo "NOTE: No solutions were validated (all skipped — solution files not yet present)."
    echo "      Run again after solution artifacts are complete."
fi
