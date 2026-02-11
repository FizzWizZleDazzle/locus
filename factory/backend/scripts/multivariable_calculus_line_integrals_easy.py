"""
multivariable_calculus - line_integrals (easy)
Generated: 2026-02-11T22:06:17.280016
"""

import sympy as sp
import random
import json

def generate_line_integral_problem():
    x, y, t = sp.symbols('x y t', real=True)
    
    problem_type = random.choice([
        'straight_line_scalar',
        'straight_line_vector', 
        'parametric_simple',
        'arc_length_element'
    ])
    
    if problem_type == 'straight_line_scalar':
        # Integrate f along straight line from (0,0) to (a,b)
        a = random.randint(1, 4)
        b = random.randint(1, 4)
        
        # Pick simple answer first
        answer_value = random.randint(2, 20)
        
        # Work backward: integral of f(x,y) from t=0 to 1 where x=at, y=bt
        # Simple case: f(x,y) = c (constant)
        # integral = c * sqrt(a^2 + b^2)
        arc_length = sp.sqrt(a**2 + b**2)
        c = answer_value / arc_length
        
        if c == int(c):
            c = int(c)
            f = sp.sympify(c)
        else:
            # Try f(x,y) = kx or ky
            if random.choice([True, False]):
                # f(x,y) = kx, x = at, integral from 0 to 1: k*a*t*sqrt(a^2+b^2) dt = k*a*sqrt(a^2+b^2)/2
                k = sp.Rational(2 * answer_value, a * int(arc_length**2)).limit_denominator(10)
                f = k * x
            else:
                k = sp.Rational(2 * answer_value, b * int(arc_length**2)).limit_denominator(10)
                f = k * y
        
        answer = answer_value
        
        question = f"Evaluate the line integral \\int_C {sp.latex(f)} \\, ds where C is the line segment from (0,0) to ({a},{b})."
        difficulty = random.randint(1000, 1150)
        
    elif problem_type == 'straight_line_vector':
        # Vector field F dot dr along straight line
        a = random.randint(1, 3)
        b = random.randint(1, 3)
        
        # Pick answer first
        answer_value = random.randint(1, 15)
        
        # F = <P, Q>, dr = <dx, dy> = <a, b>dt
        # Integral from 0 to 1: (P*a + Q*b) dt
        # For constant F: answer = P*a + Q*b
        P = random.randint(1, 5)
        Q_needed = answer_value - P * a
        if Q_needed % b == 0:
            Q = Q_needed // b
        else:
            P = 1
            Q = 1
            answer_value = P * a + Q * b
            
        answer = answer_value
        
        F_latex = f"\\langle {P}, {Q} \\rangle"
        question = f"Evaluate the line integral \\int_C \\mathbf{{F}} \\cdot d\\mathbf{{r}} where \\mathbf{{F}} = {F_latex} and C is the line segment from (0,0) to ({a},{b})."
        difficulty = random.randint(1100, 1250)
        
    elif problem_type == 'parametric_simple':
        # Simple parametric curve
        a = random.randint(1, 4)
        b = random.randint(1, 4)
        
        # r(t) = <t, t^2> from 0 to a or r(t) = <t, t> from 0 to a
        if random.choice([True, False]):
            # r(t) = <t, t>, f = c
            answer_value = random.randint(2, 12)
            c = sp.Rational(answer_value, a * sp.sqrt(2)).limit_denominator(10)
            
            answer = float(c * a * sp.sqrt(2))
            
            question = f"Evaluate \\int_C {sp.latex(c)} \\, ds where C is parameterized by \\mathbf{{r}}(t) = \\langle t, t \\rangle for 0 \\leq t \\leq {a}."
            difficulty = random.randint(1150, 1300)
        else:
            # r(t) = <t, 0>, f = k*x
            k = random.randint(1, 4)
            answer_value = k * a**2 / 2
            
            answer = answer_value
            
            question = f"Evaluate \\int_C {k}x \\, ds where C is parameterized by \\mathbf{{r}}(t) = \\langle t, 0 \\rangle for 0 \\leq t \\leq {a}."
            difficulty = random.randint(1100, 1250)
            
    else:  # arc_length_element
        # Just compute arc length
        a = random.randint(2, 5)
        
        if random.choice([True, False]):
            # Horizontal line
            answer = a
            question = f"Find the arc length of the curve C parameterized by \\mathbf{{r}}(t) = \\langle t, 0 \\rangle for 0 \\leq t \\leq {a}."
            difficulty = random.randint(1000, 1100)
        else:
            # Diagonal line
            answer = a * sp.sqrt(2)
            question = f"Find the arc length of the curve C parameterized by \\mathbf{{r}}(t) = \\langle t, t \\rangle for 0 \\leq t \\leq {a}."
            difficulty = random.randint(1050, 1150)
    
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