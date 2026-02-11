"""
precalculus - polar_coordinates (easy)
Generated: 2026-02-11T21:55:44.983794
"""

import sympy as sp
import random
import json

def generate_polar_problem():
    problem_type = random.choice([
        'rectangular_to_polar_point',
        'polar_to_rectangular_point',
        'simple_polar_distance',
        'polar_to_rectangular_equation_simple',
        'rectangular_to_polar_equation_simple'
    ])
    
    if problem_type == 'rectangular_to_polar_point':
        # Pick clean polar coordinates first
        r_val = random.choice([1, 2, 3, 4, 5])
        theta_choice = random.choice([0, sp.pi/6, sp.pi/4, sp.pi/3, sp.pi/2, 
                                     2*sp.pi/3, 3*sp.pi/4, 5*sp.pi/6, sp.pi])
        
        # Calculate rectangular coordinates
        x_val = r_val * sp.cos(theta_choice)
        y_val = r_val * sp.sin(theta_choice)
        
        # Simplify to get clean values
        x_val = sp.simplify(x_val)
        y_val = sp.simplify(y_val)
        
        question = f"Convert the rectangular coordinates $({sp.latex(x_val)}, {sp.latex(y_val)})$ to polar coordinates $(r, \\theta)$ where $r > 0$ and $0 \\leq \\theta < 2\\pi$. Give $r$."
        answer = str(r_val)
        difficulty = 1200
        
    elif problem_type == 'polar_to_rectangular_point':
        # Pick clean polar coordinates
        r_val = random.choice([2, 3, 4, 5, 6])
        theta_choice = random.choice([0, sp.pi/6, sp.pi/4, sp.pi/3, sp.pi/2,
                                     2*sp.pi/3, 3*sp.pi/4, sp.pi])
        
        # Calculate x coordinate
        x_val = r_val * sp.cos(theta_choice)
        x_val = sp.simplify(x_val)
        
        question = f"Convert the polar coordinates $(r, \\theta) = ({r_val}, {sp.latex(theta_choice)})$ to rectangular coordinates. Give the $x$-coordinate."
        answer = str(x_val)
        difficulty = 1100
        
    elif problem_type == 'simple_polar_distance':
        # Distance from origin for a polar point
        r_val = random.choice([3, 4, 5, 6, 7, 8])
        theta_choice = random.choice([sp.pi/6, sp.pi/4, sp.pi/3, sp.pi/2,
                                     2*sp.pi/3, 3*sp.pi/4, 5*sp.pi/6])
        
        question = f"Find the distance from the origin to the point with polar coordinates $({r_val}, {sp.latex(theta_choice)})$."
        answer = str(r_val)
        difficulty = 1000
        
    elif problem_type == 'polar_to_rectangular_equation_simple':
        # r = constant -> circle
        r_val = random.choice([2, 3, 4, 5])
        
        # Answer: x^2 + y^2 = r^2
        answer_expr = r_val**2
        
        question = f"Convert the polar equation $r = {r_val}$ to rectangular form. Give the value of $x^2 + y^2$."
        answer = str(answer_expr)
        difficulty = 1200
        
    else:  # rectangular_to_polar_equation_simple
        # x = constant or y = constant
        if random.choice([True, False]):
            # x = a -> r cos(theta) = a -> r = a/cos(theta) = a sec(theta)
            a_val = random.choice([1, 2, 3, 4])
            question = f"Convert the rectangular equation $x = {a_val}$ to polar form $r = f(\\theta)$. What is $r \\cos(\\theta)$?"
            answer = str(a_val)
            difficulty = 1300
        else:
            # y = b -> r sin(theta) = b
            b_val = random.choice([1, 2, 3, 4])
            question = f"Convert the rectangular equation $y = {b_val}$ to polar form. What is $r \\sin(\\theta)$?"
            answer = str(b_val)
            difficulty = 1300
    
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