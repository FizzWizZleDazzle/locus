"""
multivariable_calculus - line_integrals (hard)
Generated: 2026-02-11T22:07:02.281500
"""

import sympy as sp
import random
import json

def generate_line_integral_problem():
    x, y, z, t = sp.symbols('x y z t', real=True)
    
    problem_type = random.choice([
        'scalar_field_2d',
        'vector_field_2d', 
        'vector_field_3d',
        'conservative_field',
        'work_against_force'
    ])
    
    if problem_type == 'scalar_field_2d':
        # Line integral of scalar field: ∫ f(x,y) ds
        # Choose answer first
        answer = random.choice([sp.Rational(i, j) for i in range(1, 20) for j in range(1, 8)] + list(range(1, 15)))
        
        # Create parametric curve: circle, line segment, or parabola
        curve_type = random.choice(['circle', 'line', 'parabola'])
        
        if curve_type == 'circle':
            radius = random.randint(1, 4)
            # x = r*cos(t), y = r*sin(t), t: 0 to 2π
            # ds = r dt
            # Pick f such that integral works out
            coeffs = [random.randint(-3, 3) for _ in range(3)]
            f = coeffs[0] + coeffs[1]*x + coeffs[2]*y
            
            # Substitute parametrization
            x_param = radius * sp.cos(t)
            y_param = radius * sp.sin(t)
            f_param = f.subs([(x, x_param), (y, y_param)])
            ds = radius
            
            integrand = f_param * ds
            # Adjust coefficients to get desired answer
            actual_integral = sp.integrate(integrand, (t, 0, 2*sp.pi))
            
            if actual_integral != 0:
                scale = answer / actual_integral
                f = sp.simplify(f * scale)
            
            question = f"\\int_C {sp.latex(f)} \\, ds \\text{{ where }} C \\text{{ is the circle }} x^2 + y^2 = {radius**2} \\text{{ traversed counterclockwise}}"
            difficulty = 1650
            
        elif curve_type == 'line':
            # Line from (0,0) to (a,b)
            a, b = random.randint(1, 5), random.randint(1, 5)
            # x = at, y = bt, t: 0 to 1
            # ds = sqrt(a^2 + b^2) dt
            
            c1, c2 = random.randint(-2, 2), random.randint(-2, 2)
            f = c1*x + c2*y
            
            x_param = a*t
            y_param = b*t
            f_param = f.subs([(x, x_param), (y, y_param)])
            ds = sp.sqrt(a**2 + b**2)
            
            integrand = f_param * ds
            actual_integral = sp.integrate(integrand, (t, 0, 1))
            
            if actual_integral != 0 and actual_integral.is_number:
                scale = answer / actual_integral
                f = sp.simplify(f * scale)
                answer = sp.simplify(answer)
            else:
                answer = actual_integral
            
            question = f"\\int_C {sp.latex(f)} \\, ds \\text{{ where }} C \\text{{ is the line segment from }} (0,0) \\text{{ to }} ({a},{b})"
            difficulty = 1600
            
        else:  # parabola
            a = random.randint(1, 3)
            # y = ax^2, x: 0 to 1
            # Parametrize: x=t, y=at^2
            # ds = sqrt(1 + (2at)^2) dt
            
            f = x + y
            x_param = t
            y_param = a*t**2
            f_param = f.subs([(x, x_param), (y, y_param)])
            ds = sp.sqrt(1 + (2*a*t)**2)
            
            integrand = f_param * ds
            answer = sp.integrate(integrand, (t, 0, 1))
            
            question = f"\\int_C (x+y) \\, ds \\text{{ where }} C \\text{{ is the parabola }} y = {a}x^2 \\text{{ from }} (0,0) \\text{{ to }} (1,{a})"
            difficulty = 1700
    
    elif problem_type == 'vector_field_2d':
        # Line integral of vector field: ∫ F·dr
        # Choose simple answer
        answer = random.randint(-10, 10)
        
        # Parametric curve
        curve_type = random.choice(['circle', 'line'])
        
        if curve_type == 'circle':
            r = random.randint(1, 3)
            # Circle: x = r*cos(t), y = r*sin(t), t: 0 to 2π
            # dr = (-r*sin(t), r*cos(t)) dt
            
            # Choose F = (P, Q)
            a, b = random.randint(-2, 2), random.randint(-2, 2)
            P = -a*y
            Q = a*x + b
            
            F = sp.Matrix([P, Q])
            
            x_param = r*sp.cos(t)
            y_param = r*sp.sin(t)
            dx_dt = -r*sp.sin(t)
            dy_dt = r*sp.cos(t)
            
            P_param = P.subs([(x, x_param), (y, y_param)])
            Q_param = Q.subs([(x, x_param), (y, y_param)])
            
            integrand = P_param * dx_dt + Q_param * dy_dt
            actual_answer = sp.integrate(integrand, (t, 0, 2*sp.pi))
            answer = sp.simplify(actual_answer)
            
            question = f"\\int_C \\mathbf{{F}} \\cdot d\\mathbf{{r}} \\text{{ where }} \\mathbf{{F}} = ({sp.latex(P)}, {sp.latex(Q)}) \\text{{ and }} C \\text{{ is the circle }} x^2 + y^2 = {r**2} \\text{{ traversed counterclockwise}}"
            difficulty = 1680
        
        else:  # line
            x0, y0 = random.randint(-2, 2), random.randint(-2, 2)
            x1, y1 = random.randint(-2, 2), random.randint(-2, 2)
            
            # Line from (x0,y0) to (x1,y1)
            # x = x0 + (x1-x0)t, y = y0 + (y1-y0)t
            
            a, b, c, d = random.randint(-2, 3), random.randint(-2, 3), random.randint(-2, 3), random.randint(-2, 3)
            P = a*x + b*y
            Q = c*x + d*y
            
            x_param = x0 + (x1 - x0)*t
            y_param = y0 + (y1 - y0)*t
            dx_dt = x1 - x0
            dy_dt = y1 - y0
            
            P_param = P.subs([(x, x_param), (y, y_param)])
            Q_