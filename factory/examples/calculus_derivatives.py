#!/usr/bin/env python3
"""
Example Problem Generator Script: Calculus Derivatives

This script generates derivative problems using SymPy for symbolic mathematics.
It demonstrates proper use of SymPy for problem generation.
"""

import sympy as sp
import random
import json

# Define symbol
x = sp.Symbol('x')

# Generate random polynomial coefficients
a = random.randint(1, 10)
b = random.randint(1, 10)
c = random.randint(-10, 10)

# Create random polynomial expression
expr_type = random.choice(['polynomial', 'power', 'product'])

if expr_type == 'polynomial':
    # Random polynomial: ax^2 + bx + c
    expr = a * x**2 + b * x + c
elif expr_type == 'power':
    # Random power: ax^n
    n = random.randint(2, 5)
    expr = a * x**n
else:
    # Random product: ax * bx
    expr = a * x * b * x

# Calculate derivative using SymPy
derivative = sp.diff(expr, x)

# Format expressions as LaTeX
question_latex = f"Find $\\frac{{d}}{{dx}}\\left({sp.latex(expr)}\\right)$"

# Convert derivative to string for answer_key
answer_key = str(derivative)

# Difficulty based on expression complexity
if expr_type == 'polynomial':
    difficulty = random.randint(1200, 1500)
elif expr_type == 'power':
    difficulty = random.randint(1000, 1300)
else:
    difficulty = random.randint(1100, 1400)

# Create problem
problem = {
    "question_latex": question_latex,
    "answer_key": answer_key,
    "difficulty": difficulty,
    "main_topic": "calculus",
    "subtopic": "derivatives",
    "grading_mode": "equivalent",
    "answer_type": "expression",
    "calculator_allowed": "none"
}

# Output JSON
print(json.dumps(problem))
