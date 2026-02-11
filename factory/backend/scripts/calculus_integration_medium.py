"""
calculus - integration (medium)
Generated: 2026-02-11T22:00:29.473508
"""

import sympy as sp
import random
import json

def generate_integration_problem():
    x = sp.Symbol('x')
    
    problem_types = [
        'polynomial',
        'exponential',
        'trigonometric',
        'polynomial_with_coefficient',
        'simple_chain_rule'
    ]
    
    problem_type = random.choice(problem_types)
    
    if problem_type == 'polynomial':
        degree = random.randint(2, 4)
        coeff = random.randint(1, 5)
        answer = coeff * x**degree
        problem = sp.diff(answer, x)
        difficulty = 1300 + (degree - 2) * 50
        
    elif problem_type == 'polynomial_with_coefficient':
        degree = random.randint(2, 3)
        coeff1 = random.randint(2, 6)
        coeff2 = random.randint(1, 5)
        constant = random.randint(-10, 10)
        answer = coeff2 * x**degree + constant
        problem = coeff1 * x**(degree - 1)
        answer = problem.integrate(x)
        difficulty = 1350
        
    elif problem_type == 'exponential':
        coeff = random.randint(2, 8)
        answer = coeff * sp.exp(x)
        problem = sp.diff(answer, x)
        difficulty = 1320
        
    elif problem_type == 'trigonometric':
        trig_type = random.choice(['sin', 'cos'])
        coeff = random.randint(2, 6)
        
        if trig_type == 'sin':
            answer = -coeff * sp.cos(x)
            problem = coeff * sp.sin(x)
        else:
            answer = coeff * sp.sin(x)
            problem = coeff * sp.cos(x)
        
        difficulty = 1340
        
    elif problem_type == 'simple_chain_rule':
        inner_coeff = random.randint(2, 5)
        outer_power = random.randint(2, 3)
        
        answer = (inner_coeff * x)**outer_power / (inner_coeff * outer_power)
        problem = sp.diff(answer, x)
        problem = sp.simplify(problem)
        
        difficulty = 1500
    
    question_latex = f"\\int \\left({sp.latex(problem)}\\right) dx"
    answer_key = str(answer)
    
    result = {
        "question_latex": question_latex,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "calculus",
        "subtopic": "integration",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_integration_problem()
print(json.dumps(problem))