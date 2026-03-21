#!/bin/bash
# Generate problems from all Julia scripts, collect into SQLite, upload to GitHub Releases.
# Usage: ./generate_and_publish.sh [--workers N] [--count N] [--timeout N] [--tag TAG] [--dry-run]

set -euo pipefail

SCRIPTS_DIR="$(cd "$(dirname "$0")/backend/scripts" && pwd)"
SYSIMAGE="$SCRIPTS_DIR/julia/sysimage.so"
PROJECT="$SCRIPTS_DIR/julia"
COUNT=10000
WORKERS=$(nproc)
TIMEOUT=300
TAG="problems-$(date +%Y%m%d)"
DRY_RUN=0
DB="problems.db"
REPO="FizzWizZleDazzle/locus-scripts"

while [[ $# -gt 0 ]]; do
    case $1 in
        --workers) WORKERS="$2"; shift 2 ;;
        --count)   COUNT="$2"; shift 2 ;;
        --timeout) TIMEOUT="$2"; shift 2 ;;
        --tag)     TAG="$2"; shift 2 ;;
        --dry-run) DRY_RUN=1; shift ;;
        *) echo "Unknown arg: $1"; exit 1 ;;
    esac
done

TOTAL=$(ls "$SCRIPTS_DIR"/src/*.jl | wc -l)
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "=== Locus Problem Generator ==="
echo "Scripts:  $TOTAL Julia files"
echo "Count:    $COUNT per script"
echo "Workers:  $WORKERS parallel"
echo "Timeout:  ${TIMEOUT}s per script"
echo "Output:   $DB"
echo "Tag:      $TAG"
echo "Sysimage: $([ -f "$SYSIMAGE" ] && echo 'yes' || echo 'no')"
echo "Dry run:  $([ "$DRY_RUN" -eq 1 ] && echo 'yes' || echo 'no')"
echo ""

JULIA_EXTRA_ARGS=""
if [ -f "$SYSIMAGE" ]; then
    JULIA_EXTRA_ARGS="--sysimage=$SYSIMAGE"
fi

# --- Phase 1: Generate ---
echo "=== Phase 1: Generate ==="

PROGRESS_FILE="$TMPDIR/progress"
echo "0" > "$PROGRESS_FILE"
PASS_COUNT="$TMPDIR/pass_count"
echo "0" > "$PASS_COUNT"
FAIL_LOG="$TMPDIR/failures.log"
touch "$FAIL_LOG"

run_script() {
    local script="$1"
    local name
    name=$(basename "$script" .jl)

    local start_ms end_ms duration
    start_ms=$(date +%s%3N)

    local tmpout tmperr
    tmpout="$TMPDIR/${name}.jsonl"
    tmperr=$(mktemp)

    local exit_code=0
    timeout "$TIMEOUT" julia $JULIA_EXTRA_ARGS --project="$PROJECT" "$script" --count "$COUNT" \
        >"$tmpout" 2>"$tmperr" || exit_code=$?

    end_ms=$(date +%s%3N)
    duration=$(( end_ms - start_ms ))

    local num_problems=0
    if [ -s "$tmpout" ]; then
        num_problems=$(grep -c '^{' "$tmpout" 2>/dev/null || true)
    fi

    local success=1
    local error_msg=""
    if [ "$exit_code" -ne 0 ]; then
        success=0
        if [ "$exit_code" -eq 124 ]; then
            error_msg="TIMEOUT after ${TIMEOUT}s"
        else
            error_msg=$(tail -c 500 "$tmperr" | tr '\0' ' ')
        fi
        # Remove empty output files for failed scripts
        [ "$num_problems" -eq 0 ] && rm -f "$tmpout"
    elif [ "$num_problems" -eq 0 ]; then
        success=0
        error_msg="No JSON output"
        rm -f "$tmpout"
    fi

    rm -f "$tmperr"

    # Thread-safe progress counter
    local done
    done=$(python3 -c "
import fcntl
f='$PROGRESS_FILE'
with open(f,'r+') as fh:
    fcntl.flock(fh, fcntl.LOCK_EX)
    n=int(fh.read().strip())+1; fh.seek(0); fh.write(str(n)); fh.truncate()
    print(n)
")

    if [ "$success" -eq 1 ]; then
        python3 -c "
import fcntl
f='$PASS_COUNT'
with open(f,'r+') as fh:
    fcntl.flock(fh, fcntl.LOCK_EX)
    n=int(fh.read().strip())+1; fh.seek(0); fh.write(str(n)); fh.truncate()
"
        printf "[%d/%d] PASS  %-55s %6d problems  %5dms\n" "$done" "$TOTAL" "$name" "$num_problems" "$duration"
    else
        printf "[%d/%d] FAIL  %-55s %6d problems  %5dms  %s\n" "$done" "$TOTAL" "$name" "$num_problems" "$duration" "$error_msg" >> "$FAIL_LOG"
        printf "[%d/%d] FAIL  %-55s %6d problems  %5dms\n" "$done" "$TOTAL" "$name" "$num_problems" "$duration"
    fi
}

export -f run_script
export SCRIPTS_DIR SYSIMAGE PROJECT COUNT WORKERS TIMEOUT JULIA_EXTRA_ARGS TOTAL TMPDIR PROGRESS_FILE PASS_COUNT FAIL_LOG

GEN_START=$(date +%s)

ls "$SCRIPTS_DIR"/src/*.jl | \
    xargs -P "$WORKERS" -I{} bash -c 'run_script "$@"' _ {}

GEN_END=$(date +%s)
GEN_ELAPSED=$(( GEN_END - GEN_START ))
PASSED=$(cat "$PASS_COUNT")
FAILED=$(( TOTAL - PASSED ))

echo ""
echo "Generation complete in ${GEN_ELAPSED}s — $PASSED passed, $FAILED failed"

if [ -s "$FAIL_LOG" ]; then
    echo ""
    echo "--- Failed scripts ---"
    sort "$FAIL_LOG"
fi

JSONL_COUNT=$(ls "$TMPDIR"/*.jsonl 2>/dev/null | wc -l)
if [ "$JSONL_COUNT" -eq 0 ]; then
    echo "ERROR: No JSONL files produced. Aborting."
    exit 1
fi

# --- Phase 2 & 3: Ingest + Deduplicate ---
echo ""
echo "=== Phase 2: Ingest into SQLite ==="

rm -f "$DB"
export DB_OUT="$DB"

python3 << 'PYEOF'
import sqlite3
import json
import glob
import os
import sys

tmpdir = os.environ["TMPDIR"]
db_path = os.environ.get("DB_OUT", "problems.db")

conn = sqlite3.connect(db_path)
conn.execute("PRAGMA journal_mode=WAL")
conn.execute("PRAGMA synchronous=OFF")

conn.execute("""
CREATE TABLE problems (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    question_latex TEXT NOT NULL,
    answer_key TEXT NOT NULL,
    difficulty INTEGER,
    main_topic TEXT,
    subtopic TEXT,
    grading_mode TEXT,
    answer_type TEXT DEFAULT 'expression',
    calculator_allowed TEXT DEFAULT 'none',
    solution_latex TEXT,
    question_image TEXT,
    time_limit_seconds INTEGER
)
""")

jsonl_files = sorted(glob.glob(os.path.join(tmpdir, "*.jsonl")))
total_files = len(jsonl_files)
total_rows = 0
parse_errors = 0

for i, fpath in enumerate(jsonl_files, 1):
    batch = []
    with open(fpath) as f:
        for line in f:
            line = line.strip()
            if not line or not line.startswith("{"):
                continue
            try:
                p = json.loads(line)
            except json.JSONDecodeError:
                parse_errors += 1
                continue
            batch.append((
                p.get("question_latex", ""),
                p.get("answer_key", ""),
                p.get("difficulty"),
                p.get("main_topic"),
                p.get("subtopic"),
                p.get("grading_mode"),
                p.get("answer_type", "expression"),
                p.get("calculator_allowed", "none"),
                p.get("solution_latex"),
                p.get("question_image"),
                p.get("time_limit_seconds"),
            ))
    if batch:
        conn.executemany(
            "INSERT INTO problems (question_latex, answer_key, difficulty, main_topic, subtopic, "
            "grading_mode, answer_type, calculator_allowed, solution_latex, question_image, time_limit_seconds) "
            "VALUES (?,?,?,?,?,?,?,?,?,?,?)",
            batch,
        )
        conn.commit()
        total_rows += len(batch)
    if i % 50 == 0 or i == total_files:
        print(f"  Ingested {i}/{total_files} files — {total_rows:,} rows so far")

print(f"\nIngested {total_rows:,} problems from {total_files} files ({parse_errors} parse errors)")

# Phase 3: Deduplicate
print("\n=== Phase 3: Deduplicate ===")
before = conn.execute("SELECT COUNT(*) FROM problems").fetchone()[0]
conn.execute("""
DELETE FROM problems WHERE id NOT IN (
    SELECT MIN(id) FROM problems GROUP BY question_latex, answer_key
)
""")
conn.commit()
after = conn.execute("SELECT COUNT(*) FROM problems").fetchone()[0]
removed = before - after
print(f"Removed {removed:,} duplicates — {after:,} unique problems remain")

# Print topic breakdown
print("\n--- Topic breakdown ---")
for row in conn.execute(
    "SELECT main_topic, COUNT(*) as cnt FROM problems GROUP BY main_topic ORDER BY cnt DESC"
):
    print(f"  {row[0] or '(none)':30s} {row[1]:>8,}")

conn.execute("VACUUM")
conn.close()

db_size = os.path.getsize(db_path)
print(f"\nDatabase size: {db_size / (1024*1024):.1f} MB")
PYEOF

echo ""
echo "=== Verification ==="
sqlite3 "$DB" "SELECT COUNT(*) || ' total problems' FROM problems;"
sqlite3 "$DB" "SELECT main_topic, COUNT(*) FROM problems GROUP BY 1 ORDER BY 2 DESC LIMIT 10;"

# --- Phase 4: Upload ---
echo ""
if [ "$DRY_RUN" -eq 1 ]; then
    echo "=== Phase 4: Upload (SKIPPED — dry run) ==="
    echo "Would upload: $DB as tag $TAG to $REPO"
else
    echo "=== Phase 4: Upload to GitHub Releases ==="
    if ! command -v gh &>/dev/null; then
        echo "ERROR: gh CLI not found. Install it or use --dry-run."
        exit 1
    fi

    echo "Creating release $TAG on $REPO..."
    gh release create "$TAG" "$DB" \
        --repo "$REPO" \
        --title "Problems $TAG" \
        --notes "Generated $(date -u +%Y-%m-%dT%H:%M:%SZ) — $(sqlite3 "$DB" 'SELECT COUNT(*) FROM problems') unique problems from $PASSED/$TOTAL scripts"

    echo ""
    echo "Release URL:"
    gh release view "$TAG" --repo "$REPO" --json url -q .url
fi

echo ""
echo "=== Done ==="
