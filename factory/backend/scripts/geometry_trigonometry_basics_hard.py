"""
geometry - trigonometry_basics (hard)
Generated: 2026-02-11T21:45:13.471901
"""

import random
import json
import sympy as sp
from sympy import sin, cos, tan, cot, sec, csc, sqrt, pi, simplify, atan, asin, acos

def generate_problem():
    problem_type = random.choice([
        'double_angle_solve',
        'compound_angle_exact',
        'trig_equation_multiple_solutions',
        'elevation_depression_multi_step',
        'law_of_sines_complex',
        'inverse_trig_composition',
        'trig_identity_simplify'
    ])
    
    if problem_type == 'double_angle_solve':
        # Find angle given double angle formula result
        # sin(2x) = value, solve for x in specific range
        answer_degrees = random.choice([15, 30, 45, 60, 75])
        answer_rad = sp.Rational(answer_degrees, 180) * pi
        double_value = simplify(sin(2 * answer_rad))
        
        question = f"Solve for $\\theta$ in degrees, where $0° < \\theta < 90°$: $\\sin(2\\theta) = {sp.latex(double_value)}$"
        answer = str(answer_degrees)
        difficulty = random.randint(1650, 1750)
        
    elif problem_type == 'compound_angle_exact':
        # Calculate exact value using compound angle formulas
        angle1 = random.choice([30, 45, 60])
        angle2 = random.choice([15, 30, 45])
        operation = random.choice(['sum', 'diff'])
        
        if operation == 'sum':
            total = angle1 + angle2
            question = f"Find the exact value of $\\sin({angle1}° + {angle2}°)$ using compound angle formulas."
            answer_val = simplify(sin(sp.Rational(total, 180) * pi))
        else:
            total = angle1 - angle2
            question = f"Find the exact value of $\\cos({angle1}° - {angle2}°)$ using compound angle formulas."
            answer_val = simplify(cos(sp.Rational(total, 180) * pi))
        
        answer = str(answer_val)
        difficulty = random.randint(1700, 1800)
        
    elif problem_type == 'trig_equation_multiple_solutions':
        # Equation like 2cos²(x) - 1 = 0 in [0, 360)
        coeff = random.choice([2, 4])
        const = coeff // 2
        
        question = f"Find all solutions for $\\theta$ in the range $[0°, 360°)$: ${coeff}\\cos^2(\\theta) - {const} = 0$. List all solutions separated by commas."
        # cos²(θ) = 1/2, so cos(θ) = ±√2/2
        # Solutions: 45, 135, 225, 315
        answer = "45,135,225,315"
        difficulty = random.randint(1750, 1850)
        
    elif problem_type == 'elevation_depression_multi_step':
        # Multi-step angle of elevation problem
        height = random.choice([50, 60, 80, 100])
        distance1 = random.choice([40, 50, 60])
        
        # Person at distance1 from building base, building height is height
        # Find angle of elevation
        angle_rad = atan(sp.Rational(height, distance1))
        angle_deg = simplify(angle_rad * 180 / pi)
        
        question = f"A building is {height} meters tall. An observer stands {distance1} meters from the base of the building. What is the angle of elevation from the observer to the top of the building? Round to the nearest degree."
        
        angle_numeric = float(angle_deg)
        answer = str(int(round(angle_numeric)))
        difficulty = random.randint(1600, 1700)
        
    elif problem_type == 'law_of_sines_complex':
        # Use law of sines with specific values
        angle_A = random.choice([30, 45, 60])
        side_a = random.choice([10, 12, 15, 20])
        angle_B = random.choice([45, 60, 75])
        
        if angle_A + angle_B >= 180:
            angle_B = random.choice([30, 45])
        
        # a/sin(A) = b/sin(B)
        # b = a * sin(B) / sin(A)
        A_rad = sp.Rational(angle_A, 180) * pi
        B_rad = sp.Rational(angle_B, 180) * pi
        
        side_b = simplify(side_a * sin(B_rad) / sin(A_rad))
        
        question = f"In triangle $ABC$, angle $A = {angle_A}°$, angle $B = {angle_B}°$, and side $a = {side_a}$. Find the length of side $b$ using the Law of Sines. Express your answer in exact form."
        answer = str(side_b)
        difficulty = random.randint(1650, 1750)
        
    elif problem_type == 'inverse_trig_composition':
        # Find exact value of composition like sin(arccos(3/5))
        denominator = 5
        numerator = random.choice([3, 4])
        
        # If cos(θ) = 3/5, find sin(θ)
        # sin²(θ) + cos²(θ) = 1
        # sin(θ) = √(1 - 9/25) = √(16/25) = 4/5
        
        cos_val = sp.Rational(numerator, denominator)
        sin_result = sqrt(1 - cos_val**2)
        
        question = f"Find the exact value of $\\sin\\left(\\arccos\\left(\\frac{{{numerator}}}{{{denominator}}}\\right)\\right)$."
        answer = str(sin_result)
        difficulty = random.randint(1700, 1800)
        
    else:  # trig_identity_simplify
        # Simplify using Pythagorean identities
        choice = random.randint(1, 2)
        
        if choice == 1:
            question = "Simplify: $\\frac{\\sin^2(\\theta)}{1 - \\cos(\\theta)}$ using trigonometric identities."
            # sin²(θ)/(1-cos(θ)) = (1-cos²(θ))/(1-cos(θ)) = 1+cos(θ)
            answer = "1 + cos(theta)"
            difficulty = random.randint(1750, 1850)
        else:
            question = "Simplify: $\\tan^2(\\theta) + 1$ using trigonometric identities."
            answer = "sec(theta)**2"
            difficulty = random.randint(1600, 1700)
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "trigonometry_basics",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))