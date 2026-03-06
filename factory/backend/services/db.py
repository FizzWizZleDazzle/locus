"""Direct Postgres access for inserting problems"""

import asyncpg
from typing import List, Dict, Any, Optional


_pool: Optional[asyncpg.Pool] = None

_INSERT_SQL = """
INSERT INTO problems (
    question_latex, answer_key, difficulty, main_topic, subtopic,
    grading_mode, answer_type, calculator_allowed, solution_latex,
    question_image, time_limit_seconds
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
RETURNING id
"""


async def init_pool(database_url: str):
    """Create the connection pool. Call once at startup."""
    global _pool
    _pool = await asyncpg.create_pool(database_url, min_size=2, max_size=10)


async def close_pool():
    """Close the connection pool. Call at shutdown."""
    global _pool
    if _pool:
        await _pool.close()
        _pool = None


def get_pool() -> asyncpg.Pool:
    if _pool is None:
        raise RuntimeError("Database pool not initialized — call init_pool() first")
    return _pool


async def insert_problems(problems: List[Dict[str, Any]]) -> List[str]:
    """Insert problems directly into Postgres. Returns list of new UUIDs."""
    pool = get_pool()
    ids = []
    async with pool.acquire() as conn:
        for p in problems:
            row = await conn.fetchrow(
                _INSERT_SQL,
                p["question_latex"],
                p["answer_key"],
                p["difficulty"],
                p["main_topic"],
                p["subtopic"],
                p["grading_mode"],
                p.get("answer_type", "expression"),
                p.get("calculator_allowed", "none"),
                p.get("solution_latex", ""),
                p.get("question_image", ""),
                p.get("time_limit_seconds"),
            )
            ids.append(str(row["id"]))
    return ids
