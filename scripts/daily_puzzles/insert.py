"""Pick 365 puzzles from fetched AIME, insert into PG (problems + daily_puzzles)."""
import json
import os
import random
from datetime import date, timedelta
from pathlib import Path

import psycopg2
import psycopg2.extras

DSN = "host=192.168.1.104 port=5432 user=locus_srv password=locus7878 dbname=locus"
SRC = Path("/home/artur/Locus/scripts/daily_puzzles/aime_fetched.json")
START_DATE = date(2026, 4, 26)
N_DAYS = 365


def better_title(p: dict) -> str:
    """Generate a 3-5 word title from question text."""
    q = p["question_latex"]
    # Remove LaTeX dollars and common stopwords for a friendlier title
    text = q.replace("$", "").strip()
    # Try to find a noun-ish anchor: look for capitalized words first
    words = [w.strip(".,;:?!()") for w in text.split()]
    skip = {"the", "a", "an", "is", "are", "was", "were", "be", "been", "of", "to", "in", "on", "for", "with", "and", "or", "by", "as", "at", "from", "that", "which", "this", "these", "those", "let", "find", "given", "suppose", "consider", "what", "how"}
    # Pick first 4 substantial words
    picked = []
    for w in words:
        if not w:
            continue
        if w.lower() in skip:
            continue
        if w.isdigit():
            continue
        picked.append(w)
        if len(picked) >= 4:
            break
    title = " ".join(picked) if picked else "AIME Puzzle"
    # cap length
    if len(title) > 60:
        title = title[:57] + "..."
    return title


def pick_365(all_puzzles: list[dict]) -> list[dict]:
    """Pick 365 puzzles with good variety: balanced across years and difficulty."""
    random.seed(42)
    # Sort by (year, contest, number)
    by_year = {}
    for p in all_puzzles:
        by_year.setdefault(p["year"], []).append(p)
    picked = []
    # Take ~25 per year (15 years × 25 ≈ 375), then trim to 365
    per_year_target = 25
    for year, plist in by_year.items():
        random.shuffle(plist)
        picked.extend(plist[:per_year_target])
    random.shuffle(picked)
    return picked[:N_DAYS]


def main():
    data = json.load(open(SRC))
    print(f"Loaded {len(data)} fetched puzzles")
    chosen = pick_365(data)
    print(f"Chose {len(chosen)} for scheduling")
    assert len(chosen) == N_DAYS, f"need {N_DAYS}, got {len(chosen)}"

    # Generate dates 2026-04-26 .. 2027-04-25
    dates = [START_DATE + timedelta(days=i) for i in range(N_DAYS)]

    conn = psycopg2.connect(DSN)
    conn.autocommit = False
    cur = conn.cursor()

    # Pre-check: any of these dates already taken?
    cur.execute("SELECT puzzle_date FROM daily_puzzles WHERE puzzle_date = ANY(%s)", (dates,))
    taken = {r[0] for r in cur.fetchall()}
    if taken:
        print(f"WARN: {len(taken)} dates already have puzzles, e.g. {list(taken)[:5]}")
        return

    inserted = 0
    for d, p in zip(dates, chosen):
        # Insert problem
        cur.execute("""
            INSERT INTO problems (
                question_latex, answer_key, difficulty, grading_mode,
                main_topic, subtopic, calculator_allowed, answer_type,
                solution_latex
            ) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s) RETURNING id
        """, (
            p["question_latex"], p["answer_key"], p["difficulty"], p["grading_mode"],
            p["main_topic"], p["subtopic"], p["calculator_allowed"], p["answer_type"],
            p.get("editorial_latex", ""),
        ))
        problem_id = cur.fetchone()[0]
        # Insert daily_puzzle
        cur.execute("""
            INSERT INTO daily_puzzles (
                problem_id, puzzle_date, title, hints, editorial_latex, source, status
            ) VALUES (%s, %s, %s, %s::jsonb, %s, %s, 'scheduled')
        """, (
            problem_id, d, better_title(p), json.dumps(p.get("hints", [])),
            p.get("editorial_latex", ""), p.get("source", ""),
        ))
        inserted += 1
        if inserted % 50 == 0:
            print(f"  {inserted}/{N_DAYS}")

    conn.commit()
    cur.close()
    conn.close()
    print(f"Inserted {inserted} daily puzzles")


if __name__ == "__main__":
    main()
