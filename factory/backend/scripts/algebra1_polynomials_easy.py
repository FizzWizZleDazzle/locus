"""
algebra1 - polynomials (easy)
Generated: 2026-02-11T21:36:50.008450
"""

import sympy as sp
import random
import json

def generate_polynomial_problem():
    x = sp.Symbol('x')
    
    problem_type = random.choice([
        'evaluate_at_value',
        'add_polynomials',
        'subtract_polynomials',
        'multiply_monomial',
        'simple_expand',
        'combine_like_terms'
    ])
    
    if problem_type == 'evaluate_at_value':
        value = random.randint(1, 5)
        coef_a = random.randint(1, 5)
        coef_b = random.randint(-10, 10)
        
        if random.choice([True, False]):
            poly = coef_a * x + coef_b
            answer = coef_a * value + coef_b
            question = f"Evaluate {sp.latex(poly)} when x = {value}"
            elo = random.randint(1000, 1150)
        else:
            coef_c = random.randint(1, 4)
            poly = coef_a * x**2 + coef_b * x + coef_c
            answer = coef_a * value**2 + coef_b * value + coef_c
            question = f"Evaluate {sp.latex(poly)} when x = {value}"
            elo = random.randint(1100, 1250)
    
    elif problem_type == 'add_polynomials':
        a1, b1 = random.randint(1, 8), random.randint(-10, 10)
        a2, b2 = random.randint(1, 8), random.randint(-10, 10)
        
        poly1 = a1 * x + b1
        poly2 = a2 * x + b2
        answer = sp.simplify(poly1 + poly2)
        
        question = f"Add: ({sp.latex(poly1)}) + ({sp.latex(poly2)})"
        elo = random.randint(1050, 1200)
    
    elif problem_type == 'subtract_polynomials':
        a1, b1 = random.randint(1, 8), random.randint(-10, 10)
        a2, b2 = random.randint(1, 8), random.randint(-10, 10)
        
        poly1 = a1 * x + b1
        poly2 = a2 * x + b2
        answer = sp.simplify(poly1 - poly2)
        
        question = f"Subtract: ({sp.latex(poly1)}) - ({sp.latex(poly2)})"
        elo = random.randint(1100, 1250)
    
    elif problem_type == 'multiply_monomial':
        coef = random.randint(2, 6)
        a = random.randint(1, 6)
        b = random.randint(-10, 10)
        
        poly = a * x + b
        answer = sp.expand(coef * poly)
        
        question = f"Multiply: {coef}({sp.latex(poly)})"
        elo = random.randint(1150, 1300)
    
    elif problem_type == 'simple_expand':
        a = random.randint(1, 5)
        b = random.randint(1, 5)
        
        if random.choice([True, False]):
            expr = (x + a) * (x + b)
            answer = sp.expand(expr)
            question = f"Expand: {sp.latex(expr)}"
            elo = random.randint(1200, 1300)
        else:
            c = random.randint(2, 4)
            expr = c * (x + a)
            answer = sp.expand(expr)
            question = f"Expand: {sp.latex(expr)}"
            elo = random.randint(1100, 1250)
    
    else:  # combine_like_terms
        a1 = random.randint(1, 6)
        a2 = random.randint(1, 6)
        b = random.randint(-10, 10)
        
        expr = a1 * x + b + a2 * x
        answer = sp.simplify(expr)
        
        question = f"Simplify: {sp.latex(expr)}"
        elo = random.randint(1050, 1200)
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "algebra1",
        "subtopic": "polynomials",
        "grading_mode": "equivalent"
    }

problem = generate_polynomial_problem()
print(json.dumps(problem))