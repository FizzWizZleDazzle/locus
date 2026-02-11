"""
algebra1 - systems_of_equations (easy)
Generated: 2026-02-11T21:29:54.312195
"""

import sympy as sp
import random
import json

def generate_system_of_equations():
    x, y = sp.symbols('x y')
    
    # Pick clean integer solutions first
    x_val = random.randint(-5, 5)
    y_val = random.randint(-5, 5)
    
    # Determine difficulty level (1000-1300 range for easier problems)
    difficulty_choice = random.choice([1050, 1100, 1150, 1200, 1250, 1300])
    
    if difficulty_choice <= 1150:
        # Very simple: one equation already solved, substitute into second
        # Example: x = 3, 2x + y = 10
        a2 = random.choice([1, 2, 3])
        b2 = random.choice([1, 2, 3])
        c2 = a2 * x_val + b2 * y_val
        
        eq1_latex = f"x = {x_val}"
        eq2_latex = f"{a2}x + {b2}y = {c2}" if b2 != 1 else f"{a2}x + y = {c2}"
        if a2 == 1:
            eq2_latex = f"x + {b2}y = {c2}" if b2 != 1 else f"x + y = {c2}"
        
        question_latex = f"\\text{{Solve the system: }} \\begin{{cases}} {eq1_latex} \\\\ {eq2_latex} \\end{{cases}}"
        actual_difficulty = 1100
        
    elif difficulty_choice <= 1250:
        # Simple elimination or substitution: small coefficients, one variable has coefficient 1
        # Example: x + y = 5, x - y = 1
        b1 = random.choice([1, -1])
        c1 = x_val + b1 * y_val
        
        b2 = random.choice([1, -1, 2, -2])
        c2 = x_val + b2 * y_val
        
        eq1_latex = f"x + y = {c1}" if b1 == 1 else f"x - y = {c1}"
        if b2 == 1:
            eq2_latex = f"x + y = {c2}"
        elif b2 == -1:
            eq2_latex = f"x - y = {c2}"
        elif b2 == 2:
            eq2_latex = f"x + 2y = {c2}"
        else:
            eq2_latex = f"x - 2y = {c2}"
        
        question_latex = f"\\text{{Solve the system: }} \\begin{{cases}} {eq1_latex} \\\\ {eq2_latex} \\end{{cases}}"
        actual_difficulty = 1200
        
    else:
        # Two-step elimination: small coefficients but need to multiply
        # Example: 2x + y = 7, x + y = 4
        a1 = random.choice([1, 2, 3])
        b1 = random.choice([1, 2])
        c1 = a1 * x_val + b1 * y_val
        
        a2 = random.choice([1, 2, 3])
        b2 = random.choice([1, 2])
        c2 = a2 * x_val + b2 * y_val
        
        # Make sure they're not the same equation
        if (a1, b1, c1) == (a2, b2, c2):
            b2 = b2 + 1
            c2 = a2 * x_val + b2 * y_val
        
        def format_term(coef, var, is_first=False):
            if coef == 1:
                return var if is_first else f"+ {var}"
            elif coef == -1:
                return f"-{var}" if is_first else f"- {var}"
            elif coef > 0:
                return f"{coef}{var}" if is_first else f"+ {coef}{var}"
            else:
                return f"{coef}{var}" if is_first else f"- {abs(coef)}{var}"
        
        eq1_latex = f"{format_term(a1, 'x', True)} {format_term(b1, 'y').strip()} = {c1}"
        eq2_latex = f"{format_term(a2, 'x', True)} {format_term(b2, 'y').strip()} = {c2}"
        
        question_latex = f"\\text{{Solve the system: }} \\begin{{cases}} {eq1_latex} \\\\ {eq2_latex} \\end{{cases}}"
        actual_difficulty = 1300
    
    # Answer as ordered pair
    answer_key = f"({x_val}, {y_val})"
    
    return {
        "question_latex": question_latex,
        "answer_key": answer_key,
        "difficulty": actual_difficulty,
        "main_topic": "algebra1",
        "subtopic": "systems_of_equations",
        "grading_mode": "equivalent"
    }

problem = generate_system_of_equations()
print(json.dumps(problem))