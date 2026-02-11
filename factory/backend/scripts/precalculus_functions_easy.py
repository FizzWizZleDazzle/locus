"""
precalculus - functions (easy)
Generated: 2026-02-11T21:51:31.119843
"""

import random
import json
from sympy import symbols, simplify, latex, sympify, sqrt, Rational, expand, factor

def generate_problem():
    x = symbols('x')
    problem_type = random.choice([
        'evaluate_function',
        'find_domain_simple',
        'function_composition_simple',
        'inverse_simple'
    ])
    
    if problem_type == 'evaluate_function':
        a = random.randint(1, 5)
        b = random.randint(-5, 5)
        eval_at = random.randint(-3, 3)
        
        answer = a * eval_at + b
        
        question = f"If $f(x) = {a}x + {b}$, find $f({eval_at})$."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1050,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'find_domain_simple':
        denom = random.randint(1, 8)
        
        question = f"Find the domain of $f(x) = \\frac{{1}}{{x - {denom}}}$. Express your answer in interval notation."
        answer = f"(-oo, {denom}) U ({denom}, oo)"
        
        return {
            "question_latex": question,
            "answer_key": answer,
            "difficulty": 1150,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'function_composition_simple':
        a = random.randint(2, 5)
        b = random.randint(1, 4)
        c = random.randint(1, 3)
        
        eval_at = random.randint(1, 3)
        g_result = b * eval_at
        f_result = a * g_result + c
        
        question = f"If $f(x) = {a}x + {c}$ and $g(x) = {b}x$, find $f(g({eval_at}))$."
        
        return {
            "question_latex": question,
            "answer_key": str(f_result),
            "difficulty": 1250,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }
    
    else:  # inverse_simple
        a = random.choice([2, 3, 4, 5])
        b = random.randint(-5, 5)
        
        inv_expr = f"(x - {b})/{a}"
        
        if b >= 0:
            question = f"Find the inverse of $f(x) = {a}x + {b}$."
        else:
            question = f"Find the inverse of $f(x) = {a}x - {-b}$."
        
        return {
            "question_latex": question,
            "answer_key": inv_expr,
            "difficulty": 1280,
            "main_topic": "precalculus",
            "subtopic": "functions",
            "grading_mode": "equivalent"
        }

problem = generate_problem()
print(json.dumps(problem))