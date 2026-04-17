#!/usr/bin/env python3
"""Bulk import JSONL problems into PostgreSQL using COPY protocol."""

import csv
import io
import json
import sys
import subprocess

DB_URL = "postgresql://locus_srv:locus7878@192.168.1.104:5432/locus"
BATCH_SIZE = 10000

COLUMNS = [
    "question_latex", "answer_key", "solution_latex", "difficulty",
    "main_topic", "subtopic", "grading_mode", "answer_type",
    "calculator_allowed", "question_image", "time_limit_seconds",
]

# Map non-standard answer_types to valid DB enum values
ANSWER_TYPE_MAP = {
    "coordinate": "tuple",
    "coordinate_pair": "tuple",
    "integer": "numeric",
    "domain_range": "interval",
    "compound_inequality": "inequality",
    "boundary_value": "numeric",
    "value": "numeric",
    "complex": "expression",
    "multiple": "multi_part",
    "number": "numeric",
}

def main():
    if len(sys.argv) < 2:
        print("Usage: import_jsonl.py <file.jsonl>", file=sys.stderr)
        sys.exit(1)

    path = sys.argv[1]
    total = 0
    errors = 0

    # Use psql COPY FROM STDIN for maximum throughput
    proc = subprocess.Popen(
        ["psql", DB_URL, "-c",
         "COPY problems (question_latex, answer_key, solution_latex, difficulty, "
         "main_topic, subtopic, grading_mode, answer_type, calculator_allowed, "
         "question_image, time_limit_seconds) FROM STDIN WITH (FORMAT csv, NULL '\\N')"],
        stdin=subprocess.PIPE,
        text=True,
        bufsize=1 << 20,
    )

    with open(path, "r") as f:
        writer = csv.writer(proc.stdin)
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                obj = json.loads(line)
                row = [
                    obj["question_latex"],
                    obj["answer_key"],
                    obj.get("solution_latex", ""),
                    obj["difficulty"],
                    obj["main_topic"],
                    obj["subtopic"],
                    obj.get("grading_mode", "equivalent"),
                    ANSWER_TYPE_MAP.get(obj.get("answer_type", "expression"), obj.get("answer_type", "expression")),
                    obj.get("calculator_allowed", "none"),
                    obj.get("question_image", ""),
                    obj.get("time_limit_seconds") or "\\N",
                ]
                writer.writerow(row)
                total += 1
            except (json.JSONDecodeError, KeyError) as e:
                errors += 1
                if errors <= 5:
                    print(f"Error line {total + errors}: {e}", file=sys.stderr)

            if total % 100000 == 0 and total > 0:
                print(f"  {total:,} rows sent...", file=sys.stderr)

    proc.stdin.close()
    rc = proc.wait()

    if rc == 0:
        print(f"Done: {total:,} rows imported, {errors} errors", file=sys.stderr)
    else:
        print(f"psql exited with code {rc}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
