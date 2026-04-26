"""Dump daily_puzzles + problem fields to single-table sqlite."""
import sqlite3
from pathlib import Path

import psycopg2

DSN = "host=192.168.1.104 port=5432 user=locus_srv password=locus7878 dbname=locus"
OUT = Path("/home/artur/Locus/scripts/daily_puzzles/locus_daily_puzzles.db")

if OUT.exists():
    OUT.unlink()

pg = psycopg2.connect(DSN)
pg_cur = pg.cursor()
pg_cur.execute("""
    SELECT
        d.puzzle_date,
        d.title,
        d.source,
        d.status,
        p.question_latex,
        p.answer_key,
        p.difficulty,
        p.main_topic,
        p.subtopic,
        p.answer_type,
        p.grading_mode,
        p.calculator_allowed,
        p.solution_latex,
        d.editorial_latex,
        d.hints::text AS hints
    FROM daily_puzzles d
    JOIN problems p ON p.id = d.problem_id
    WHERE d.puzzle_date IS NOT NULL
    ORDER BY d.puzzle_date
""")
rows = pg_cur.fetchall()
print(f"Fetched {len(rows)} puzzles from PG")

sl = sqlite3.connect(OUT)
sl.execute("""
    CREATE TABLE daily_puzzles (
        puzzle_date          TEXT PRIMARY KEY,
        title                TEXT NOT NULL,
        source               TEXT NOT NULL,
        status               TEXT NOT NULL,
        question_latex       TEXT NOT NULL,
        answer_key           TEXT NOT NULL,
        difficulty           INTEGER NOT NULL,
        main_topic           TEXT NOT NULL,
        subtopic             TEXT NOT NULL,
        answer_type          TEXT NOT NULL,
        grading_mode         TEXT NOT NULL,
        calculator_allowed   TEXT NOT NULL,
        solution_latex       TEXT NOT NULL,
        editorial_latex      TEXT NOT NULL,
        hints_json           TEXT NOT NULL
    )
""")
sl.execute("CREATE INDEX idx_dp_date ON daily_puzzles(puzzle_date)")
sl.execute("CREATE INDEX idx_dp_topic ON daily_puzzles(main_topic, subtopic)")
sl.execute("CREATE INDEX idx_dp_difficulty ON daily_puzzles(difficulty)")

sl.executemany("""
    INSERT INTO daily_puzzles (
        puzzle_date, title, source, status,
        question_latex, answer_key, difficulty,
        main_topic, subtopic, answer_type, grading_mode, calculator_allowed,
        solution_latex, editorial_latex, hints_json
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
""", [
    (
        r[0].isoformat(), r[1], r[2], r[3],
        r[4], r[5], r[6],
        r[7], r[8], r[9], r[10], r[11],
        r[12] or "", r[13] or "", r[14] or "[]",
    )
    for r in rows
])
sl.commit()
sl.close()
pg_cur.close()
pg.close()
print(f"Wrote {OUT} ({OUT.stat().st_size} bytes)")
