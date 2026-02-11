"""
multivariable_calculus - line_integrals (medium)
Generated: 2026-02-11T22:06:36.940447
"""

import sympy as sp
import random
import json

def generate_line_integral_problem():
    t = sp.Symbol('t')
    x, y, z = sp.symbols('x y z')
    
    problem_type = random.choice(['scalar_field_2d', 'vector_field_2d', 'scalar_field_3d', 'vector_field_3d'])
    
    if problem_type == 'scalar_field_2d':
        a = random.randint(0, 2)
        b = random.randint(1, 3)
        
        answer = random.randint(2, 20)
        
        if random.choice([True, False]):
            coeff = answer / (b - a)
            f_expr = coeff
            curve_x = t
            curve_y = sp.Integer(0)
            
            question = f"Evaluate the line integral \\int_C {sp.latex(f_expr)} \\, ds where C is the line segment from ({a}, 0) to ({b}, 0)."
            difficulty = 1300
        else:
            m = random.randint(1, 3)
            length = sp.sqrt(1 + m**2) * (b - a)
            coeff = answer / length
            f_expr = coeff
            curve_x = t
            curve_y = m * t
            
            question = f"Evaluate the line integral \\int_C {sp.latex(f_expr)} \\, ds where C is the line segment from ({a}, {m*a}) to ({b}, {m*b})."
            difficulty = 1350
    
    elif problem_type == 'vector_field_2d':
        a = random.randint(0, 2)
        b = random.randint(2, 4)
        
        if random.choice([True, False]):
            k = random.randint(1, 4)
            answer = k * (b**2 - a**2) / 2
            
            P = k * y
            Q = sp.Integer(0)
            curve_x = t
            curve_y = t
            
            question = f"Evaluate the line integral \\int_C {sp.latex(P)} \\, dx + {sp.latex(Q)} \\, dy where C is parameterized by x = t, y = t for {a} \\leq t \\leq {b}."
            difficulty = 1400
        else:
            k = random.randint(1, 3)
            R = random.randint(1, 3)
            answer = 2 * sp.pi * R**2 * k
            
            P = -k * y
            Q = k * x
            
            question = f"Evaluate the line integral \\int_C {sp.latex(P)} \\, dx + {sp.latex(Q)} \\, dy where C is the circle x^2 + y^2 = {R**2} traversed counterclockwise."
            difficulty = 1550
    
    elif problem_type == 'scalar_field_3d':
        a = random.randint(0, 1)
        b = random.randint(2, 4)
        
        if random.choice([True, False]):
            k = random.randint(1, 3)
            answer = k * (b - a)
            
            f_expr = k
            curve_x = t
            curve_y = sp.Integer(0)
            curve_z = sp.Integer(0)
            
            question = f"Evaluate the line integral \\int_C {sp.latex(f_expr)} \\, ds where C is the line segment from ({a}, 0, 0) to ({b}, 0, 0)."
            difficulty = 1320
        else:
            k = random.randint(1, 3)
            R = random.randint(1, 3)
            answer = 2 * sp.pi * R * k
            
            f_expr = k
            
            question = f"Evaluate the line integral \\int_C {sp.latex(f_expr)} \\, ds where C is the circle x^2 + y^2 = {R**2}, z = 0 in the xy-plane."
            difficulty = 1450
    
    else:
        a = random.randint(0, 1)
        b = random.randint(2, 4)
        
        if random.choice([True, False]):
            k = random.randint(1, 4)
            answer = k * (b**3 - a**3) / 3
            
            P = k * x**2
            Q = sp.Integer(0)
            R = sp.Integer(0)
            
            question = f"Evaluate the line integral \\int_C {sp.latex(P)} \\, dx + {sp.latex(Q)} \\, dy + {sp.latex(R)} \\, dz where C is parameterized by x = t, y = 0, z = 0 for {a} \\leq t \\leq {b}."
            difficulty = 1420
        else:
            k = random.randint(1, 3)
            answer = k * (b**2 - a**2)
            
            P = sp.Integer(0)
            Q = sp.Integer(0)
            R = k * z
            
            question = f"Evaluate the line integral \\int_C {sp.latex(P)} \\, dx + {sp.latex(Q)} \\, dy + {sp.latex(R)} \\, dz where C is parameterized by x = 0, y = 0, z = t for {a} \\leq t \\leq {b}."
            difficulty = 1380
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "multivariable_calculus",
        "subtopic": "line_integrals",
        "grading_mode": "equivalent"
    }

problem = generate_line_integral_problem()
print(json.dumps(problem))