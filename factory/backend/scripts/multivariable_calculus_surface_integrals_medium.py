"""
multivariable_calculus - surface_integrals (medium)
Generated: 2026-02-11T22:07:52.513341
"""

import sympy as sp
import random
import json

def generate_surface_integral_problem():
    x, y, z, u, v = sp.symbols('x y z u v', real=True)
    
    problem_type = random.choice(['scalar_field', 'vector_field_flux'])
    
    if problem_type == 'scalar_field':
        # Surface integral of scalar field - standard range 1300-1500
        choice = random.randint(1, 3)
        
        if choice == 1:
            # Plane surface, simple polynomial
            a = random.randint(1, 3)
            b = random.randint(1, 3)
            c = random.randint(1, 3)
            
            # Pick answer first
            answer = sp.Rational(random.randint(10, 50), random.randint(2, 6))
            
            # f(x,y,z) over z = c on rectangle [0,a] x [0,b]
            # Integral is just f*a*b, so f should be answer/(a*b)
            f_val = answer / (a * b)
            
            question = f"\\text{{Evaluate }} \\iint_S {sp.latex(f_val)} \\, dS \\text{{ where }} S \\text{{ is the surface }} z = {c} \\text{{ over the rectangle }} [0, {a}] \\times [0, {b}]"
            difficulty = 1300
            
        elif choice == 2:
            # Cylinder surface
            r = random.randint(2, 4)
            h = random.randint(2, 5)
            
            # x^2 + y^2 = r^2, 0 <= z <= h
            # Simple scalar field like z
            # Answer: integral of z over cylinder = pi*r*h^2
            answer = sp.pi * r * h**2
            
            question = f"\\text{{Evaluate }} \\iint_S z \\, dS \\text{{ where }} S \\text{{ is the lateral surface of cylinder }} x^2 + y^2 = {r**2}, \\, 0 \\le z \\le {h}"
            difficulty = 1450
            
        else:
            # Paraboloid z = x^2 + y^2 over disk
            r = random.randint(1, 2)
            
            # For z = x^2 + y^2, integrate constant over disk x^2+y^2 <= r^2
            # dS = sqrt(1 + 4x^2 + 4y^2) dA
            # Integrate 1: answer involves computation
            if r == 1:
                answer = sp.pi * (5 * sp.sqrt(5) - 1) / 6
            else:
                answer = sp.pi * ((1 + 4*r**2)**(sp.Rational(3,2)) - 1) / 6
            
            question = f"\\text{{Evaluate }} \\iint_S 1 \\, dS \\text{{ where }} S \\text{{ is the surface }} z = x^2 + y^2 \\text{{ over }} x^2 + y^2 \\le {r**2}"
            difficulty = 1550
    
    else:
        # Vector field flux through surface - standard range 1400-1600
        choice = random.randint(1, 3)
        
        if choice == 1:
            # Constant vector field through plane
            a = random.randint(1, 4)
            b = random.randint(1, 4)
            c = random.randint(1, 4)
            d = random.randint(2, 4)
            
            # F = <a, b, c> through z = 0 over square [0,d] x [0,d]
            # Normal is <0,0,1>, flux is c*d^2
            answer = c * d**2
            
            question = f"\\text{{Find the flux of }} \\mathbf{{F}} = \\langle {a}, {b}, {c} \\rangle \\text{{ through the surface }} z = 0 \\text{{ over }} [0, {d}] \\times [0, {d}] \\text{{ (upward normal)}}."
            difficulty = 1350
            
        elif choice == 2:
            # F = <x, y, z> through sphere (divergence theorem hint but direct)
            r = random.randint(1, 3)
            
            # Through sphere x^2+y^2+z^2=r^2, flux = 4*pi*r^3
            answer = 4 * sp.pi * r**3
            
            question = f"\\text{{Calculate the flux of }} \\mathbf{{F}} = \\langle x, y, z \\rangle \\text{{ through the sphere }} x^2 + y^2 + z^2 = {r**2} \\text{{ (outward normal)}}."
            difficulty = 1500
            
        else:
            # Simple field through explicit surface
            a = random.randint(1, 3)
            b = random.randint(2, 4)
            
            # F = <0, 0, z> through z = a over disk x^2+y^2 <= b^2
            # n = <0,0,1>, F.n = z = a, flux = a*pi*b^2
            answer = a * sp.pi * b**2
            
            question = f"\\text{{Compute the flux of }} \\mathbf{{F}} = \\langle 0, 0, {a} \\rangle \\text{{ through }} z = 0, \\, x^2 + y^2 \\le {b**2} \\text{{ (upward normal)}}."
            difficulty = 1400
    
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