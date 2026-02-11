"""
algebra1 - polynomials (medium)
Generated: 2026-02-11T21:37:03.815618
"""

import random
import json
from sympy import symbols, expand, factor, simplify, latex, Poly, Add, Mul

x = symbols('x')

def generate_problem():
    problem_type = random.choice([
        'expand_binomial',
        'factor_trinomial',
        'add_polynomials',
        'multiply_binomials',
        'subtract_polynomials'
    ])
    
    if problem_type == 'expand_binomial':
        a = random.randint(1, 5)
        b = random.randint(1, 10)
        c = random.randint(1, 5)
        d = random.randint(1, 10)
        
        if random.choice([True, False]):
            d = -d
        if random.choice([True, False]):
            b = -b
            
        expr1 = a*x + b
        expr2 = c*x + d
        answer = expand(expr1 * expr2)
        
        question = f"\\text{{Expand: }} ({latex(expr1)})({latex(expr2)})"
        difficulty = 1400
        
    elif problem_type == 'factor_trinomial':
        r1 = random.randint(-6, 6)
        r2 = random.randint(-6, 6)
        while r1 == 0 or r2 == 0 or r1 == r2:
            r1 = random.randint(-6, 6)
            r2 = random.randint(-6, 6)
        
        factored = (x - r1) * (x - r2)
        trinomial = expand(factored)
        
        question = f"\\text{{Factor: }} {latex(trinomial)}"
        answer = factored
        difficulty = 1500
        
    elif problem_type == 'add_polynomials':
        a1 = random.randint(1, 8)
        b1 = random.randint(-10, 10)
        c1 = random.randint(-10, 10)
        
        a2 = random.randint(1, 8)
        b2 = random.randint(-10, 10)
        c2 = random.randint(-10, 10)
        
        poly1 = a1*x**2 + b1*x + c1
        poly2 = a2*x**2 + b2*x + c2
        
        answer = simplify(poly1 + poly2)
        
        question = f"\\text{{Simplify: }} ({latex(poly1)}) + ({latex(poly2)})"
        difficulty = 1300
        
    elif problem_type == 'multiply_binomials':
        a = random.randint(2, 6)
        b = random.randint(1, 8)
        c = random.randint(2, 6)
        d = random.randint(1, 8)
        
        if random.choice([True, False]):
            b = -b
        if random.choice([True, False]):
            d = -d
            
        expr1 = a*x + b
        expr2 = c*x + d
        answer = expand(expr1 * expr2)
        
        question = f"\\text{{Multiply: }} ({latex(expr1)})({latex(expr2)})"
        difficulty = 1350
        
    else:  # subtract_polynomials
        a1 = random.randint(2, 9)
        b1 = random.randint(-12, 12)
        c1 = random.randint(-12, 12)
        
        a2 = random.randint(1, 7)
        b2 = random.randint(-12, 12)
        c2 = random.randint(-12, 12)
        
        poly1 = a1*x**2 + b1*x + c1
        poly2 = a2*x**2 + b2*x + c2
        
        answer = simplify(poly1 - poly2)
        
        question = f"\\text{{Simplify: }} ({latex(poly1)}) - ({latex(poly2)})"
        difficulty = 1350
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "polynomials",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))