"""
calculus - derivatives (easy)
Generated: 2026-02-11T21:59:45.344673
"""

import sympy as sp
import random
import json

x = sp.Symbol('x')

def generate_problem():
    problem_type = random.choice([
        'power_rule_basic',
        'power_rule_coefficient',
        'constant',
        'sum_simple',
        'polynomial_two_term'
    ])
    
    if problem_type == 'power_rule_basic':
        n = random.randint(2, 5)
        answer = n * x**(n-1)
        question = x**n
        difficulty = 1050
        question_latex = f"\\frac{{d}}{{dx}}\\left[{sp.latex(question)}\\right]"
    
    elif problem_type == 'power_rule_coefficient':
        coeff = random.choice([2, 3, 4, 5, -2, -3])
        n = random.randint(2, 4)
        answer = coeff * n * x**(n-1)
        question = coeff * x**n
        difficulty = 1100
        question_latex = f"\\frac{{d}}{{dx}}\\left[{sp.latex(question)}\\right]"
    
    elif problem_type == 'constant':
        c = random.randint(1, 20)
        answer = sp.Integer(0)
        question = sp.Integer(c)
        difficulty = 1000
        question_latex = f"\\frac{{d}}{{dx}}\\left[{c}\\right]"
    
    elif problem_type == 'sum_simple':
        n = random.randint(2, 4)
        c = random.randint(-10, 10)
        if c == 0:
            c = 5
        question = x**n + c
        answer = n * x**(n-1)
        difficulty = 1150
        question_latex = f"\\frac{{d}}{{dx}}\\left[{sp.latex(question)}\\right]"
    
    elif problem_type == 'polynomial_two_term':
        coeff1 = random.choice([2, 3, 4, 5])
        n1 = random.randint(2, 3)
        coeff2 = random.choice([2, 3, 4, 5, -2, -3])
        question = coeff1 * x**n1 + coeff2 * x
        answer = coeff1 * n1 * x**(n1-1) + coeff2
        difficulty = 1250
        question_latex = f"\\frac{{d}}{{dx}}\\left[{sp.latex(question)}\\right]"
    
    answer_simplified = sp.simplify(answer)
    
    result = {
        "question_latex": question_latex,
        "answer_key": str(answer_simplified),
        "difficulty": difficulty,
        "main_topic": "calculus",
        "subtopic": "derivatives",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_problem()
print(json.dumps(problem))