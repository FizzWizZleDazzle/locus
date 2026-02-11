"""
calculus - limits (easy)
Generated: 2026-02-11T21:58:56.528952
"""

import sympy as sp
import random
import json

x = sp.Symbol('x')

def generate_problem():
    problem_type = random.choice([
        'direct_substitution',
        'simple_polynomial',
        'factoring_simple',
        'one_sided'
    ])
    
    if problem_type == 'direct_substitution':
        answer = random.choice([1, 2, 3, 4, 5, -1, -2, 0])
        a = random.randint(1, 5)
        b = answer - a * answer
        expr = a * x + b
        point = answer
        
        question = f"\\lim_{{x \\to {point}}} ({sp.latex(expr)})"
        elo = random.randint(1000, 1150)
        
    elif problem_type == 'simple_polynomial':
        answer = random.choice([0, 1, 2, 4, 5, 8, 9])
        point = random.randint(1, 3)
        a = random.randint(1, 3)
        b = random.randint(1, 4)
        c = answer - a * point**2 - b * point
        expr = a * x**2 + b * x + c
        
        question = f"\\lim_{{x \\to {point}}} ({sp.latex(expr)})"
        elo = random.randint(1100, 1250)
        
    elif problem_type == 'factoring_simple':
        point = random.choice([1, 2, 3])
        answer = random.choice([2, 3, 4, 5, 6])
        
        numerator = (x - point) * (x + answer)
        denominator = x - point
        
        expr = numerator / denominator
        
        question = f"\\lim_{{x \\to {point}}} \\left({sp.latex(expr)}\\right)"
        elo = random.randint(1200, 1300)
        
    else:  # one_sided
        answer = random.choice([0, 1, 2, 3])
        point = random.randint(1, 3)
        a = random.randint(1, 4)
        b = answer - a * point
        expr = a * x + b
        
        side = random.choice(['+', '-'])
        question = f"\\lim_{{x \\to {point}^{side}}} ({sp.latex(expr)})"
        elo = random.randint(1150, 1280)
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "calculus",
        "subtopic": "limits",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))