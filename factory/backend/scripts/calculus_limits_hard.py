"""
calculus - limits (hard)
Generated: 2026-02-11T21:59:35.543847
"""

import sympy as sp
import random
import json

x = sp.Symbol('x')
h = sp.Symbol('h')

def generate_limit_problem():
    problem_type = random.choice([
        'lhopital_advanced',
        'squeeze_theorem',
        'epsilon_delta_inspired',
        'piecewise_continuity',
        'nested_composition',
        'indeterminate_advanced'
    ])
    
    if problem_type == 'lhopital_advanced':
        difficulty = random.randint(1650, 1850)
        
        a = random.choice([0, 1, 2])
        n = random.randint(2, 4)
        m = random.randint(2, 4)
        
        answer = sp.Rational(n, m)
        
        if a == 0:
            numerator = sp.sin(n * x)
            denominator = sp.sin(m * x)
            question = f"\\lim_{{x \\to 0}} \\frac{{\\sin({n}x)}}{{\\sin({m}x)}}"
        else:
            k = random.randint(1, 3)
            numerator = (sp.exp(k*x) - sp.exp(k*a))
            denominator = x - a
            answer = k * sp.exp(k*a)
            question = f"\\lim_{{x \\to {a}}} \\frac{{e^{{{k}x}} - e^{{{k*a}}}}}{{x - {a}}}"
        
        difficulty = 1700
        
    elif problem_type == 'squeeze_theorem':
        difficulty = random.randint(1700, 1900)
        
        a = random.randint(2, 4)
        question = f"\\lim_{{x \\to 0}} x^{{{a}}} \\sin\\left(\\frac{{1}}{{x}}\\right)"
        answer = sp.Integer(0)
        difficulty = 1750
        
    elif problem_type == 'epsilon_delta_inspired':
        difficulty = random.randint(1650, 1850)
        
        b = random.randint(2, 5)
        c = random.randint(1, 4)
        question = f"\\lim_{{x \\to \\infty}} \\frac{{{b}x^2 + {c}x}}{{x^2 + 1}}"
        answer = sp.Integer(b)
        difficulty = 1650
        
    elif problem_type == 'piecewise_continuity':
        difficulty = random.randint(1700, 1900)
        
        a = random.randint(1, 3)
        k = random.randint(2, 5)
        answer = k * a**2
        
        question = f"\\text{{Find }} k \\text{{ such that }} \\lim_{{x \\to {a}}} \\frac{{kx^2 - {answer}}}{{x - {a}}} = {2*k*a}"
        answer = sp.Integer(k)
        difficulty = 1800
        
    elif problem_type == 'nested_composition':
        difficulty = random.randint(1750, 1900)
        
        a = random.randint(2, 4)
        b = random.randint(1, 3)
        
        question = f"\\lim_{{x \\to 0^+}} \\frac{{\\ln(1 + {a}x)}}{{x}}"
        answer = sp.Integer(a)
        difficulty = 1700
        
    else:  # indeterminate_advanced
        difficulty = random.randint(1700, 1900)
        
        base_type = random.choice(['exp_limit', 'power_limit'])
        
        if base_type == 'exp_limit':
            a = random.randint(2, 5)
            b = random.randint(1, 4)
            question = f"\\lim_{{x \\to \\infty}} \\left(1 + \\frac{{{a}}}{{x}}\\right)^{{{b}x}}"
            answer = sp.exp(a * b)
            difficulty = 1800
        else:
            a = random.randint(2, 4)
            question = f"\\lim_{{x \\to 0^+}} x^x"
            answer = sp.Integer(1)
            difficulty = 1850
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "calculus",
        "subtopic": "limits",
        "grading_mode": "equivalent"
    }

problem = generate_limit_problem()
print(json.dumps(problem))