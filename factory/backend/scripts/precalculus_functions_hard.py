"""
precalculus - functions (hard)
Generated: 2026-02-11T21:52:09.551943
"""

import random
import json
from sympy import symbols, simplify, expand, factor, sqrt, log, ln, exp, sin, cos, tan, Rational, latex, Abs, solve, compose, Lambda, Function, sympify

def generate_problem():
    x, y, t, u = symbols('x y t u')
    problem_type = random.choice([
        'composition_evaluate',
        'inverse_function',
        'composition_formula',
        'domain_restriction',
        'piecewise_composition',
        'transformation_composition'
    ])
    
    if problem_type == 'composition_evaluate':
        a = random.randint(2, 5)
        b = random.randint(1, 4)
        c = random.randint(2, 6)
        d = random.randint(1, 5)
        eval_point = random.randint(1, 4)
        
        inner_result = a * eval_point + b
        answer = c * inner_result**2 + d
        
        question = f"Let $f(x) = {c}x^2 + {d}$ and $g(x) = {a}x + {b}$. Find $f(g({eval_point}))$."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1650,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'inverse_function':
        a = random.choice([2, 3, 4, 5])
        b = random.choice([1, 2, 3, 4, 5, 6])
        c = random.choice([1, 2, 3])
        
        answer_expr = (x - b) / a
        
        question = f"Find the inverse function of $f(x) = {a}x + {b}$. Express your answer as $f^{{-1}}(x)$."
        
        return {
            "question_latex": question,
            "answer_key": str(answer_expr),
            "difficulty": 1620,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'composition_formula':
        a = random.randint(2, 4)
        b = random.randint(1, 5)
        c = random.randint(2, 5)
        
        f_expr = a * x**2 + b
        g_expr = c * x
        
        composed = simplify(f_expr.subs(x, g_expr))
        
        question = f"Let $f(x) = {a}x^2 + {b}$ and $g(x) = {c}x$. Find the formula for $(f \\circ g)(x)$."
        
        return {
            "question_latex": question,
            "answer_key": str(composed),
            "difficulty": 1680,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'domain_restriction':
        a = random.randint(2, 6)
        b = random.randint(1, 5)
        
        restricted_value = b
        
        question = f"Find the domain of $h(x) = \\frac{{{a}x + {random.randint(1,5)}}}{{x - {b}}}$. Express your answer using interval notation, or state the excluded value as $x \\neq c$ where $c$ is the excluded value."
        
        return {
            "question_latex": question,
            "answer_key": str(b),
            "difficulty": 1640,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'piecewise_composition':
        a = random.randint(2, 5)
        b = random.randint(1, 4)
        c = random.randint(1, 3)
        threshold = random.randint(0, 2)
        eval_point = random.choice([-2, -1, 3, 4, 5])
        
        if eval_point <= threshold:
            g_val = a * eval_point + b
        else:
            g_val = eval_point**2 + c
        
        d = random.randint(2, 4)
        e = random.randint(1, 3)
        
        if g_val <= threshold:
            answer = a * g_val + b
        else:
            answer = g_val**2 + c
        
        question = f"Let $g(x) = \\begin{{cases}} {a}x + {b} & \\text{{if }} x \\leq {threshold} \\\\ x^2 + {c} & \\text{{if }} x > {threshold} \\end{{cases}}$. Find $g(g({eval_point}))$."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1780,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'transformation_composition':
        a = random.randint(2, 4)
        h = random.randint(1, 3)
        k = random.randint(1, 4)
        eval_point = random.randint(1, 4)
        
        transformed_input = eval_point - h
        base_output = a * transformed_input**2
        answer = base_output + k
        
        question = f"If $f(x) = {a}x^2$, find the value of $g(x) = f(x - {h}) + {k}$ at $x = {eval_point}$."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1720,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }

problem = generate_problem()
print(json.dumps(problem))