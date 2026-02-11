"""
multivariable_calculus - surface_integrals (easy)
Generated: 2026-02-11T22:07:28.144576
"""

import sympy as sp
import random
import json

def generate_surface_integral_problem():
    x, y, z, u, v = sp.symbols('x y z u v', real=True)
    
    problem_type = random.choice([
        'scalar_field_flat',
        'vector_field_flat',
        'scalar_field_cylinder',
        'scalar_field_sphere_patch'
    ])
    
    if problem_type == 'scalar_field_flat':
        # Easiest: surface integral of scalar field over flat region in xy-plane
        # Pick answer first
        answer = random.choice([4, 6, 8, 9, 12, 16, 18])
        
        # Work backwards: answer = ∫∫ f(x,y) dA over rectangle [0,a]×[0,b]
        a = random.randint(1, 3)
        b = random.randint(1, 3)
        area = a * b
        
        # Choose constant function
        c = answer // area
        
        if answer % area == 0:
            f = c
            x_range = f"0 \\leq x \\leq {a}"
            y_range = f"0 \\leq y \\leq {b}"
            
            question = f"\\int \\int_S {c} \\, dS \\text{{ where }} S \\text{{ is the flat surface }} z=0 \\text{{ with }} {x_range}, {y_range}"
            
            difficulty = 1050
        else:
            # Use linear function
            answer = random.choice([6, 8, 12])
            a = 2
            b = 2
            if answer == 6:
                f_expr = x + y
                f_int = sp.integrate(sp.integrate(f_expr, (x, 0, a)), (y, 0, b))
                answer = int(f_int)
                question = f"\\int \\int_S (x+y) \\, dS \\text{{ where }} S \\text{{ is the flat surface }} z=0 \\text{{ with }} 0 \\leq x \\leq {a}, 0 \\leq y \\leq {b}"
            else:
                f_expr = 2*x
                f_int = sp.integrate(sp.integrate(f_expr, (x, 0, a)), (y, 0, b))
                answer = int(f_int)
                question = f"\\int \\int_S 2x \\, dS \\text{{ where }} S \\text{{ is the flat surface }} z=0 \\text{{ with }} 0 \\leq x \\leq {a}, 0 \\leq y \\leq {b}"
            
            difficulty = 1150
            
    elif problem_type == 'vector_field_flat':
        # Vector field flux through flat surface (simple dot product)
        # Pick answer first
        answer = random.choice([8, 12, 16, 20])
        
        a = random.randint(2, 4)
        b = random.randint(2, 4)
        area = a * b
        
        k_val = answer // area
        
        if answer % area == 0:
            question = f"\\int \\int_S \\mathbf{{F}} \\cdot d\\mathbf{{S}} \\text{{ where }} \\mathbf{{F}} = \\langle 0, 0, {k_val} \\rangle \\text{{ and }} S \\text{{ is }} z=0, 0 \\leq x \\leq {a}, 0 \\leq y \\leq {b} \\text{{ with upward normal}}"
            difficulty = 1100
        else:
            answer = 8
            a = 2
            b = 2
            question = f"\\int \\int_S \\mathbf{{F}} \\cdot d\\mathbf{{S}} \\text{{ where }} \\mathbf{{F}} = \\langle x, y, 2 \\rangle \\text{{ and }} S \\text{{ is }} z=0, 0 \\leq x \\leq {a}, 0 \\leq y \\leq {b} \\text{{ with upward normal}}"
            difficulty = 1200
            
    elif problem_type == 'scalar_field_cylinder':
        # Scalar field over part of cylinder (parametrized surface)
        # Pick answer first
        answer = random.choice([2, 4, 6, 8])
        
        r = random.randint(1, 2)
        h = random.randint(1, 3)
        
        # For constant function over cylinder lateral surface
        # Surface area = 2πrh, so c = answer / (2πrh)
        if answer == 2 and r == 1 and h == 1:
            c = 1
            question = f"\\int \\int_S 1 \\, dS \\text{{ where }} S \\text{{ is the lateral surface }} x^2+y^2={r}^2, 0 \\leq z \\leq {h}"
            answer_key = f"2*pi*{r}*{h}"
            answer = sp.simplify(sp.sympify(answer_key))
            difficulty = 1250
        else:
            r = 1
            h = 2
            question = f"\\int \\int_S z \\, dS \\text{{ where }} S \\text{{ is the lateral surface }} x^2+y^2={r}^2, 0 \\leq z \\leq {h}"
            answer = sp.sympify("4*pi")
            difficulty = 1300
            
    else:  # scalar_field_sphere_patch
        # Simple sphere patch
        answer = random.choice([4, 8, 16])
        radius = random.randint(1, 2)
        
        if answer == 4 and radius == 1:
            question = f"\\int \\int_S 1 \\, dS \\text{{ where }} S \\text{{ is the upper hemisphere }} x^2+y^2+z^2={radius}^2, z \\geq 0"
            answer = sp.sympify("2*pi")
            difficulty = 1280
        else:
            radius = 2
            question = f"\\int \\int_S 1 \\, dS \\text{{ where }} S \\text{{ is the upper hemisphere }} x^2+y^2+z^2={radius}^2, z \\geq 0"
            answer = sp.sympify("8*pi")
            difficulty = 1280
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "multivariable_calculus",
        "subtopic": "surface_integrals",
        "grading_mode": "equivalent"
    }

problem = generate_surface_integral_problem()
print(json.dumps(problem))