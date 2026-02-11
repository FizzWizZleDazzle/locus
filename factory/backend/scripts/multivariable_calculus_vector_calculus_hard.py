"""
multivariable_calculus - vector_calculus (hard)
Generated: 2026-02-11T22:05:52.137677
"""

import sympy as sp
import random
import json

def generate_vector_calculus_problem():
    x, y, z, t, u, v = sp.symbols('x y z t u v', real=True)
    
    problem_type = random.choice([
        'line_integral_vector_field',
        'surface_integral',
        'divergence_theorem',
        'stokes_theorem',
        'curl_computation',
        'gradient_field_potential',
        'flux_integral'
    ])
    
    if problem_type == 'line_integral_vector_field':
        a = random.choice([1, 2, 3])
        b = random.choice([1, 2, 3])
        c = random.choice([1, 2])
        
        answer = random.choice([sp.pi, 2*sp.pi, sp.pi/2, 0, sp.Rational(1,2), sp.Rational(1,3)])
        
        if answer == 0:
            F1 = y
            F2 = -x
            path_desc = "the circle x^2 + y^2 = " + str(a**2) + " traversed counterclockwise"
            question = f"\\int_C \\mathbf{{F}} \\cdot d\\mathbf{{r}} \\text{{ where }} \\mathbf{{F}} = {sp.latex(F1)}\\mathbf{{i}} + {sp.latex(F2)}\\mathbf{{j}} \\text{{ and }} C \\text{{ is {path_desc}}}"
            difficulty = 1650
        else:
            F1 = -a*y
            F2 = b*x
            F3 = 0
            r = c
            curl_F = b + a
            area = sp.pi * r**2
            answer = curl_F * area
            
            path_desc = f"the circle x^2 + y^2 = {r**2} in the xy-plane, traversed counterclockwise"
            question = f"\\text{{Evaluate }} \\int_C \\mathbf{{F}} \\cdot d\\mathbf{{r}} \\text{{ where }} \\mathbf{{F}} = {sp.latex(F1)}\\mathbf{{i}} + {sp.latex(F2)}\\mathbf{{j}} \\text{{ and }} C \\text{{ is {path_desc}}}"
            difficulty = 1700
    
    elif problem_type == 'surface_integral':
        a = random.choice([1, 2, 3])
        h = random.choice([1, 2, 3, 4])
        
        answer = a**2 * h * sp.pi
        
        question = f"\\text{{Find the flux of }} \\mathbf{{F}} = x\\mathbf{{i}} + y\\mathbf{{j}} + z\\mathbf{{k}} \\text{{ through the lateral surface of the cylinder }} x^2 + y^2 = {a**2}, \\ 0 \\le z \\le {h}, \\text{{ oriented outward}}"
        difficulty = 1750
    
    elif problem_type == 'divergence_theorem':
        a = random.choice([1, 2, 3])
        
        div_F = 3
        volume = sp.Rational(4, 3) * sp.pi * a**3
        answer = div_F * volume
        
        question = f"\\text{{Use the Divergence Theorem to evaluate }} \\iint_S \\mathbf{{F}} \\cdot d\\mathbf{{S}} \\text{{ where }} \\mathbf{{F}} = x\\mathbf{{i}} + y\\mathbf{{j}} + z\\mathbf{{k}} \\text{{ and }} S \\text{{ is the sphere }} x^2 + y^2 + z^2 = {a**2} \\text{{ oriented outward}}"
        difficulty = 1800
    
    elif problem_type == 'stokes_theorem':
        a = random.choice([1, 2, 3])
        b = random.choice([1, 2, 3])
        c = random.choice([1, 2])
        
        curl_z = b - (-a)
        area = sp.pi * c**2
        answer = curl_z * area
        
        question = f"\\text{{Use Stokes' Theorem to evaluate }} \\int_C \\mathbf{{F}} \\cdot d\\mathbf{{r}} \\text{{ where }} \\mathbf{{F}} = {-a}y\\mathbf{{i}} + {b}x\\mathbf{{j}} + z\\mathbf{{k}} \\text{{ and }} C \\text{{ is the circle }} x^2 + y^2 = {c**2}, z = 0, \\text{{ oriented counterclockwise when viewed from above}}"
        difficulty = 1850
    
    elif problem_type == 'curl_computation':
        a = random.choice([1, 2, 3])
        b = random.choice([1, 2, 3])
        c = random.choice([1, 2, 3])
        
        F1 = a*y*z
        F2 = b*x*z
        F3 = c*x*y
        
        curl_x = c*x - b*x
        curl_y = c*y - a*y
        curl_z = b*z - a*z
        
        curl_1 = c - b
        curl_2 = c - a
        curl_3 = b - a
        
        answer_vec = [curl_1, curl_2, curl_3]
        answer = sp.Matrix(answer_vec)
        
        question = f"\\text{{Find }} \\nabla \\times \\mathbf{{F}} \\text{{ where }} \\mathbf{{F}} = {sp.latex(F1)}\\mathbf{{i}} + {sp.latex(F2)}\\mathbf{{j}} + {sp.latex(F3)}\\mathbf{{k}}"
        difficulty = 1650
    
    elif problem_type == 'gradient_field_potential':
        a = random.choice([1, 2, 3])
        b = random.choice([1, 2, 3])
        
        f = a*x**2 + b*y**2 + x*y
        
        grad_f = [sp.diff(f, x), sp.diff(f, y)]
        
        x1, y1 = random.choice([0, 1]), random.choice([0, 1])
        x2, y2 = random.choice([1, 2]), random.choice([1, 2])
        
        answer = f.subs([(x, x2), (y, y2)]) - f.subs([(x, x1), (y, y1)])
        
        F1 = sp.diff(f, x)
        F2 = sp.diff(f, y)
        
        question = f"\\text{{Evaluate }} \\int_C \\mathbf{{F}} \\cdot d\\mathbf{{r}} \\text{{ where }} \\mathbf{{F}} = ({sp.latex(F1)})\\mathbf{{i}} + ({sp.latex(F2)})\\mathbf{{j}} \\text{{ and }} C \\text{{ is any path from }} ({x1}, {y1}) \\text{{ to }} ({x2}, {y2})"
        difficulty = 1700
    
    else:  # flux_integral
        a = random.choice([1, 2])
        b = random.choice([1, 2])
        
        answer = a * b**2 * sp.pi
        
        question = f"\\text{{Calculate the flux of }} \\mathbf{{F}} = {a}z\\mathbf{{k}} \\text{{ upward