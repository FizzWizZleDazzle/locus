"""
multivariable_calculus - vector_calculus (easy)
Generated: 2026-02-11T22:05:05.954199
"""

import sympy as sp
import random
import json

def generate_problem():
    x, y, z, t = sp.symbols('x y z t')
    
    problem_types = [
        'gradient_basic',
        'divergence_simple',
        'curl_simple',
        'line_integral_straight',
        'partial_derivative_vector',
        'dot_product_fields',
        'magnitude_vector_field'
    ]
    
    problem_type = random.choice(problem_types)
    
    if problem_type == 'gradient_basic':
        # Gradient of simple scalar functions (1000-1200)
        a = random.randint(1, 5)
        b = random.randint(1, 5)
        c = random.randint(1, 5)
        
        if random.choice([True, False]):
            # Linear function
            f = a*x + b*y
            grad_f = sp.Matrix([a, b])
            question = f"\\text{{Find }} \\nabla f \\text{{ where }} f(x,y) = {sp.latex(f)}"
            answer = sp.latex(grad_f)
            difficulty = random.randint(1000, 1100)
        else:
            # Quadratic terms
            f = a*x**2 + b*y**2
            grad_f = sp.Matrix([2*a*x, 2*b*y])
            question = f"\\text{{Find }} \\nabla f \\text{{ where }} f(x,y) = {sp.latex(f)}"
            answer = sp.latex(grad_f)
            difficulty = random.randint(1100, 1200)
    
    elif problem_type == 'divergence_simple':
        # Divergence of simple vector fields (1100-1250)
        a = random.randint(1, 5)
        b = random.randint(1, 5)
        c = random.randint(1, 5)
        
        if random.choice([True, False]):
            # Linear components
            F = sp.Matrix([a*x, b*y, c*z])
            div_F = a + b + c
            question = f"\\text{{Find }} \\nabla \\cdot \\mathbf{{F}} \\text{{ where }} \\mathbf{{F}}(x,y,z) = {sp.latex(F)}"
            answer = str(div_F)
            difficulty = random.randint(1050, 1150)
        else:
            # Simple polynomial
            F = sp.Matrix([a*x**2, b*y, c*z])
            div_F = 2*a*x + b + c
            question = f"\\text{{Find }} \\nabla \\cdot \\mathbf{{F}} \\text{{ where }} \\mathbf{{F}}(x,y,z) = {sp.latex(F)}"
            answer = sp.latex(div_F)
            difficulty = random.randint(1150, 1250)
    
    elif problem_type == 'curl_simple':
        # Curl of simple 2D vector fields (1100-1300)
        a = random.randint(1, 5)
        b = random.randint(1, 5)
        
        if random.choice([True, False]):
            # Conservative field (curl = 0)
            F = sp.Matrix([a*x, b*y, 0])
            curl_F = sp.Matrix([0, 0, 0])
            question = f"\\text{{Find }} \\nabla \\times \\mathbf{{F}} \\text{{ where }} \\mathbf{{F}}(x,y,z) = {sp.latex(F)}"
            answer = sp.latex(curl_F)
            difficulty = random.randint(1100, 1200)
        else:
            # Non-zero curl
            F = sp.Matrix([a*y, b*x, 0])
            curl_F = sp.Matrix([0, 0, b - a])
            question = f"\\text{{Find }} \\nabla \\times \\mathbf{{F}} \\text{{ where }} \\mathbf{{F}}(x,y,z) = {sp.latex(F)}"
            answer = sp.latex(curl_F)
            difficulty = random.randint(1200, 1300)
    
    elif problem_type == 'line_integral_straight':
        # Line integral along straight line (1200-1300)
        a = random.randint(1, 4)
        b = random.randint(1, 4)
        c = random.randint(1, 3)
        
        # F = (a, b) along line from (0,0) to (c, 0)
        F = sp.Matrix([a, b])
        # r(t) = (ct, 0), t from 0 to 1
        # dr = (c, 0)dt
        result = a*c
        
        question = f"\\text{{Evaluate }} \\int_C \\mathbf{{F}} \\cdot d\\mathbf{{r}} \\text{{ where }} \\mathbf{{F}} = {sp.latex(F)} \\text{{ and }} C \\text{{ is the line segment from }} (0,0) \\text{{ to }} ({c},0)"
        answer = str(result)
        difficulty = random.randint(1200, 1300)
    
    elif problem_type == 'partial_derivative_vector':
        # Partial derivative of vector field component (1000-1150)
        a = random.randint(1, 5)
        b = random.randint(1, 5)
        
        if random.choice([True, False]):
            # Linear
            expr = a*x + b*y
            deriv = a
            var = 'x'
            question = f"\\text{{Find }} \\frac{{\\partial}}{{\\partial x}} ({sp.latex(expr)})"
            answer = str(deriv)
            difficulty = random.randint(1000, 1050)
        else:
            # Quadratic
            expr = a*x**2 + b*y
            deriv = 2*a*x
            var = 'x'
            question = f"\\text{{Find }} \\frac{{\\partial}}{{\\partial x}} ({sp.latex(expr)})"
            answer = sp.latex(deriv)
            difficulty = random.randint(1050, 1150)
    
    elif problem_type == 'dot_product_fields':
        # Dot product of two vector fields at a point (1100-1250)
        a = random.randint(1, 4)
        b = random.randint(1, 4)
        c = random.randint(1, 4)
        d = random.randint(1, 4)
        
        F = sp.Matrix([a, b])
        G = sp.Matrix([c, d])
        dot_prod = a*c + b*d
        
        question = f"\\text{{Find }} \\mathbf{{F}} \\cdot \\mathbf{{G}} \\text{{ where }} \\mathbf{{F}} = {sp.latex(F)} \\text{{ and }} \\mathbf{{G}} = {sp.latex(G)}"
        answer = str(dot_prod)
        difficulty = random.randint(1100, 1200)
    
    else:  # magnitude_vector_field
        # Magnitude of vector field at point (1050-1200)
        a = random.randint(1, 4)
        b = random.randint(1, 4)
        
        F = sp.Matrix([a, b])
        mag = sp.sqrt(a**2 + b**2)
        
        question = f"\\text{{Find }} |\\mathbf{{F}}| \\text{{ where }} \\mathbf{{F}} = {sp.