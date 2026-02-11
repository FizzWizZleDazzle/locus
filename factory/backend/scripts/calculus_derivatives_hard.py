"""
calculus - derivatives (hard)
Generated: 2026-02-11T22:00:08.820401
"""

import sympy as sp
import random
import json

def generate_derivative_problem():
    x = sp.Symbol('x')
    
    problem_types = [
        'chain_product',
        'quotient_chain',
        'implicit_trig',
        'logarithmic',
        'composite_three'
    ]
    
    problem_type = random.choice(problem_types)
    
    if problem_type == 'chain_product':
        a = random.randint(2, 5)
        b = random.randint(2, 4)
        c = random.randint(1, 3)
        
        inner = a*x + c
        f = x**b * sp.sin(inner)
        derivative = sp.diff(f, x)
        
        question = f"\\frac{{d}}{{dx}}\\left[x^{{{b}}} \\sin({sp.latex(inner)})\\right]"
        elo = random.randint(1650, 1750)
        
    elif problem_type == 'quotient_chain':
        a = random.randint(2, 4)
        b = random.randint(2, 4)
        c = random.randint(1, 3)
        
        numerator = sp.exp(a*x)
        denominator = x**b + c
        f = numerator / denominator
        derivative = sp.diff(f, x)
        
        question = f"\\frac{{d}}{{dx}}\\left[\\frac{{e^{{{a}x}}}}{{x^{{{b}}} + {c}}}\\right]"
        elo = random.randint(1700, 1800)
        
    elif problem_type == 'implicit_trig':
        a = random.randint(2, 4)
        b = random.randint(2, 4)
        
        f = sp.sin(x)**a * sp.cos(x)**b
        derivative = sp.diff(f, x)
        
        question = f"\\frac{{d}}{{dx}}\\left[\\sin^{{{a}}}(x) \\cos^{{{b}}}(x)\\right]"
        elo = random.randint(1600, 1700)
        
    elif problem_type == 'logarithmic':
        a = random.randint(2, 5)
        b = random.randint(2, 4)
        c = random.randint(1, 4)
        
        inner = x**b + c
        f = sp.ln(inner)**a
        derivative = sp.diff(f, x)
        
        question = f"\\frac{{d}}{{dx}}\\left[\\ln^{{{a}}}(x^{{{b}}} + {c})\\right]"
        elo = random.randint(1650, 1750)
        
    else:  # composite_three
        a = random.randint(2, 4)
        b = random.randint(2, 3)
        
        f = sp.exp(sp.sin(a*x**b))
        derivative = sp.diff(f, x)
        
        question = f"\\frac{{d}}{{dx}}\\left[e^{{\\sin({a}x^{{{b}}})}}\\right]"
        elo = random.randint(1750, 1850)
    
    return {
        "question_latex": question,
        "answer_key": str(derivative),
        "difficulty": elo,
        "main_topic": "calculus",
        "subtopic": "derivatives",
        "grading_mode": "equivalent"
    }

problem = generate_derivative_problem()
print(json.dumps(problem))