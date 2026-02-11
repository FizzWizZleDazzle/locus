"""
multivariable_calculus - partial_derivatives (medium)
Generated: 2026-02-11T22:03:21.058101
"""

import sympy as sp
import random
import json

def generate_partial_derivative_problem():
    x, y, z = sp.symbols('x y z')
    
    problem_type = random.choice([
        'basic_polynomial',
        'product_rule',
        'chain_rule',
        'mixed_partial',
        'second_order'
    ])
    
    if problem_type == 'basic_polynomial':
        # ELO: 1200-1400 - Direct partial differentiation
        coeff1 = random.randint(1, 5)
        coeff2 = random.randint(1, 5)
        power_x = random.randint(2, 4)
        power_y = random.randint(2, 4)
        constant = random.randint(-10, 10)
        
        f = coeff1 * x**power_x + coeff2 * y**power_y + constant
        var = random.choice([x, y])
        answer = sp.diff(f, var)
        
        question = f"\\text{{Find }} \\frac{{\\partial f}}{{\\partial {sp.latex(var)}}} \\text{{ where }} f(x,y) = {sp.latex(f)}"
        difficulty = 1300
        
    elif problem_type == 'product_rule':
        # ELO: 1400-1500 - Requires product rule
        power1 = random.randint(2, 3)
        power2 = random.randint(2, 3)
        coeff = random.randint(2, 5)
        
        f = x**power1 * y**power2
        var = random.choice([x, y])
        answer = sp.diff(f, var)
        
        question = f"\\text{{Find }} \\frac{{\\partial f}}{{\\partial {sp.latex(var)}}} \\text{{ where }} f(x,y) = {sp.latex(f)}"
        difficulty = 1450
        
    elif problem_type == 'chain_rule':
        # ELO: 1500-1600 - Requires chain rule
        inner_power = random.randint(2, 3)
        outer_power = random.randint(2, 3)
        coeff = random.randint(1, 4)
        
        inner = x**2 + y**2
        f = (coeff * inner)**outer_power
        var = random.choice([x, y])
        answer = sp.diff(f, var)
        
        question = f"\\text{{Find }} \\frac{{\\partial f}}{{\\partial {sp.latex(var)}}} \\text{{ where }} f(x,y) = {sp.latex(f)}"
        difficulty = 1550
        
    elif problem_type == 'mixed_partial':
        # ELO: 1500-1600 - Mixed partial derivatives
        coeff = random.randint(2, 5)
        power_x = random.randint(2, 4)
        power_y = random.randint(2, 4)
        
        f = coeff * x**power_x * y**power_y
        answer = sp.diff(f, x, y)
        
        question = f"\\text{{Find }} \\frac{{\\partial^2 f}}{{\\partial x \\partial y}} \\text{{ where }} f(x,y) = {sp.latex(f)}"
        difficulty = 1520
        
    else:  # second_order
        # ELO: 1400-1500 - Second order partial derivatives
        coeff1 = random.randint(2, 5)
        coeff2 = random.randint(2, 5)
        power_x = random.randint(2, 4)
        power_y = random.randint(2, 4)
        
        f = coeff1 * x**power_x + coeff2 * y**power_y
        var = random.choice([x, y])
        answer = sp.diff(f, var, 2)
        
        question = f"\\text{{Find }} \\frac{{\\partial^2 f}}{{\\partial {sp.latex(var)}^2}} \\text{{ where }} f(x,y) = {sp.latex(f)}"
        difficulty = 1450
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "multivariable_calculus",
        "subtopic": "partial_derivatives",
        "grading_mode": "equivalent"
    }

problem = generate_partial_derivative_problem()
print(json.dumps(problem))