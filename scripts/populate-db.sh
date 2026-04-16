#!/bin/bash
# Generate problems from YAML files → SQLite database
#
# Usage: ./scripts/populate-db.sh [problems_per_file] [output.db] [parallel]
# Default: 4700 per file, 24 parallel, output to factory/exports/problems-v2.db

set -e

PROBLEMS_DIR="problems"
PER_FILE=${1:-4700}
OUTPUT_DB=${2:-"factory/exports/problems-v2.db"}
PARALLEL=${3:-24}
TIMEOUT=120
TMPDIR=$(mktemp -d /tmp/locus-populate.XXXX)
SLOW_LOG="$TMPDIR/slow.txt"
FAIL_LOG="$TMPDIR/fail.txt"
BINARY=$(realpath /media/artur/Storage/cargo-target/release/dsl-cli)

> "$SLOW_LOG"
> "$FAIL_LOG"

echo "Building dsl-cli..."
cargo build --release --bin dsl-cli 2>&1 | tail -1

mapfile -t FILES < <(find "$PROBLEMS_DIR" -name "*.yaml" | sort)
TOTAL=${#FILES[@]}
echo "Files: $TOTAL | Per file: $PER_FILE | Target: $((TOTAL * PER_FILE)) | Parallel: $PARALLEL"
echo "Output: $OUTPUT_DB | Temp: $TMPDIR"
echo ""

DONE=0
FAILED=0

# Process files with controlled parallelism
for i in "${!FILES[@]}"; do
    f="${FILES[$i]}"
    idx=$((i + 1))
    outfile="$TMPDIR/$idx.jsonl"

    (
        start=$(date +%s)
        "$BINARY" generate "$f" --fast -n "$PER_FILE" 2>/dev/null | grep "^{" > "$outfile"
        count=$(wc -l < "$outfile")
        elapsed=$(( $(date +%s) - start ))

        if [ "$count" -eq 0 ]; then
            echo "[$idx/$TOTAL] FAIL $f" >&2
            echo "$f" >> "$FAIL_LOG"
            rm -f "$outfile"
        elif [ "$elapsed" -gt "$TIMEOUT" ]; then
            echo "[$idx/$TOTAL] SLOW $f ($count, ${elapsed}s)" >&2
            echo "$f ${elapsed}s $count" >> "$SLOW_LOG"
        else
            echo "[$idx/$TOTAL] OK $f ($count, ${elapsed}s)" >&2
        fi
    ) &

    # Throttle: wait if we hit parallel limit
    while [ "$(jobs -rp | wc -l)" -ge "$PARALLEL" ]; do
        sleep 0.5
    done
done

# Wait for all remaining
wait

echo ""
echo "=== Merging into SQLite ==="

rm -f "$OUTPUT_DB"
sqlite3 "$OUTPUT_DB" "CREATE TABLE problems (
    id TEXT PRIMARY KEY,
    question_latex TEXT NOT NULL,
    answer_key TEXT NOT NULL,
    difficulty INTEGER NOT NULL,
    main_topic TEXT NOT NULL,
    subtopic TEXT NOT NULL,
    grading_mode TEXT NOT NULL DEFAULT 'equivalent',
    answer_type TEXT NOT NULL DEFAULT 'expression',
    calculator_allowed TEXT NOT NULL DEFAULT 'none',
    solution_latex TEXT NOT NULL DEFAULT '',
    question_image TEXT NOT NULL DEFAULT '',
    time_limit_seconds INTEGER
);"

TOTAL_LINES=$(cat "$TMPDIR"/*.jsonl 2>/dev/null | wc -l)
echo "Inserting $TOTAL_LINES problems..."

cat "$TMPDIR"/*.jsonl 2>/dev/null | python3 -c "
import json, sys, uuid
print('BEGIN;')
for line in sys.stdin:
    try:
        p = json.loads(line)
    except: continue
    uid = str(uuid.uuid4())
    q = p['question_latex'].replace(\"'\", \"''\")
    a = p['answer_key'].replace(\"'\", \"''\")
    s = p.get('solution_latex', '').replace(\"'\", \"''\")
    qi = p.get('question_image', '').replace(\"'\", \"''\")
    t = p.get('time_limit_seconds') or 'NULL'
    print(f\"INSERT INTO problems VALUES ('{uid}','{q}','{a}',{p['difficulty']},'{p['main_topic']}','{p['subtopic']}','{p['grading_mode']}','{p['answer_type']}','{p['calculator_allowed']}','{s}','{qi}',{t});\")
print('COMMIT;')
" | sqlite3 "$OUTPUT_DB"

rm -rf "$TMPDIR"

echo ""
echo "=== Results ==="
sqlite3 "$OUTPUT_DB" "SELECT COUNT(*) || ' problems' FROM problems;"
sqlite3 "$OUTPUT_DB" "SELECT COUNT(DISTINCT main_topic || '/' || subtopic) || ' topics' FROM problems;"
echo "Size: $(du -h "$OUTPUT_DB" | cut -f1)"

echo ""
echo "Distribution:"
sqlite3 "$OUTPUT_DB" "SELECT main_topic, COUNT(*) FROM problems GROUP BY main_topic ORDER BY 2 DESC;"

if [ -s "$SLOW_LOG" ]; then
    echo ""
    echo "=== Slow files (>${TIMEOUT}s) ==="
    sort -t' ' -k2 -rn "$SLOW_LOG"
fi

FAIL_COUNT=$(wc -l < "$FAIL_LOG")
if [ "$FAIL_COUNT" -gt 0 ]; then
    echo ""
    echo "=== Failed files ($FAIL_COUNT) ==="
    cat "$FAIL_LOG"
fi
