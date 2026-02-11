"""
precalculus - functions (medium)
Generated: 2026-02-11T21:51:48.424039
"""

import random
import json
from sympy import symbols, expand, factor, simplify, latex, Rational, sqrt, Abs, solve, sympify

def generate_problem():
    x, h = symbols('x h')
    problem_type = random.choice([
        'composition',
        'difference_quotient',
        'inverse_simple',
        'domain_rational',
        'piecewise_eval',
        'function_eval'
    ])
    
    if problem_type == 'composition':
        a = random.randint(1, 5)
        b = random.randint(-5, 5)
        c = random.randint(1, 4)
        d = random.randint(-4, 4)
        
        f_expr = a * x + b
        g_expr = c * x + d
        
        answer = simplify(f_expr.subs(x, g_expr))
        
        question = f"Let $f(x) = {latex(f_expr)}$ and $g(x) = {latex(g_expr)}$. Find $f(g(x))$."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1400,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'difference_quotient':
        a = random.randint(1, 4)
        b = random.randint(-3, 3)
        
        f_expr = a * x**2 + b * x
        f_xh = f_expr.subs(x, x + h)
        diff_quot = simplify((f_xh - f_expr) / h)
        
        question = f"Find the difference quotient $\\frac{{f(x+h) - f(x)}}{{h}}$ for $f(x) = {latex(f_expr)}$."
        
        return {
            "question_latex": question,
            "answer_key": str(diff_quot),
            "difficulty": 1500,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'inverse_simple':
        a = random.randint(2, 6)
        b = random.randint(-5, 5)
        
        f_expr = a * x + b
        y = symbols('y')
        inverse_solutions = solve(y - f_expr, x)
        inverse = inverse_solutions[0]
        
        question = f"Find the inverse function $f^{{-1}}(x)$ for $f(x) = {latex(f_expr)}$."
        answer = inverse.subs(y, x)
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1400,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'domain_rational':
        a = random.randint(1, 5)
        b = random.randint(-5, 5)
        
        f_expr = 1 / (x - a)
        
        question = f"Find the domain of $f(x) = {latex(f_expr)}$. Express your answer as the excluded value(s)."
        
        return {
            "question_latex": question,
            "answer_key": str(a),
            "difficulty": 1300,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'piecewise_eval':
        a = random.randint(2, 5)
        b = random.randint(1, 4)
        c = random.randint(-3, 3)
        threshold = random.randint(0, 3)
        eval_point = threshold + random.randint(1, 3)
        
        answer = a * eval_point + c
        
        question = f"Let $f(x) = \\begin{{cases}} {latex(b*x)} & \\text{{if }} x \\leq {threshold} \\\\ {latex(a*x + c)} & \\text{{if }} x > {threshold} \\end{{cases}}$. Find $f({eval_point})$."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1350,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    else:  # function_eval
        a = random.randint(1, 4)
        b = random.randint(-4, 4)
        c = random.randint(1, 4)
        eval_point = random.randint(-3, 3)
        
        f_expr = a * x**2 + b * x + c
        answer = f_expr.subs(x, eval_point)
        
        question = f"Evaluate $f({eval_point})$ where $f(x) = {latex(f_expr)}$."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1300,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }

problem = generate_problem()
print(json.dumps(problem))