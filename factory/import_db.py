#!/usr/bin/env python3
"""
import_db.py — Import a SQLite problems .db file directly into Postgres.
Skips duplicates (by question_latex + answer_key). Uses batch inserts for speed.

Usage:
  python3 import_db.py problems-v1.db
  python3 import_db.py problems-v1.db --url postgres://locus:pass@localhost:5433/locus
  python3 import_db.py problems-v1.db --dry-run
"""

import argparse
import sqlite3
import sys
from pathlib import Path

import psycopg2
import psycopg2.extras

DEFAULT_URL = "postgres://locus:locus_dev_password@localhost:5433/locus"
BATCH_SIZE = 2000

COLS = [
    "question_latex", "answer_key", "difficulty", "main_topic", "subtopic",
    "grading_mode", "answer_type", "calculator_allowed", "solution_latex",
    "question_image", "time_limit_seconds",
]


def main():
    parser = argparse.ArgumentParser(
        description="Import SQLite problems DB into Postgres"
    )
    parser.add_argument("db", help="Path to .db file")
    parser.add_argument("--url", default=DEFAULT_URL, help="Postgres connection URL")
    parser.add_argument("--dry-run", action="store_true", help="Count rows without inserting")
    args = parser.parse_args()

    db_path = Path(args.db)
    if not db_path.exists():
        print(f"[ERROR] Not found: {db_path}")
        sys.exit(1)

    src = sqlite3.connect(db_path)
    src.row_factory = sqlite3.Row
    total_src = src.execute("SELECT COUNT(*) FROM problems").fetchone()[0]
    print(f"[*] Source: {db_path.name} — {total_src:,} problems")

    if args.dry_run:
        print(f"[DRY RUN] Would import up to {total_src:,} rows into {args.url}")
        src.close()
        return

    dst = psycopg2.connect(args.url)
    cur = dst.cursor()

    before = cur.execute("SELECT COUNT(*) FROM problems") or cur.fetchone()[0]
    cur.execute("SELECT COUNT(*) FROM problems")
    before = cur.fetchone()[0]
    print(f"[*] Destination: {before:,} problems currently in DB")

    # Bulk insert into temp table, then merge to avoid needing a unique constraint
    cur.execute(f"""
        CREATE TEMP TABLE _import ({', '.join(f'{c} TEXT' for c in COLS)})
        ON COMMIT DROP
    """)

    inserted_to_temp = 0
    batch_num = 0
    rows_iter = src.execute(f"SELECT {', '.join(COLS)} FROM problems")

    while True:
        batch = rows_iter.fetchmany(BATCH_SIZE)
        if not batch:
            break
        batch_num += 1
        values = [tuple(str(v) if v is not None else None for v in row) for row in batch]
        psycopg2.extras.execute_values(
            cur, f"INSERT INTO _import ({', '.join(COLS)}) VALUES %s",
            values, page_size=BATCH_SIZE,
        )
        inserted_to_temp += len(batch)
        print(f"  Loaded batch {batch_num}: {inserted_to_temp:,} rows in temp table")

    print(f"[*] Merging into problems (skipping duplicates)...")
    # Build SELECT with proper type casting for integer columns only
    select_cols = []
    for col in COLS:
        if col == 'difficulty':
            select_cols.append(f"CAST(i.{col} AS INTEGER) AS {col}")
        elif col == 'time_limit_seconds':
            # Handle empty strings as NULL, otherwise cast to integer
            select_cols.append(f"CAST(NULLIF(i.{col}, '') AS INTEGER) AS {col}")
        else:
            # All other columns are VARCHAR/TEXT, keep as is
            select_cols.append(f"i.{col}")
    
    cur.execute(f"""
        INSERT INTO problems ({', '.join(COLS)})
        SELECT {', '.join(select_cols)} FROM _import i
        WHERE NOT EXISTS (
            SELECT 1 FROM problems p
            WHERE p.question_latex = i.question_latex
              AND p.answer_key = i.answer_key
        )
    """)
    inserted = cur.rowcount
    skipped = inserted_to_temp - inserted

    dst.commit()

    cur.execute("SELECT COUNT(*) FROM problems")
    after = cur.fetchone()[0]

    src.close()
    cur.close()
    dst.close()

    print(f"\n[OK] Done — inserted {inserted:,}, skipped {skipped:,} duplicates")
    print(f"     DB: {before:,} → {after:,} problems")


if __name__ == "__main__":
    main()
