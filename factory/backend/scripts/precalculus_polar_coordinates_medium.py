"""
precalculus - polar_coordinates (medium)
Generated: 2026-02-11T21:56:01.825089
"""

import sympy as sp
import random
import json

def generate_polar_problem():
    problem_type = random.choice([
        'rectangular_to_polar',
        'polar_to_rectangular',
        'distance_between_polar',
        'polar_equation_convert'
    ])
    
    if problem_type == 'rectangular_to_polar':
        # Pick clean polar coordinates first
        r = random.choice([2, 3, 4, 5, 6])
        theta_choice = random.choice([
            (sp.pi/6, '30°'), (sp.pi/4, '45°'), (sp.pi/3, '60°'),
            (sp.pi/2, '90°'), (2*sp.pi/3, '120°'), (3*sp.pi/4, '135°'),
            (5*sp.pi/6, '150°'), (sp.pi, '180°')
        ])
        theta = theta_choice[0]
        
        # Calculate rectangular coordinates
        x = sp.simplify(r * sp.cos(theta))
        y = sp.simplify(r * sp.sin(theta))
        
        question = f"Convert the rectangular coordinates $({sp.latex(x)}, {sp.latex(y)})$ to polar coordinates $(r, \\theta)$ where $r > 0$ and $0 \\leq \\theta < 2\\pi$."
        answer = f"({r}, {sp.latex(theta)})"
        difficulty = 1350
        
    elif problem_type == 'polar_to_rectangular':
        # Pick clean polar coordinates
        r = random.choice([2, 3, 4, 5, 6])
        theta_choice = random.choice([
            sp.pi/6, sp.pi/4, sp.pi/3, sp.pi/2,
            2*sp.pi/3, 3*sp.pi/4, 5*sp.pi/6
        ])
        
        # Calculate rectangular
        x = sp.simplify(r * sp.cos(theta_choice))
        y = sp.simplify(r * sp.sin(theta_choice))
        
        question = f"Convert the polar coordinates $({r}, {sp.latex(theta_choice)})$ to rectangular coordinates $(x, y)$."
        answer = f"({sp.latex(x)}, {sp.latex(y)})"
        difficulty = 1300
        
    elif problem_type == 'distance_between_polar':
        # Pick two points with clean polar coordinates
        r1 = random.choice([2, 3, 4, 5])
        r2 = random.choice([2, 3, 4, 5])
        theta1 = random.choice([0, sp.pi/6, sp.pi/4, sp.pi/3, sp.pi/2])
        theta2 = random.choice([sp.pi/2, 2*sp.pi/3, 3*sp.pi/4, sp.pi])
        
        # Use distance formula: d^2 = r1^2 + r2^2 - 2*r1*r2*cos(theta2-theta1)
        d_squared = r1**2 + r2**2 - 2*r1*r2*sp.cos(theta2 - theta1)
        d = sp.simplify(sp.sqrt(d_squared))
        
        question = f"Find the distance between the points with polar coordinates $({r1}, {sp.latex(theta1)})$ and $({r2}, {sp.latex(theta2)})$."
        answer = sp.latex(d)
        difficulty = 1500
        
    else:  # polar_equation_convert
        # Convert simple polar equations to rectangular
        equation_type = random.choice(['circle', 'line'])
        
        if equation_type == 'circle':
            a = random.choice([2, 3, 4, 5])
            # r = 2a*cos(theta) converts to (x-a)^2 + y^2 = a^2
            question = f"Convert the polar equation $r = {2*a}\\cos(\\theta)$ to rectangular form."
            answer = f"(x - {a})^2 + y^2 = {a**2}"
            difficulty = 1550
            
        else:  # line
            # r*cos(theta) = a becomes x = a
            a = random.choice([1, 2, 3, 4, 5])
            question = f"Convert the polar equation $r\\cos(\\theta) = {a}$ to rectangular form."
            answer = f"x = {a}"
            difficulty = 1400
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "precalculus",
        "subtopic": "polar_coordinates",
        "grading_mode": "equivalent"
    }

problem = generate_polar_problem()
print(json.dumps(problem))