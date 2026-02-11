"""
calculus - derivatives (medium)
Generated: 2026-02-11T21:59:55.996313
"""

import random
import json
from sympy import symbols, diff, sin, cos, tan, exp, ln, sqrt, latex, simplify, expand

x = symbols('x')

def generate_derivative_problem():
    problem_type = random.choice([
        'polynomial',
        'product_rule',
        'quotient_rule', 
        'chain_rule_trig',
        'chain_rule_exp',
        'combination'
    ])
    
    if problem_type == 'polynomial':
        degree = random.randint(2, 4)
        coeffs = [random.randint(-5, 5) for _ in range(degree + 1)]
        coeffs = [c if c != 0 else 1 for c in coeffs]
        
        expr = sum(coeffs[i] * x**i for i in range(degree + 1))
        derivative = diff(expr, x)
        
        difficulty = 1300 + degree * 50
        
    elif problem_type == 'product_rule':
        a = random.randint(1, 5)
        b = random.randint(1, 4)
        c = random.randint(1, 5)
        d = random.randint(1, 3)
        
        expr = (a * x**b) * (c * x**d)
        derivative = diff(expr, x)
        
        difficulty = 1450
        
    elif problem_type == 'quotient_rule':
        a = random.randint(1, 5)
        b = random.randint(1, 3)
        c = random.randint(1, 5)
        
        expr = (a * x**b) / (c * x + random.randint(1, 5))
        derivative = simplify(diff(expr, x))
        
        difficulty = 1550
        
    elif problem_type == 'chain_rule_trig':
        inner_coeff = random.randint(2, 5)
        outer_func = random.choice([sin, cos])
        
        expr = outer_func(inner_coeff * x)
        derivative = diff(expr, x)
        
        difficulty = 1400
        
    elif problem_type == 'chain_rule_exp':
        inner_coeff = random.randint(2, 5)
        constant = random.randint(1, 5)
        
        expr = exp(inner_coeff * x + constant)
        derivative = diff(expr, x)
        
        difficulty = 1450
        
    else:  # combination
        a = random.randint(1, 4)
        b = random.randint(2, 4)
        trig_func = random.choice([sin, cos])
        
        expr = a * x**2 + trig_func(b * x)
        derivative = diff(expr, x)
        
        difficulty = 1500
    
    question_latex = r"\frac{d}{dx}\left[" + latex(expr) + r"\right]"
    answer_key = str(derivative)
    
    return {
        "question_latex": question_latex,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "calculus",
        "subtopic": "derivatives",
        "grading_mode": "equivalent"
    }

problem = generate_derivative_problem()
print(json.dumps(problem))