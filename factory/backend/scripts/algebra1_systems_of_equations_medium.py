"""
algebra1 - systems_of_equations (medium)
Generated: 2026-02-11T21:34:51.375537
"""

import sympy as sp
import random
import json

def generate_system_of_equations():
    x, y = sp.symbols('x y')
    
    # Pick clean integer solutions first
    x_sol = random.randint(-5, 5)
    y_sol = random.randint(-5, 5)
    
    # Choose difficulty level within standard range
    difficulty = random.choice([1300, 1350, 1400, 1450, 1500, 1550, 1600])
    
    if difficulty <= 1350:
        # Simple substitution or elimination (2-3 steps)
        # Form: x + y = sum, x - y = diff or ax + by = c, dx + ey = f with nice coefficients
        a = random.choice([1, 2])
        b = random.choice([1, 2])
        c = a * x_sol + b * y_sol
        
        d = random.choice([1, 2, 3])
        e = random.choice([1, 2, 3])
        f = d * x_sol + e * y_sol
        
        question = f"\\text{{Solve the system: }} \\begin{{cases}} {sp.latex(a*x + b*y)} = {c} \\\\ {sp.latex(d*x + e*y)} = {f} \\end{{cases}}"
        
    elif difficulty <= 1500:
        # Requires distribution or more complex coefficients (3-4 steps)
        a = random.randint(2, 4)
        b = random.randint(2, 4)
        c = a * x_sol + b * y_sol
        
        d = random.randint(2, 5)
        e = random.randint(2, 5)
        # Ensure not proportional
        if a * e == b * d:
            e += 1
        f = d * x_sol + e * y_sol
        
        question = f"\\text{{Solve the system: }} \\begin{{cases}} {sp.latex(a*x + b*y)} = {c} \\\\ {sp.latex(d*x + e*y)} = {f} \\end{{cases}}"
        
    else:
        # More complex coefficients or requires careful elimination (4-5 steps)
        a = random.randint(3, 6)
        b = random.randint(3, 6)
        c = a * x_sol + b * y_sol
        
        d = random.randint(3, 7)
        e = random.randint(3, 7)
        # Ensure not proportional
        if a * e == b * d:
            e += 2
        f = d * x_sol + e * y_sol
        
        question = f"\\text{{Solve the system: }} \\begin{{cases}} {sp.latex(a*x + b*y)} = {c} \\\\ {sp.latex(d*x + e*y)} = {f} \\end{{cases}}"
    
    # Answer as ordered pair
    answer = f"({x_sol}, {y_sol})"
    
    result = {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "systems_of_equations",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_system_of_equations()
print(json.dumps(problem))