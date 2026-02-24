#!/usr/bin/env python3
"""
publish_db.py — Combine selected SQL export files into a SQLite .db,
deduplicate by (question_latex, answer_key), and publish to GitHub Releases.

Usage:
  python3 publish_db.py exports/problems_A.sql exports/problems_B.sql
  python3 publish_db.py exports/*.sql --tag problems-v2
  python3 publish_db.py exports/*.sql --dry-run
"""

import argparse
import sqlite3
import subprocess
import sys
from datetime import datetime
from pathlib import Path

REPO = "FizzWizZleDazzle/locus-scripts"

SCHEMA = """
CREATE TABLE IF NOT EXISTS problems (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    question_latex      TEXT NOT NULL,
    answer_key          TEXT NOT NULL,
    difficulty          INTEGER,
    main_topic          TEXT,
    subtopic            TEXT,
    grading_mode        TEXT,
    answer_type         TEXT DEFAULT 'expression',
    calculator_allowed  TEXT DEFAULT 'none',
    solution_latex      TEXT,
    question_image      TEXT,
    time_limit_seconds  INTEGER
);
"""

RELEASE_NOTES = """\
A pre-generated SQLite database of math problems produced by the \
[locus-scripts](https://github.com/FizzWizZleDazzle/locus-scripts) generation \
scripts. Each row is a self-contained problem ready to import directly into a \
Locus-compatible database. Problems span a wide range of topics and difficulty \
levels, with full LaTeX question/answer/solution text and optional SVG diagrams.

### Schema

```sql
CREATE TABLE problems (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    question_latex      TEXT,
    answer_key          TEXT,
    difficulty          INTEGER,   -- Elo-style rating (1000–2000)
    main_topic          TEXT,
    subtopic            TEXT,
    grading_mode        TEXT,      -- equivalent | factor | expand
    answer_type         TEXT,      -- expression | numeric | set | ...
    calculator_allowed  TEXT,      -- none | scientific | graphing | cas
    solution_latex      TEXT,
    question_image      TEXT,      -- compressed SVG string (s1:... prefix)
    time_limit_seconds  INTEGER
);
```

### Usage

```bash
# Download
gh release download {tag} --repo {repo} -p '*.db'

# Inspect
sqlite3 {db} "SELECT COUNT(*) FROM problems;"
sqlite3 {db} "SELECT main_topic, COUNT(*) FROM problems GROUP BY 1 ORDER BY 2 DESC;"
```
"""


def import_sql_file(conn: sqlite3.Connection, path: Path) -> int:
    sql = path.read_text()
    lines = [l for l in sql.splitlines() if not l.startswith("--")]
    sql = "\n".join(lines).strip()
    if not sql:
        return 0
    before = conn.execute("SELECT COUNT(*) FROM problems").fetchone()[0]
    conn.executescript(sql)
    after = conn.execute("SELECT COUNT(*) FROM problems").fetchone()[0]
    return after - before


def deduplicate(conn: sqlite3.Connection) -> int:
    before = conn.execute("SELECT COUNT(*) FROM problems").fetchone()[0]
    conn.execute("""
        DELETE FROM problems WHERE id NOT IN (
            SELECT MIN(id) FROM problems
            GROUP BY question_latex, answer_key
        )
    """)
    conn.commit()
    after = conn.execute("SELECT COUNT(*) FROM problems").fetchone()[0]
    return before - after


def publish_release(db_path: Path, tag: str, count: int) -> str:
    notes = RELEASE_NOTES.format(tag=tag, repo=REPO, db=db_path.name)
    notes += f"\n**{count:,} problems**"
    result = subprocess.run(
        ["gh", "release", "create", tag, str(db_path),
         "--repo", REPO,
         "--title", f"Problems DB {tag}",
         "--notes", notes],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip())
    return result.stdout.strip()


def main():
    parser = argparse.ArgumentParser(
        description="Build SQLite DB from SQL exports and publish to GitHub Releases"
    )
    parser.add_argument("sql_files", nargs="+", help="SQL export files to combine")
    parser.add_argument("--tag", default=None, help="Release tag (default: problems-YYYYMMDD_HHMMSS)")
    parser.add_argument("--output", default=None, help="Output .db filename")
    parser.add_argument("--dry-run", action="store_true", help="Build DB but skip GitHub publish")
    args = parser.parse_args()

    sql_paths = [Path(p) for p in args.sql_files]
    missing = [p for p in sql_paths if not p.exists()]
    if missing:
        for p in missing:
            print(f"[ERROR] Not found: {p}")
        sys.exit(1)

    tag = args.tag or f"problems-{datetime.utcnow().strftime('%Y%m%d_%H%M%S')}"
    db_path = Path(args.output) if args.output else Path(f"{tag}.db")

    print(f"[*] Building {db_path} from {len(sql_paths)} file(s)...")
    conn = sqlite3.connect(db_path)
    conn.executescript(SCHEMA)
    conn.commit()

    total_imported = 0
    for p in sql_paths:
        n = import_sql_file(conn, p)
        print(f"    {p.name}: +{n:,} rows")
        total_imported += n
    print(f"    Total imported: {total_imported:,}")

    print(f"[*] Deduplicating...")
    removed = deduplicate(conn)
    final = conn.execute("SELECT COUNT(*) FROM problems").fetchone()[0]
    print(f"    Removed {removed:,} duplicates → {final:,} problems")
    conn.close()

    if args.dry_run:
        print(f"[DRY RUN] DB written to {db_path} ({db_path.stat().st_size / 1e6:.1f} MB)")
        print(f"[DRY RUN] Would publish as release '{tag}' to {REPO}")
        return

    print(f"[*] Publishing release '{tag}' to {REPO}...")
    try:
        url = publish_release(db_path, tag, final)
        print(f"[OK] {url}")
    except RuntimeError as e:
        print(f"[ERROR] GitHub release failed: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
