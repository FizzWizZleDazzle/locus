"""
algebra2 - rational_expressions (easy)
Generated: 2026-02-11T21:46:30.595082
"""

import sympy as sp
import random
import json

x = sp.Symbol('x')

def generate_problem():
    problem_type = random.choice([
        'simplify_monomial',
        'simplify_binomial', 
        'multiply_simple',
        'divide_simple',
        'add_same_denom',
        'subtract_same_denom'
    ])
    
    if problem_type == 'simplify_monomial':
        # Simplify (ax)/(bx) = a/b
        a = random.randint(2, 12)
        b = random.randint(2, 12)
        while sp.gcd(a, b) != 1:
            a = random.randint(2, 12)
            b = random.randint(2, 12)
        
        numerator = a * x
        denominator = b * x
        answer = sp.simplify(numerator / denominator)
        
        question = f"\\text{{Simplify: }} \\frac{{{sp.latex(numerator)}}}{{{sp.latex(denominator)}}}"
        difficulty = 1050
        
    elif problem_type == 'simplify_binomial':
        # Simplify (ax + b)/(cx + d) where there's a common factor
        factor = random.randint(2, 5)
        a = random.randint(1, 4)
        b = random.randint(1, 6)
        
        numerator = factor * (a * x + b)
        denominator = factor * random.randint(2, 6)
        answer = sp.simplify(numerator / denominator)
        
        question = f"\\text{{Simplify: }} \\frac{{{sp.latex(sp.expand(numerator))}}}{{{denominator}}}"
        difficulty = 1150
        
    elif problem_type == 'multiply_simple':
        # Multiply two simple fractions
        a = random.randint(1, 6)
        b = random.randint(2, 8)
        c = random.randint(1, 6)
        d = random.randint(2, 8)
        
        frac1 = a / b
        frac2 = (c * x) / d
        answer = sp.simplify(frac1 * frac2)
        
        question = f"\\text{{Multiply: }} \\frac{{{a}}}{{{b}}} \\cdot \\frac{{{sp.latex(c*x)}}}{{{d}}}"
        difficulty = 1100
        
    elif problem_type == 'divide_simple':
        # Divide simple fractions
        a = random.randint(1, 6)
        b = random.randint(2, 6)
        c = random.randint(1, 6)
        d = random.randint(2, 6)
        
        frac1 = (a * x) / b
        frac2 = c / d
        answer = sp.simplify(frac1 / frac2)
        
        question = f"\\text{{Divide: }} \\frac{{{sp.latex(a*x)}}}{{{b}}} \\div \\frac{{{c}}}{{{d}}}"
        difficulty = 1200
        
    elif problem_type == 'add_same_denom':
        # Add fractions with same denominator
        denom = random.randint(3, 10)
        a = random.randint(1, 5)
        b = random.randint(1, 5)
        c = random.randint(1, 5)
        d = random.randint(1, 5)
        
        frac1 = (a * x + b) / denom
        frac2 = (c * x + d) / denom
        answer = sp.simplify(frac1 + frac2)
        
        question = f"\\text{{Add: }} \\frac{{{sp.latex(a*x + b)}}}{{{denom}}} + \\frac{{{sp.latex(c*x + d)}}}{{{denom}}}"
        difficulty = 1150
        
    else:  # subtract_same_denom
        # Subtract fractions with same denominator
        denom = random.randint(3, 10)
        a = random.randint(2, 6)
        b = random.randint(3, 8)
        c = random.randint(1, 4)
        d = random.randint(1, 6)
        
        frac1 = (a * x + b) / denom
        frac2 = (c * x + d) / denom
        answer = sp.simplify(frac1 - frac2)
        
        question = f"\\text{{Subtract: }} \\frac{{{sp.latex(a*x + b)}}}{{{denom}}} - \\frac{{{sp.latex(c*x + d)}}}{{{denom}}}"
        difficulty = 1200
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "rational_expressions",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))