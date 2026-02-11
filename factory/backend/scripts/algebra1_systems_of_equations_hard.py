"""
algebra1 - systems_of_equations (hard)
Generated: 2026-02-11T21:35:44.938515
"""

import random
import json
from sympy import symbols, Eq, solve, latex, simplify

def generate_hard_system_problem():
    x, y, z = symbols('x y z')
    
    problem_type = random.choice([
        'three_variable_system',
        'word_problem_mixture',
        'word_problem_motion',
        'parametric_system',
        'system_with_fractions'
    ])
    
    if problem_type == 'three_variable_system':
        x_val = random.randint(-5, 5)
        y_val = random.randint(-5, 5)
        z_val = random.randint(-5, 5)
        
        a1, b1, c1 = random.randint(1, 4), random.randint(1, 4), random.randint(1, 4)
        a2, b2, c2 = random.randint(1, 4), random.randint(-3, 3), random.randint(1, 4)
        a3, b3, c3 = random.randint(1, 3), random.randint(1, 3), random.randint(-3, 3)
        
        d1 = a1 * x_val + b1 * y_val + c1 * z_val
        d2 = a2 * x_val + b2 * y_val + c2 * z_val
        d3 = a3 * x_val + b3 * y_val + c3 * z_val
        
        question = f"\\text{{Solve the system of equations:}} \\\\ {a1}x + {b1}y + {c1}z = {d1} \\\\ {a2}x + {b2}y + {c2}z = {d2} \\\\ {a3}x + {a3}y + {c3}z = {d3}"
        answer = f"({x_val}, {y_val}, {z_val})"
        difficulty = random.randint(1700, 1850)
        
    elif problem_type == 'word_problem_mixture':
        sol_a = random.randint(10, 30)
        sol_b = random.randint(10, 30)
        
        conc_a = random.choice([10, 15, 20, 25, 30])
        conc_b = random.choice([40, 50, 60, 70])
        total_vol = sol_a + sol_b
        total_conc = random.randint(25, 45)
        
        required_amount = (conc_a * sol_a + conc_b * sol_b) / 100
        
        question = f"\\text{{A chemist needs to create {total_vol} liters of a {total_conc}\\% acid solution by mixing}} \\\\ \\text{{a {conc_a}\\% acid solution with a {conc_b}\\% acid solution. How many liters of each}} \\\\ \\text{{solution should be mixed? Let }} x \\text{{ be liters of {conc_a}\\% solution and }} y \\text{{ be liters}} \\\\ \\text{{of {conc_b}\\% solution.}}"
        answer = f"({sol_a}, {sol_b})"
        difficulty = random.randint(1650, 1800)
        
    elif problem_type == 'word_problem_motion':
        speed1 = random.randint(40, 70)
        speed2 = random.randint(30, 60)
        time_val = random.randint(2, 5)
        distance = speed1 * time_val + speed2 * time_val
        
        question = f"\\text{{Two cars start from the same point and travel in opposite directions.}} \\\\ \\text{{One car travels at {speed1} mph and the other at {speed2} mph.}} \\\\ \\text{{After how many hours will they be {distance} miles apart?}} \\\\ \\text{{Set up and solve the system where }} t \\text{{ is time in hours.}}"
        answer = f"{time_val}"
        difficulty = random.randint(1600, 1750)
        
    elif problem_type == 'parametric_system':
        x_val = random.randint(2, 8)
        y_val = random.randint(2, 8)
        
        a = random.randint(2, 5)
        b = random.randint(2, 5)
        c = random.randint(2, 4)
        d = random.randint(2, 4)
        
        rhs1 = a * x_val + b * y_val
        rhs2 = c * x_val - d * y_val
        
        question = f"\\text{{Solve the system:}} \\\\ {a}x + {b}y = {rhs1} \\\\ {c}x - {d}y = {rhs2}"
        answer = f"({x_val}, {y_val})"
        difficulty = random.randint(1600, 1750)
        
    else:  # system_with_fractions
        x_val = random.randint(2, 6)
        y_val = random.randint(2, 6)
        
        num1, den1 = random.randint(1, 3), random.randint(2, 4)
        num2, den2 = random.randint(1, 3), random.randint(2, 4)
        num3, den3 = random.randint(1, 2), random.randint(2, 3)
        num4, den4 = random.randint(1, 2), random.randint(2, 3)
        
        rhs1 = num1/den1 * x_val + num2/den2 * y_val
        rhs2 = num3/den3 * x_val - num4/den4 * y_val
        
        question = f"\\text{{Solve the system:}} \\\\ \\frac{{{num1}}}{{{den1}}}x + \\frac{{{num2}}}{{{den2}}}y = {rhs1} \\\\ \\frac{{{num3}}}{{{den3}}}x - \\frac{{{num4}}}{{{den4}}}y = {rhs2}"
        answer = f"({x_val}, {y_val})"
        difficulty = random.randint(1700, 1900)
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "systems_of_equations",
        "grading_mode": "equivalent"
    }

problem = generate_hard_system_problem()
print(json.dumps(problem))