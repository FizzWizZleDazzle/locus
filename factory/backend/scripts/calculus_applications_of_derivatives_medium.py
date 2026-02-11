"""
calculus - applications_of_derivatives (medium)
Generated: 2026-02-11T22:01:15.764061
"""

import sympy as sp
import random
import json

def generate_problem():
    x = sp.Symbol('x')
    
    problem_type = random.choice([
        'optimization_rectangle',
        'related_rates_circle',
        'motion_particle',
        'optimization_box',
        'tangent_line_application'
    ])
    
    if problem_type == 'optimization_rectangle':
        perimeter = random.choice([40, 60, 80, 100, 120])
        max_area = (perimeter / 4) ** 2
        
        question = f"A rectangle has a perimeter of {perimeter} units. Find the maximum area of the rectangle using calculus."
        answer = sp.Integer(int(max_area))
        difficulty = 1450
        
    elif problem_type == 'related_rates_circle':
        dr_dt = random.choice([2, 3, 4, 5])
        radius = random.choice([5, 6, 8, 10])
        dA_dt = 2 * sp.pi * radius * dr_dt
        
        question = f"The radius of a circle is increasing at a rate of {dr_dt} cm/s. Find the rate at which the area is increasing when the radius is {radius} cm."
        answer = dA_dt
        difficulty = 1500
        
    elif problem_type == 'motion_particle':
        a = random.choice([2, 3, 4])
        b = random.choice([12, 15, 18, 24])
        c = random.choice([10, 20, 30])
        t_crit = sp.Rational(b, 2*a)
        v_expr = -2*a*x + b
        
        question = f"A particle moves along a line with position function $s(t) = -{a}t^2 + {b}t + {c}$ (in meters). Find the time when the particle's velocity is zero."
        answer = t_crit
        difficulty = 1400
        
    elif problem_type == 'optimization_box':
        side_length = random.choice([12, 18, 24, 30])
        x_opt = sp.Rational(side_length, 6)
        max_volume = x_opt * (side_length - 2*x_opt)**2
        
        question = f"A square piece of cardboard with side length {side_length} inches has squares of side length $x$ cut from each corner. The sides are then folded up to form an open box. Find the value of $x$ that maximizes the volume of the box."
        answer = x_opt
        difficulty = 1550
        
    else:  # tangent_line_application
        a = random.choice([1, 2, 3, 4])
        x0 = random.choice([1, 2, 3])
        y0 = a * x0**2
        slope = 2 * a * x0
        b_intercept = y0 - slope * x0
        
        question = f"Find the y-intercept of the tangent line to the curve $y = {a}x^2$ at the point where $x = {x0}$."
        answer = sp.Integer(int(b_intercept))
        difficulty = 1350
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "calculus",
        "subtopic": "applications_of_derivatives",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_problem()
print(json.dumps(problem))