"""
calculus - limits (medium)
Generated: 2026-02-11T21:59:18.616809
"""

import random
import json
from sympy import symbols, limit, oo, sin, cos, tan, exp, ln, sqrt, simplify, S

def generate_limit_problem():
    x = symbols('x')
    problem_type = random.choice([
        'polynomial_ratio',
        'trig_limit',
        'indeterminate_factoring',
        'infinity_ratio',
        'exponential_limit',
        'sqrt_rationalization'
    ])
    
    if problem_type == 'polynomial_ratio':
        # ELO 1300-1400: Simple polynomial limits at finite points
        a = random.randint(-5, 5)
        b = random.randint(1, 5)
        c = random.randint(-5, 5)
        numerator = b * x + c
        denominator = random.randint(1, 4) * x + random.randint(1, 5)
        point = random.randint(-3, 3)
        
        expr = numerator / denominator
        answer = limit(expr, x, point)
        
        question = f"\\lim_{{x \\to {point}}} \\frac{{{simplify(numerator)}}}{{{simplify(denominator)}}}"
        elo = 1320
        
    elif problem_type == 'trig_limit':
        # ELO 1400-1500: sin(x)/x type limits
        k = random.choice([1, 2, 3])
        limit_type = random.choice(['sinx_x', 'trig_zero'])
        
        if limit_type == 'sinx_x':
            if k == 1:
                expr = sin(x) / x
                point = 0
                answer = S(1)
                question = f"\\lim_{{x \\to 0}} \\frac{{\\sin(x)}}{{x}}"
                elo = 1450
            else:
                expr = sin(k * x) / x
                point = 0
                answer = S(k)
                question = f"\\lim_{{x \\to 0}} \\frac{{\\sin({k}x)}}{{x}}"
                elo = 1480
        else:
            a = random.randint(1, 3)
            expr = sin(a * x) / (a * x)
            point = 0
            answer = S(1)
            question = f"\\lim_{{x \\to 0}} \\frac{{\\sin({a}x)}}{{{a}x}}"
            elo = 1420
            
    elif problem_type == 'indeterminate_factoring':
        # ELO 1450-1550: Factor and cancel
        a = random.randint(1, 5)
        b = random.randint(-4, 4)
        if b == 0:
            b = 1
        
        # (x - a)(x + b) / (x - a) as x -> a
        numerator = (x - a) * (x + b)
        denominator = x - a
        expr = numerator / denominator
        answer = a + b
        
        num_expanded = simplify(numerator)
        question = f"\\lim_{{x \\to {a}}} \\frac{{{num_expanded}}}{{x - {a}}}"
        elo = 1500
        
    elif problem_type == 'infinity_ratio':
        # ELO 1400-1550: Limits at infinity with polynomials
        n = random.choice([2, 3])
        m = random.choice([2, 3])
        
        a = random.randint(1, 5)
        b = random.randint(1, 5)
        
        if n == m:
            # Same degree - ratio of leading coefficients
            numerator = a * x**n + random.randint(-5, 5)
            denominator = b * x**n + random.randint(-5, 5)
            answer = S(a) / S(b)
            elo = 1450
        elif n > m:
            # Higher degree on top - infinity
            numerator = a * x**n + random.randint(-5, 5)
            denominator = b * x**m + random.randint(-5, 5)
            answer = oo
            elo = 1480
        else:
            # Higher degree on bottom - zero
            numerator = a * x**n + random.randint(-5, 5)
            denominator = b * x**m + random.randint(-5, 5)
            answer = S(0)
            elo = 1480
            
        question = f"\\lim_{{x \\to \\infty}} \\frac{{{simplify(numerator)}}}{{{simplify(denominator)}}}"
        expr = numerator / denominator
        
    elif problem_type == 'exponential_limit':
        # ELO 1350-1450: Simple exponential limits
        a = random.randint(1, 3)
        direction = random.choice(['0+', 'infinity'])
        
        if direction == '0+':
            expr = exp(-a / x)
            answer = S(0)
            question = f"\\lim_{{x \\to 0^+}} e^{{-\\frac{{{a}}}{{x}}}}"
            elo = 1400
        else:
            expr = exp(-a * x)
            answer = S(0)
            question = f"\\lim_{{x \\to \\infty}} e^{{-{a}x}}"
            elo = 1380
            
    else:  # sqrt_rationalization
        # ELO 1500-1600: Rationalize and simplify
        a = random.randint(1, 4)
        
        # (sqrt(x+a) - sqrt(x)) as x -> infinity
        expr = sqrt(x + a) - sqrt(x)
        answer = S(0)
        question = f"\\lim_{{x \\to \\infty}} \\left(\\sqrt{{x + {a}}} - \\sqrt{{x}}\\right)"
        elo = 1550
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "calculus",
        "subtopic": "limits",
        "grading_mode": "equivalent"
    }

problem = generate_limit_problem()
print(json.dumps(problem))