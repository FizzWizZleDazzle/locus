"""
calculus - applications_of_derivatives (hard)
Generated: 2026-02-11T22:01:40.785885
"""

import random
import json
from sympy import symbols, sympify, latex, diff, solve, sqrt, log, exp, sin, cos, tan, pi, Rational, simplify, oo

def generate_problem():
    x = symbols('x')
    problem_type = random.choice([
        'related_rates',
        'optimization',
        'mean_value_theorem',
        'curve_sketching',
        'linearization',
        'lhopital'
    ])
    
    if problem_type == 'related_rates':
        scenario = random.choice(['cylinder', 'cone', 'sphere'])
        
        if scenario == 'cylinder':
            r_rate = random.choice([2, 3, 4, 5])
            h_rate = random.choice([1, 2, 3])
            r_val = random.choice([3, 4, 5, 6])
            h_val = random.choice([8, 10, 12])
            
            dV_dt = pi * (2 * r_val * r_rate * h_val + r_val**2 * h_rate)
            
            question = f"The radius of a cylinder is increasing at {r_rate} cm/s and the height is increasing at {h_rate} cm/s. Find the rate of change of volume (in cm³/s) when r = {r_val} cm and h = {h_val} cm."
            answer = dV_dt
            elo = 1650
            
        elif scenario == 'cone':
            r_rate = random.choice([2, 3, 4])
            h_val = random.choice([8, 10, 12])
            r_val = h_val // 2
            
            dV_dt = pi * r_val * r_rate * h_val / 3
            
            question = f"A cone has height twice its radius. If the radius is increasing at {r_rate} cm/s, find the rate of change of volume (in cm³/s) when the height is {h_val} cm."
            answer = dV_dt
            elo = 1700
            
        else:
            r_rate = random.choice([2, 3, 4, 5])
            r_val = random.choice([5, 6, 8, 10])
            
            dV_dt = 4 * pi * r_val**2 * r_rate
            
            question = f"The radius of a sphere is increasing at {r_rate} cm/s. Find the rate of change of volume (in cm³/s) when the radius is {r_val} cm."
            answer = dV_dt
            elo = 1620
    
    elif problem_type == 'optimization':
        opt_scenario = random.choice(['rectangle', 'box', 'fence'])
        
        if opt_scenario == 'rectangle':
            perimeter = random.choice([40, 60, 80, 100, 120])
            max_area = (perimeter / 4) ** 2
            
            question = f"Find the maximum area of a rectangle with perimeter {perimeter} units."
            answer = max_area
            elo = 1650
            
        elif opt_scenario == 'box':
            sheet_size = random.choice([12, 18, 24])
            cut_size = sheet_size / 6
            max_volume = cut_size * (sheet_size - 2*cut_size)**2
            
            question = f"A square sheet of metal with side length {sheet_size} inches has squares cut from each corner to form an open-top box. Find the maximum volume (in cubic inches) of such a box."
            answer = max_volume
            elo = 1750
            
        else:
            total_fence = random.choice([200, 300, 400, 500])
            max_area = total_fence**2 / 8
            
            question = f"A farmer has {total_fence} feet of fencing to enclose a rectangular area against a barn (one side needs no fence). Find the maximum area that can be enclosed."
            answer = max_area
            elo = 1680
    
    elif problem_type == 'mean_value_theorem':
        a = random.choice([0, 1, 2])
        b = a + random.choice([2, 3, 4])
        coeff = random.choice([1, 2, 3])
        
        f_a = coeff * a**2
        f_b = coeff * b**2
        c_val = (a + b) / 2
        
        question = f"Find the value c guaranteed by the Mean Value Theorem for f(x) = {coeff}x² on [{a}, {b}]."
        answer = c_val
        elo = 1620
    
    elif problem_type == 'curve_sketching':
        a = random.choice([2, 3, 4])
        inflection_point = 0
        
        question = f"Find the x-coordinate of the inflection point of f(x) = x³ - {3*a}x²."
        answer = a
        elo = 1660
    
    elif problem_type == 'linearization':
        func_choice = random.choice(['sqrt', 'cube_root'])
        
        if func_choice == 'sqrt':
            base = random.choice([4, 9, 16, 25])
            delta = random.choice([0.1, 0.2, 0.5, 1])
            
            L_value = sqrt(base) + delta / (2 * sqrt(base))
            
            question = f"Use linear approximation to estimate sqrt({base + delta})."
            answer = L_value
            elo = 1680
            
        else:
            base = random.choice([8, 27, 64])
            delta = random.choice([1, 2])
            
            L_value = base**(Rational(1,3)) + delta / (3 * base**(Rational(2,3)))
            
            question = f"Use linear approximation to estimate the cube root of {base + delta}."
            answer = L_value
            elo = 1720
    
    else:  # lhopital
        limit_type = random.choice(['poly_ratio', 'exp_ratio'])
        
        if limit_type == 'poly_ratio':
            a = random.choice([2, 3, 4, 5])
            
            question = f"Evaluate the limit: lim_(x→∞) ({a}x² + x) / (x² + 1)"
            answer = a
            elo = 1640
            
        else:
            k = random.choice([2, 3, 4])
            
            question = f"Evaluate the limit: lim_(x→0) (e^({k}x) - 1) / x"
            answer = k
            elo = 1700
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "calculus",
        "subtopic": "applications_of_derivatives",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))