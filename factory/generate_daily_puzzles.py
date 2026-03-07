#!/usr/bin/env python3
"""Generate olympiad-level daily puzzles using LLM.

Inserts into both `problems` and `daily_puzzles` tables with status='draft'.
A reviewer then approves and schedules dates via the factory UI or direct SQL.

Usage:
    python generate_daily_puzzles.py --count 10 --topic number_theory
    python generate_daily_puzzles.py --count 5 --dry-run
"""

import argparse
import asyncio
import json
import os
import sys
import uuid
from pathlib import Path

import asyncpg
import httpx
from dotenv import load_dotenv

# Load .env from factory directory
load_dotenv(Path(__file__).parent / ".env")

LLM_ENDPOINT = os.getenv("LLM_ENDPOINT")
LLM_API_KEY = os.getenv("LLM_API_KEY")
LLM_MODEL = os.getenv("LLM_MODEL", "gpt-4")
DATABASE_URL = os.getenv(
    "DATABASE_URL", "postgres://locus:locus_dev_password@localhost:5433/locus"
)

TOPICS = [
    "number_theory",
    "combinatorics",
    "algebra",
    "geometry",
    "analysis",
]

SYSTEM_PROMPT = """You are an expert competition mathematics problem author. Generate olympiad-level math problems suitable for a daily puzzle feature on a competitive math platform.

CRITICAL REQUIREMENTS:
1. The answer MUST be a concrete, auto-gradeable value: a number, algebraic expression, set, tuple, or list. NO PROOFS.
2. The problem must be solvable by a skilled student in 10-30 minutes.
3. The difficulty should be at the AMC 10/12, AIME, or early Putnam level (ELO 2500-4000).
4. Include 3 progressive hints (from gentle nudge to strong hint).
5. Include a detailed editorial solution with clear steps.
6. The question_latex must be valid LaTeX.
7. The answer_key must be a machine-parseable math expression.

Output a JSON object with exactly these fields:
{
  "title": "Short descriptive title",
  "question_latex": "Full problem statement in LaTeX",
  "answer_key": "Exact answer (number, expression, set like {1,2,3}, tuple like (3,5))",
  "answer_type": "numeric|expression|set|tuple|list",
  "difficulty": 2500-4000,
  "main_topic": "one of: arithmetic, algebra1, algebra2, geometry, precalculus, calculus, linear_algebra",
  "subtopic": "specific subtopic from the platform",
  "grading_mode": "equivalent",
  "solution_latex": "Step-by-step solution in LaTeX",
  "hints": ["Hint 1 (gentle)", "Hint 2 (moderate)", "Hint 3 (strong)"],
  "source": "Original" or "Inspired by [competition] [year] #[number]"
}

Output ONLY valid JSON, no markdown fences or extra text."""

TOPIC_PROMPTS = {
    "number_theory": "Generate a number theory problem involving concepts like modular arithmetic, Euler's theorem, Diophantine equations, prime factorization, or number-theoretic functions. The answer should be a specific integer or expression.",
    "combinatorics": "Generate a combinatorics problem involving counting, probability, generating functions, recurrences, or graph theory. The answer should be a specific integer or simplified fraction.",
    "algebra": "Generate an algebra problem involving polynomials, inequalities, functional equations, or sequences/series. The answer should be a specific number, expression, or set of values.",
    "geometry": "Generate a geometry problem involving triangles, circles, areas, lengths, or coordinate geometry. The answer should be a specific numeric value or expression. Avoid problems requiring diagrams.",
    "analysis": "Generate a calculus/analysis problem involving limits, integrals, series, or optimization. The answer should be a specific numeric value or closed-form expression.",
}


async def call_llm(topic: str) -> dict | None:
    """Call the LLM to generate a single daily puzzle."""
    topic_prompt = TOPIC_PROMPTS.get(topic, TOPIC_PROMPTS["algebra"])
    user_prompt = f"{topic_prompt}\n\nRemember: the answer must be a concrete gradeable value, not a proof. Output only valid JSON."

    is_anthropic = LLM_ENDPOINT and "anthropic.com" in LLM_ENDPOINT

    if is_anthropic:
        headers = {
            "x-api-key": LLM_API_KEY,
            "anthropic-version": "2023-06-01",
            "Content-Type": "application/json",
        }
        payload = {
            "model": LLM_MODEL,
            "system": SYSTEM_PROMPT,
            "messages": [{"role": "user", "content": user_prompt}],
            "max_tokens": 4096,
        }
    else:
        headers = {
            "Authorization": f"Bearer {LLM_API_KEY}",
            "Content-Type": "application/json",
        }
        payload = {
            "model": LLM_MODEL,
            "messages": [
                {"role": "system", "content": SYSTEM_PROMPT},
                {"role": "user", "content": user_prompt},
            ],
            "max_tokens": 4096,
            "temperature": 0.8,
        }

    async with httpx.AsyncClient(timeout=120) as client:
        resp = await client.post(LLM_ENDPOINT, headers=headers, json=payload)
        resp.raise_for_status()
        data = resp.json()

    # Extract text from response
    if is_anthropic:
        text = data["content"][0]["text"]
    else:
        text = data["choices"][0]["message"]["content"]

    # Strip markdown fences if present
    text = text.strip()
    if text.startswith("```"):
        text = text.split("\n", 1)[1]
        if text.endswith("```"):
            text = text[:-3]
        text = text.strip()

    try:
        return json.loads(text)
    except json.JSONDecodeError as e:
        print(f"  Failed to parse LLM response as JSON: {e}", file=sys.stderr)
        print(f"  Raw: {text[:200]}...", file=sys.stderr)
        return None


def validate_puzzle(data: dict) -> list[str]:
    """Validate that generated puzzle data has required fields."""
    errors = []
    required = [
        "title",
        "question_latex",
        "answer_key",
        "answer_type",
        "difficulty",
        "main_topic",
        "solution_latex",
        "hints",
    ]
    for field in required:
        if field not in data:
            errors.append(f"Missing field: {field}")

    if "answer_type" in data:
        valid_types = {
            "expression",
            "numeric",
            "set",
            "tuple",
            "list",
            "interval",
            "inequality",
            "equation",
            "boolean",
            "word",
            "matrix",
            "multi_part",
        }
        if data["answer_type"] not in valid_types:
            errors.append(f"Invalid answer_type: {data['answer_type']}")

    if "difficulty" in data:
        d = data["difficulty"]
        if not isinstance(d, int) or d < 100 or d > 5000:
            errors.append(f"Difficulty out of range: {d}")

    if "hints" in data:
        if not isinstance(data["hints"], list) or len(data["hints"]) < 1:
            errors.append("hints must be a non-empty list")

    return errors


async def insert_puzzle(pool: asyncpg.Pool, data: dict) -> str:
    """Insert a puzzle into problems + daily_puzzles tables. Returns daily_puzzle id."""
    async with pool.acquire() as conn:
        async with conn.transaction():
            # Insert into problems table
            problem_id = await conn.fetchval(
                """
                INSERT INTO problems (
                    question_latex, answer_key, difficulty, main_topic, subtopic,
                    grading_mode, answer_type, calculator_allowed, solution_latex,
                    question_image, time_limit_seconds
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING id
                """,
                data["question_latex"],
                str(data["answer_key"]),
                data["difficulty"],
                data.get("main_topic", "algebra2"),
                data.get("subtopic", ""),
                data.get("grading_mode", "equivalent"),
                data.get("answer_type", "numeric"),
                data.get("calculator_allowed", "none"),
                data.get("solution_latex", ""),
                "",  # question_image
                None,  # time_limit_seconds
            )

            # Insert into daily_puzzles table (draft, no date)
            dp_id = await conn.fetchval(
                """
                INSERT INTO daily_puzzles (
                    problem_id, title, hints, editorial_latex, source, status
                ) VALUES ($1, $2, $3::jsonb, $4, $5, 'draft')
                RETURNING id
                """,
                problem_id,
                data.get("title", ""),
                json.dumps(data.get("hints", [])),
                data.get("solution_latex", ""),
                data.get("source", "Original"),
            )

    return str(dp_id)


async def main():
    parser = argparse.ArgumentParser(description="Generate daily puzzles via LLM")
    parser.add_argument("--count", type=int, default=5, help="Number of puzzles to generate")
    parser.add_argument(
        "--topic",
        choices=TOPICS,
        default=None,
        help="Topic category (random if not specified)",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Generate and validate but don't insert into DB",
    )
    args = parser.parse_args()

    if not LLM_ENDPOINT or not LLM_API_KEY:
        print("Error: LLM_ENDPOINT and LLM_API_KEY must be set in .env", file=sys.stderr)
        sys.exit(1)

    import random

    pool = None
    if not args.dry_run:
        pool = await asyncpg.create_pool(DATABASE_URL, min_size=1, max_size=3)

    generated = 0
    errors = 0

    for i in range(args.count):
        topic = args.topic or random.choice(TOPICS)
        print(f"[{i + 1}/{args.count}] Generating {topic} puzzle...")

        try:
            data = await call_llm(topic)
            if data is None:
                errors += 1
                continue

            validation_errors = validate_puzzle(data)
            if validation_errors:
                print(f"  Validation errors: {validation_errors}", file=sys.stderr)
                errors += 1
                continue

            if args.dry_run:
                print(f"  OK: {data['title']}")
                print(f"  Answer: {data['answer_key']} ({data['answer_type']})")
                print(f"  Difficulty: {data['difficulty']}")
                print(f"  Hints: {len(data.get('hints', []))}")
                print(json.dumps(data, indent=2))
            else:
                dp_id = await insert_puzzle(pool, data)
                print(f"  Inserted: {data['title']} (id={dp_id})")

            generated += 1

        except Exception as e:
            print(f"  Error: {e}", file=sys.stderr)
            errors += 1

    if pool:
        await pool.close()

    print(f"\nDone: {generated} generated, {errors} errors")


if __name__ == "__main__":
    asyncio.run(main())
