"""Shared utilities for problem generation scripts.

Provides helpers so scripts can focus on math logic instead of JSON boilerplate.
Usage: from problem_utils import *
"""

import json
from random import randint, choice, uniform

from sympy import (
    symbols, Symbol, latex, solve, simplify, expand, factor, cancel,
    diff, integrate, sqrt, cbrt, Abs,
    sin, cos, tan, sec, csc, cot,
    asin, acos, atan, atan2,
    exp, log, ln,
    Rational, Integer, Float, oo, pi, E, I,
    Eq, Ne, Lt, Le, Gt, Ge,
    Matrix, eye, zeros, ones, diag,
    FiniteSet, Interval, Union, Intersection,
    binomial, factorial, gamma,
    limit, Sum, Product, series,
    Piecewise, Heaviside,
    gcd, lcm, isprime, nextprime, factorint,
    Number,
)

# Pre-declared symbols
x, y, z, t = symbols('x y z t')
n, m, k = symbols('n m k', integer=True)
a, b, c, d = symbols('a b c d')

# ---------------------------------------------------------------------------
# Validation constants
# ---------------------------------------------------------------------------

_VALID_ANSWER_TYPES = {
    "expression", "numeric", "set", "tuple", "list",
    "interval", "inequality", "equation", "boolean",
    "word", "matrix", "multi_part",
}
_VALID_GRADING_MODES = {"equivalent", "factor", "expand"}
_VALID_CALCULATOR = {"none", "scientific", "graphing", "cas"}

# ---------------------------------------------------------------------------
# Small helpers
# ---------------------------------------------------------------------------


def nonzero(lo: int, hi: int) -> int:
    """Random integer in [lo, hi] excluding 0."""
    while True:
        v = randint(lo, hi)
        if v != 0:
            return v


def steps(*strings: str) -> str:
    """Join solution steps with <br> for solution_latex."""
    return "<br>".join(strings)

# ---------------------------------------------------------------------------
# Answer formatting helpers
# ---------------------------------------------------------------------------


def fmt_set(*elements) -> str:
    """Format elements as a set string: {1, 2, 3}"""
    try:
        elems = sorted(elements, key=lambda e: float(e))
    except (TypeError, ValueError):
        elems = elements
    return "{" + ", ".join(str(e) for e in elems) + "}"


def fmt_tuple(*elements) -> str:
    """Format as ordered tuple: (3, 5)"""
    return "(" + ", ".join(str(e) for e in elements) + ")"


def fmt_list(*elements) -> str:
    """Format as list: [-2, 2]"""
    return "[" + ", ".join(str(e) for e in elements) + "]"


def fmt_matrix(matrix) -> str:
    """Format a SymPy Matrix or nested list as [[r1], [r2], ...]"""
    if hasattr(matrix, 'tolist'):
        rows = matrix.tolist()
    else:
        rows = matrix
    return "[" + ", ".join(
        "[" + ", ".join(str(e) for e in row) + "]"
        for row in rows
    ) + "]"


def fmt_interval(left, right, left_open=False, right_open=False) -> str:
    """Format interval in DB format: open:1,closed:7"""
    l_type = "open" if left_open else "closed"
    r_type = "open" if right_open else "closed"
    return f"{l_type}:{left},{r_type}:{right}"


def fmt_equation(lhs, rhs) -> str:
    """Format as equation string: lhs = rhs"""
    return f"{lhs} = {rhs}"


def fmt_multipart(*parts: str) -> str:
    """Format multiple answer parts: type:value|||type:value"""
    return "|||".join(parts)

# ---------------------------------------------------------------------------
# Auto-detection and conversion
# ---------------------------------------------------------------------------


def _detect_answer_type(answer) -> str:
    """Auto-detect answer_type from Python/SymPy type."""
    # bool before int (bool is subclass of int)
    if isinstance(answer, bool):
        return "boolean"
    if isinstance(answer, (int, float)):
        return "numeric"
    if isinstance(answer, Number):
        return "numeric"
    # SymPy expression with no free symbols -> numeric (but not Matrix/Set/Eq)
    if hasattr(answer, 'free_symbols') and not answer.free_symbols:
        if not isinstance(answer, (Matrix, FiniteSet, Eq)):
            return "numeric"
    if isinstance(answer, Matrix):
        return "matrix"
    if isinstance(answer, FiniteSet):
        return "set"
    if isinstance(answer, tuple):
        return "tuple"
    if isinstance(answer, list):
        return "list"
    if isinstance(answer, Eq):
        return "equation"
    return "expression"


def _answer_to_str(answer, answer_type: str) -> str:
    """Convert answer to string based on detected type."""
    if isinstance(answer, bool):
        return "true" if answer else "false"
    if isinstance(answer, Matrix):
        return fmt_matrix(answer)
    if isinstance(answer, FiniteSet):
        try:
            elems = sorted(answer, key=lambda e: float(e))
        except (TypeError, ValueError):
            elems = list(answer)
        return "{" + ", ".join(str(e) for e in elems) + "}"
    if isinstance(answer, tuple):
        return fmt_tuple(*answer)
    if isinstance(answer, list):
        return fmt_list(*answer)
    if isinstance(answer, Eq):
        return fmt_equation(answer.lhs, answer.rhs)
    return str(answer)

# ---------------------------------------------------------------------------
# SVG compression
# ---------------------------------------------------------------------------

_SVG_DICT = [
    ("~X", 'xmlns="http://www.w3.org/2000/svg"'),
    ("~D", 'dominant-baseline="central"'),
    ("~s", 'stroke="currentColor"'),
    ("~f", 'fill="currentColor"'),
    ("~n", 'fill="none"'),
    ("~m", 'text-anchor="middle"'),
    ("~e", 'text-anchor="end"'),
    ("~d", 'stroke-dasharray="6,4"'),
    ("~o", 'stroke-opacity="'),
    ("~i", 'font-style="italic"'),
    ("~F", 'fill-opacity="0.15"'),
    ("~w", 'stroke-width="'),
    ("~z", 'font-size="'),
    ("~v", 'viewBox="'),
    ("~M", 'style="max-width:'),
    ("~L", "<line "),
    ("~T", "<text "),
    ("~C", "<circle "),
    ("~P", '<path d="'),
    ("~Q", '<polyline points="'),
    ("~G", '<polygon points="'),
    ("~E", "</text>"),
    ("~g", 'class="g"'),
    ("~a", 'class="a"'),
    ("~t", 'class="t"'),
]


def compress_svg(svg: str) -> str:
    """Replace common SVG substrings with short tokens, prepend 's1:' prefix."""
    out = svg
    for token, expansion in _SVG_DICT:
        out = out.replace(expansion, token)
    return "s1:" + out

# ---------------------------------------------------------------------------
# Main API
# ---------------------------------------------------------------------------


def problem(question, answer, difficulty, topic, solution="", *,
            grading_mode="equivalent", answer_type=None, calculator="none",
            image="", time=None) -> dict:
    """Build a complete problem dict with validation.

    Args:
        question: LaTeX string for the question
        answer: Answer value (auto-converted to string)
        difficulty: int ELO or (lo, hi) tuple for randint
        topic: "main/sub" format string
        solution: Solution LaTeX string
        grading_mode: "equivalent", "factor", or "expand"
        answer_type: Auto-detected from answer if None
        calculator: "none", "scientific", "graphing", or "cas"
        time: Optional expected solve time in seconds (1-3600) for speed bonus
    """
    # Handle difficulty tuple
    if isinstance(difficulty, tuple):
        difficulty = randint(difficulty[0], difficulty[1])

    # Split topic
    parts = topic.split("/", 1)
    if len(parts) != 2:
        raise ValueError(f"topic must be 'main/sub' format, got: {topic!r}")
    main_topic, subtopic = parts

    # Auto-detect answer_type
    if answer_type is None:
        answer_type = _detect_answer_type(answer)

    # Convert answer to string
    answer_key = _answer_to_str(answer, answer_type)

    # Validate
    if grading_mode not in _VALID_GRADING_MODES:
        raise ValueError(f"Invalid grading_mode: {grading_mode!r}")
    if answer_type not in _VALID_ANSWER_TYPES:
        raise ValueError(f"Invalid answer_type: {answer_type!r}")
    if calculator not in _VALID_CALCULATOR:
        raise ValueError(f"Invalid calculator: {calculator!r}")
    if time is not None:
        if not isinstance(time, int):
            raise ValueError(f"time must be an int, got {type(time).__name__}")
        if time < 1 or time > 3600:
            raise ValueError(f"time must be between 1 and 3600, got {time}")

    return {
        "question_latex": question,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": main_topic,
        "subtopic": subtopic,
        "grading_mode": grading_mode,
        "answer_type": answer_type,
        "calculator_allowed": calculator,
        "solution_latex": solution,
        "question_image": compress_svg(image) if image else "",
        "time_limit_seconds": time,
    }


def emit(problem_dict: dict):
    """Print problem as JSON to stdout."""
    print(json.dumps(problem_dict))
