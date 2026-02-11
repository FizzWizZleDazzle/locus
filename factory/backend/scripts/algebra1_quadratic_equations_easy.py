"""
algebra1 - quadratic_equations (easy)
Generated: 2026-02-11T21:38:15.019084
"""

import sympy as sp
import random
import json

def generate_quadratic_problem():
    x = sp.Symbol('x')
    
    problem_type = random.choice([
        'perfect_square',
        'factored_form', 
        'simple_unfactored',
        'completing_square_easy'
    ])
    
    if problem_type == 'perfect_square':
        # (x + a)^2 = b^2 where b is small integer
        a = random.randint(-5, 5)
        b = random.randint(1, 4)
        b_squared = b**2
        
        question = f"Solve for $x$: $(x {'+' if a >= 0 else ''} {a})^2 = {b_squared}$"
        
        solutions = [b - a, -b - a]
        solutions.sort()
        answer_key = str(solutions)
        difficulty = 1050
        
    elif problem_type == 'factored_form':
        # (x - r1)(x - r2) = 0 with small integer roots
        r1 = random.randint(-6, 6)
        r2 = random.randint(-6, 6)
        
        expanded = sp.expand((x - r1) * (x - r2))
        
        question = f"Solve for $x$: ${sp.latex(expanded)} = 0$"
        
        solutions = sorted([r1, r2])
        answer_key = str(solutions)
        difficulty = 1100
        
    elif problem_type == 'simple_unfactored':
        # x^2 = c where c is perfect square
        c = random.choice([4, 9, 16, 25, 36, 49, 64, 81, 100])
        
        question = f"Solve for $x$: $x^2 = {c}$"
        
        root = int(c**0.5)
        solutions = [-root, root]
        answer_key = str(solutions)
        difficulty = 1000
        
    else:  # completing_square_easy
        # x^2 + 2bx + b^2 = c where c makes clean answer
        b = random.choice([2, 3, 4, 5])
        c_extra = random.choice([1, 4, 9, 16])
        c = b**2 + c_extra
        
        coeff = 2 * b
        constant = b**2
        
        question = f"Solve for $x$ by completing the square: $x^2 {'+' if coeff >= 0 else ''} {coeff}x {'+' if constant >= 0 else ''} {constant} = {c}$"
        
        sqrt_val = int(c_extra**0.5)
        solutions = sorted([-b - sqrt_val, -b + sqrt_val])
        answer_key = str(solutions)
        difficulty = 1250
    
    return {
        "question_latex": question,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "quadratic_equations",
        "grading_mode": "equivalent"
    }

problem = generate_quadratic_problem()
print(json.dumps(problem))