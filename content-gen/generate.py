#!/usr/bin/env python3
"""
Problem Generator for Locus

Generates math problems using SymPy and outputs them in a format
suitable for importing into PostgreSQL.

Usage:
    python generate.py --topic algebra --count 50 --output problems.sql
"""

import argparse
import random
from sympy import (
    symbols, expand, factor, diff, simplify, latex,
    sin, cos, tan, exp, log, sqrt, Rational,
    Matrix, det, trace
)
from sympy.parsing.sympy_parser import parse_expr

x, y, z, a, b, c, t = symbols('x y z a b c t')


def generate_algebra_problem(difficulty: int) -> tuple[str, str, str]:
    """Generate an algebra problem."""
    if difficulty < 1400:
        # Simple combining like terms
        coef1, coef2 = random.randint(1, 5), random.randint(1, 5)
        expr = coef1 * x + coef2 * x
        question = f"Simplify: ${coef1}x + {coef2}x$"
        answer = str(simplify(expr))
        return question, answer, "equivalent"

    elif difficulty < 1500:
        # Expanding simple binomials
        a_val, b_val = random.randint(1, 5), random.randint(-5, 5)
        c_val, d_val = random.randint(1, 5), random.randint(-5, 5)
        expr = (a_val * x + b_val) * (c_val * x + d_val)
        expanded = expand(expr)

        # Format for display
        term1 = f"{a_val}x" if a_val != 1 else "x"
        sign1 = "+" if b_val >= 0 else "-"
        term2 = f"{c_val}x" if c_val != 1 else "x"
        sign2 = "+" if d_val >= 0 else "-"

        question = f"Expand: $({term1} {sign1} {abs(b_val)})({term2} {sign2} {abs(d_val)})$"
        answer = str(expanded).replace("**", "^").replace(" ", "")
        return question, answer, "equivalent"

    elif difficulty < 1600:
        # Factoring quadratics
        r1, r2 = random.randint(-5, 5), random.randint(-5, 5)
        expr = expand((x - r1) * (x - r2))
        factored = factor(expr)

        question = f"Factor: ${latex(expr)}$"
        answer = str(factored).replace("**", "^").replace(" ", "")
        return question, answer, "factor"

    else:
        # More complex factoring
        a_val = random.choice([2, 3])
        r1, r2 = random.randint(-3, 3), random.randint(-3, 3)
        expr = expand(a_val * (x - r1) * (x - r2))
        factored = factor(expr)

        question = f"Factor: ${latex(expr)}$"
        answer = str(factored).replace("**", "^").replace(" ", "")
        return question, answer, "factor"


def generate_calculus_problem(difficulty: int) -> tuple[str, str, str]:
    """Generate a calculus (derivatives) problem."""
    if difficulty < 1450:
        # Simple power rule
        n = random.randint(2, 5)
        coef = random.randint(1, 5)
        expr = coef * x**n
        derivative = diff(expr, x)

        question = f"Find $\\frac{{d}}{{dx}}[{coef}x^{n}]$"
        answer = str(derivative).replace("**", "^").replace(" ", "")
        return question, answer, "equivalent"

    elif difficulty < 1550:
        # Sum of terms
        terms = []
        for _ in range(random.randint(2, 3)):
            n = random.randint(1, 4)
            coef = random.randint(1, 5)
            terms.append(coef * x**n)
        expr = sum(terms)
        derivative = diff(expr, x)

        question = f"Find $\\frac{{d}}{{dx}}[{latex(expr)}]$"
        answer = str(derivative).replace("**", "^").replace(" ", "")
        return question, answer, "equivalent"

    elif difficulty < 1650:
        # Trig functions
        func = random.choice([sin(x), cos(x), sin(2*x), cos(3*x)])
        derivative = diff(func, x)

        question = f"Find $\\frac{{d}}{{dx}}[{latex(func)}]$"
        answer = str(derivative).replace("**", "^").replace(" ", "")
        return question, answer, "equivalent"

    else:
        # Product rule or chain rule
        choice = random.choice(["product", "chain"])
        if choice == "product":
            expr = x * exp(x)
            derivative = diff(expr, x)
            question = f"Find $\\frac{{d}}{{dx}}[x \\cdot e^x]$"
        else:
            n = random.randint(2, 3)
            expr = exp(n * x)
            derivative = diff(expr, x)
            question = f"Find $\\frac{{d}}{{dx}}[e^{{{n}x}}]$"

        answer = str(derivative).replace("**", "^").replace(" ", "").replace("exp", "e^")
        return question, answer, "equivalent"


def generate_linear_algebra_problem(difficulty: int) -> tuple[str, str, str]:
    """Generate a linear algebra problem."""
    if difficulty < 1450:
        # Simple determinant
        a, b = random.randint(1, 5), random.randint(0, 3)
        c, d = random.randint(0, 3), random.randint(1, 5)
        mat = Matrix([[a, b], [c, d]])
        result = det(mat)

        question = f"Find the determinant of $\\begin{{pmatrix}} {a} & {b} \\\\ {c} & {d} \\end{{pmatrix}}$"
        answer = str(result)
        return question, answer, "equivalent"

    elif difficulty < 1550:
        # Trace
        a, b = random.randint(1, 5), random.randint(0, 3)
        c, d = random.randint(0, 3), random.randint(1, 5)
        mat = Matrix([[a, b], [c, d]])
        result = trace(mat)

        question = f"Find the trace of $\\begin{{pmatrix}} {a} & {b} \\\\ {c} & {d} \\end{{pmatrix}}$"
        answer = str(result)
        return question, answer, "equivalent"

    else:
        # Dot product
        v1 = [random.randint(1, 5), random.randint(1, 5)]
        v2 = [random.randint(1, 5), random.randint(1, 5)]
        result = v1[0] * v2[0] + v1[1] * v2[1]

        question = f"If $\\vec{{u}} = ({v1[0]}, {v1[1]})$ and $\\vec{{v}} = ({v2[0]}, {v2[1]})$, find $\\vec{{u}} \\cdot \\vec{{v}}$"
        answer = str(result)
        return question, answer, "equivalent"


def generate_problem(topic: str, difficulty: int) -> tuple[str, str, str]:
    """Generate a problem for the given topic and difficulty."""
    generators = {
        "algebra": generate_algebra_problem,
        "calculus": generate_calculus_problem,
        "linear_algebra": generate_linear_algebra_problem,
    }

    if topic not in generators:
        raise ValueError(f"Unknown topic: {topic}")

    return generators[topic](difficulty)


def main():
    parser = argparse.ArgumentParser(description="Generate math problems for Locus")
    parser.add_argument("--topic", choices=["algebra", "calculus", "linear_algebra", "all"],
                        default="all", help="Topic to generate problems for")
    parser.add_argument("--count", type=int, default=20, help="Number of problems per topic")
    parser.add_argument("--output", default="problems.sql", help="Output SQL file")
    parser.add_argument("--min-difficulty", type=int, default=1200, help="Minimum difficulty")
    parser.add_argument("--max-difficulty", type=int, default=1800, help="Maximum difficulty")

    args = parser.parse_args()

    topics = ["algebra", "calculus", "linear_algebra"] if args.topic == "all" else [args.topic]

    problems = []
    for topic in topics:
        for _ in range(args.count):
            difficulty = random.randint(args.min_difficulty, args.max_difficulty)
            try:
                question, answer, mode = generate_problem(topic, difficulty)
                problems.append((question, answer, difficulty, topic, mode))
            except Exception as e:
                print(f"Warning: Failed to generate problem: {e}")

    # Write SQL output
    with open(args.output, "w") as f:
        f.write("-- Generated problems for Locus\n\n")
        f.write("INSERT INTO problems (question_latex, answer_key, difficulty, topic, grading_mode) VALUES\n")

        lines = []
        for q, a, d, t, m in problems:
            # Escape single quotes
            q_escaped = q.replace("'", "''")
            a_escaped = a.replace("'", "''")
            lines.append(f"('{q_escaped}', '{a_escaped}', {d}, '{t}', '{m}')")

        f.write(",\n".join(lines))
        f.write(";\n")

    print(f"Generated {len(problems)} problems to {args.output}")


if __name__ == "__main__":
    main()
