"""
multivariable_calculus - surface_integrals (hard)
Generated: 2026-02-11T22:08:24.050301
"""

import sympy as sp
import random
import json

def generate_surface_integral_problem():
    x, y, z, u, v, t, s = sp.symbols('x y z u v t s', real=True)
    
    problem_type = random.choice([
        'parametric_scalar',
        'parametric_vector', 
        'graph_scalar',
        'flux_through_surface',
        'stokes_theorem'
    ])
    
    if problem_type == 'parametric_scalar':
        # Surface integral of scalar function over parametrized surface
        # Pick answer first
        answer = random.choice([sp.pi, 2*sp.pi, 4*sp.pi, sp.pi/2, 3*sp.pi])
        
        # Construct: cylinder, sphere portion, or cone
        surface_choice = random.choice(['cylinder', 'sphere', 'paraboloid'])
        
        if surface_choice == 'cylinder':
            radius = random.randint(1, 3)
            height = random.randint(1, 4)
            # r(u,v) = (R*cos(u), R*sin(u), v), 0 <= u <= 2π, 0 <= v <= h
            # f(x,y,z) chosen so integral gives clean answer
            # |r_u × r_v| = R
            # Answer = ∫∫ f dS, work backward: answer/(2π*h*R) = avg f
            avg_f = answer / (2*sp.pi*height*radius)
            if avg_f == 1:
                f_expr = "z"
                actual_answer = sp.pi * radius * height**2
            else:
                f_expr = "1"
                actual_answer = 2*sp.pi*radius*height
            
            question = f"\\text{{Evaluate }} \\iint_S {f_expr} \\, dS \\text{{ where }} S \\text{{ is the lateral surface of the cylinder }} x^2 + y^2 = {radius**2}, \\, 0 \\le z \\le {height}"
            answer_key = str(actual_answer)
            difficulty = 1650
            
        elif surface_choice == 'sphere':
            radius = random.randint(1, 3)
            # Hemisphere or full sphere
            portion = random.choice(['upper_hemisphere', 'full_sphere'])
            if portion == 'upper_hemisphere':
                # f = 1, answer = 2πR²
                question = f"\\text{{Compute }} \\iint_S dS \\text{{ where }} S \\text{{ is the upper hemisphere }} x^2 + y^2 + z^2 = {radius**2}, \\, z \\ge 0"
                answer_key = str(2*sp.pi*radius**2)
                difficulty = 1600
            else:
                # f = z², parametrize as spherical
                question = f"\\text{{Evaluate }} \\iint_S z^2 \\, dS \\text{{ where }} S \\text{{ is the sphere }} x^2 + y^2 + z^2 = {radius**2}"
                answer_key = str(sp.Rational(4,3)*sp.pi*radius**4)
                difficulty = 1750
        
        else:  # paraboloid
            a = random.randint(1, 2)
            bound = random.randint(1, 3)
            question = f"\\text{{Evaluate }} \\iint_S (x^2 + y^2) \\, dS \\text{{ where }} S \\text{{ is the portion of the paraboloid }} z = {a}(x^2 + y^2) \\text{{ with }} x^2 + y^2 \\le {bound**2}"
            # This requires computing |∇g| and setting up in polar
            answer_val = sp.pi * bound**2 * (bound**2 * (4*a**2*bound**2 + 1)**sp.Rational(3,2) - 1) / (6*a**2)
            answer_key = str(sp.simplify(sp.pi*bound**4/3 * sp.sqrt(4*a**2*bound**2 + 1)))
            difficulty = 1800
    
    elif problem_type == 'parametric_vector':
        # Surface integral with parametrization given
        radius = random.randint(1, 3)
        height = random.randint(1, 3)
        
        question = f"\\text{{Let }} S \\text{{ be parametrized by }} \\mathbf{{r}}(u,v) = \\langle {radius}\\cos(u), {radius}\\sin(u), v \\rangle, \\, 0 \\le u \\le 2\\pi, \\, 0 \\le v \\le {height}. \\text{{ Compute }} \\iint_S (x^2 + y^2) \\, dS"
        # x² + y² = R²
        # |r_u × r_v| = R
        answer_key = str(2*sp.pi*radius**3*height)
        difficulty = 1700
    
    elif problem_type == 'graph_scalar':
        # Surface as graph z = f(x,y)
        func_type = random.choice(['linear', 'quadratic'])
        
        if func_type == 'linear':
            a = random.randint(1, 3)
            b = random.randint(1, 3)
            bound = random.randint(1, 2)
            question = f"\\text{{Evaluate }} \\iint_S z \\, dS \\text{{ where }} S \\text{{ is the portion of the plane }} z = {a}x + {b}y \\text{{ over the square }} [0,{bound}] \\times [0,{bound}]"
            # dS = √(1 + a² + b²) dA
            # ∫∫ (ax + by) √(...) dA
            integrand_avg = a*bound/2 + b*bound/2
            answer_key = str(integrand_avg * bound**2 * sp.sqrt(1 + a**2 + b**2))
            difficulty = 1650
        else:
            question = f"\\text{{Compute }} \\iint_S 1 \\, dS \\text{{ where }} S \\text{{ is the part of the paraboloid }} z = x^2 + y^2 \\text{{ that lies under the plane }} z = 4"
            # Region: x² + y² ≤ 4
            # dS = √(1 + 4x² + 4y²) dA
            answer_key = str(sp.pi*(17*sp.sqrt(17) - 1)/6)
            difficulty = 1850
    
    elif problem_type == 'flux_through_surface':
        # Vector field flux
        field_choice = random.choice(['simple', 'radial'])
        
        if field_choice == 'simple':
            c = random.randint(1, 4)
            radius = random.randint(1, 2)
            question = f"\\text{{Find the flux of }} \\mathbf{{F}} = \\langle 0, 0, {c} \\rangle \\text{{ through the upper hemisphere }} x^2 + y^2 + z^2 = {radius**2}, \\, z \\ge 0, \\text{{ oriented upward}}"
            # Flux = c * (area of disk) = c * πR²
            answer_key = str(c*sp.pi*radius**2)
            difficulty = 1600
        else:
            radius = random.randint(1, 3)
            question = f"\\text{{Calculate the flux of }} \\mathbf{{F}} = \\langle x