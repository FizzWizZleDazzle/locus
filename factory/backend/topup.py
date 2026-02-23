#!/usr/bin/env python3
"""
topup.py — Re-run only scripts for subtopics below a target count.
Reads an existing SQLite DB to find thin subtopics, runs those scripts,
and writes a new SQL export file.

Usage:
  python3 topup.py /tmp/locus_check.db --target 1500 --output exports/topup.sql
"""

import argparse
import sqlite3
import sys
from concurrent.futures import ProcessPoolExecutor, as_completed
from datetime import datetime
from pathlib import Path

# Reuse helpers from script_runner
sys.path.insert(0, str(Path(__file__).parent))
from services.script_runner import _run_script_n_times, _SQL_COLS, _problem_to_sql_row

SCRIPTS_DIR = Path(__file__).parent / "scripts" / "src"


def get_thin_subtopics(db_path: Path, target: int):
    """Return {(main_topic, subtopic): current_count} for subtopics below target."""
    conn = sqlite3.connect(db_path)
    rows = conn.execute("""
        SELECT main_topic, subtopic, COUNT(*) as count
        FROM problems
        GROUP BY main_topic, subtopic
        HAVING count < ?
        ORDER BY count
    """, (target,)).fetchall()
    conn.close()
    return {(r[0], r[1]): r[2] for r in rows}


def find_scripts(topic: str, subtopic: str) -> list[Path]:
    """Find all difficulty scripts for a given topic/subtopic."""
    pattern = f"{topic}_{subtopic}_*.py"
    return list(SCRIPTS_DIR.glob(pattern))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("db", help="SQLite DB to check current counts against")
    parser.add_argument("--target", type=int, default=1500)
    parser.add_argument("--output", default=None)
    parser.add_argument("--workers", type=int, default=8)
    args = parser.parse_args()

    db_path = Path(args.db)
    if not db_path.exists():
        print(f"[ERROR] DB not found: {db_path}")
        sys.exit(1)

    thin = get_thin_subtopics(db_path, args.target)
    print(f"[*] Found {len(thin)} subtopics below {args.target}")

    # Build task list: (script_path, scripts_dir, run_count)
    tasks = []
    for (topic, subtopic), current in thin.items():
        scripts = find_scripts(topic, subtopic)
        if not scripts:
            print(f"    [WARN] No scripts found for {topic}/{subtopic}")
            continue
        deficit = args.target - current
        # 2× buffer for failure rates, min 200
        runs = max(200, (deficit // len(scripts)) * 2)
        for sp in scripts:
            tasks.append((str(sp), str(SCRIPTS_DIR), runs))

    print(f"[*] Running {len(tasks)} scripts (across {len(thin)} subtopics)...")

    timestamp = datetime.utcnow().strftime('%Y%m%d_%H%M%S')
    output_path = Path(args.output) if args.output else \
        Path(__file__).parent / "exports" / f"topup_{timestamp}.sql"

    total = 0
    errors = []
    first_row = True

    with open(output_path, 'w') as f:
        f.write(f"-- Locus Factory top-up export\n")
        f.write(f"-- Generated at: {datetime.utcnow().isoformat()}\n")
        f.write(f"-- Target: {args.target} per subtopic\n\n")
        f.write(f"INSERT INTO problems ({_SQL_COLS}) VALUES\n")

        with ProcessPoolExecutor(max_workers=args.workers) as pool:
            futures = {pool.submit(_run_script_n_times, t): t[0] for t in tasks}
            for future in as_completed(futures):
                script_name, problems, errs = future.result()
                for p in problems:
                    if not first_row:
                        f.write(",\n")
                    f.write(_problem_to_sql_row(p))
                    first_row = False
                total += len(problems)
                errors.extend(errs)
                if problems:
                    print(f"    {Path(script_name).stem}: +{len(problems)}")
            f.flush()

        f.write(";\n" if total > 0 else "-- No problems generated\n")

    print(f"\n[OK] {total:,} problems written to {output_path}")
    if errors:
        print(f"[WARN] {len(errors)} errors (first few):")
        for e in errors[:5]:
            print(f"    {e}")


if __name__ == "__main__":
    main()
