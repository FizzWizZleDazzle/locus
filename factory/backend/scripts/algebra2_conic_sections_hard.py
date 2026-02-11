"""
algebra2 - conic_sections (hard)
Generated: 2026-02-11T21:51:18.376254
"""

import sympy as sp
import random
import json

def generate_conic_problem():
    x, y = sp.symbols('x y', real=True)
    
    problem_type = random.choice([
        'ellipse_focus_eccentricity',
        'hyperbola_asymptotes',
        'parabola_directrix_focus',
        'rotation_elimination',
        'general_form_classification',
        'tangent_line_conic',
        'parametric_intersection'
    ])
    
    if problem_type == 'ellipse_focus_eccentricity':
        a = random.randint(4, 8)
        e = sp.Rational(random.randint(1, a-1), a)
        c = a * e
        b_sq = a**2 - c**2
        
        question = f"An ellipse has semi-major axis $a = {a}$ and eccentricity $e = {sp.latex(e)}$. Find the equation of the ellipse centered at the origin with foci on the x-axis. Express your answer in the form $\\frac{{x^2}}{{a^2}} + \\frac{{y^2}}{{b^2}} = 1$. What is $b^2$?"
        answer = b_sq
        difficulty = 1650
        
    elif problem_type == 'hyperbola_asymptotes':
        m = random.randint(2, 5)
        h = random.randint(-3, 3)
        k = random.randint(-3, 3)
        a = random.randint(2, 4)
        b = m * a
        
        question = f"A hyperbola has center $({h}, {k})$, transverse axis parallel to the x-axis with $a = {a}$, and asymptotes with slopes $\\pm {m}$. Find the value of $b$ in the equation $\\frac{{(x-{h})^2}}{{{a**2}}} - \\frac{{(y-{k})^2}}{{b^2}} = 1$."
        answer = b
        difficulty = 1620
        
    elif problem_type == 'parabola_directrix_focus':
        p = random.randint(2, 6)
        h = random.randint(-4, 4)
        k = random.randint(-4, 4)
        
        question = f"A parabola has focus at $({h}, {k + p})$ and directrix $y = {k - p}$. Find the x-coordinate of the vertex."
        answer = h
        difficulty = 1680
        
    elif problem_type == 'rotation_elimination':
        b_coef = random.choice([6, 8, 10, 12])
        a_coef = random.randint(3, 7)
        c_coef = a_coef + random.choice([2, 4, 6])
        
        cot_2theta = sp.Rational(a_coef - c_coef, b_coef)
        
        question = f"The conic section ${a_coef}x^2 + {b_coef}xy + {c_coef}y^2 = 1$ can be rotated to eliminate the $xy$ term. Find $\\cot(2\\theta)$ where $\\theta$ is the rotation angle."
        answer = cot_2theta
        difficulty = 1750
        
    elif problem_type == 'general_form_classification':
        A = random.randint(1, 4)
        C = random.randint(1, 4)
        B = random.choice([2*sp.sqrt(A*C), -2*sp.sqrt(A*C)])
        discriminant = B**2 - 4*A*C
        
        if discriminant > 0:
            conic_type = "hyperbola"
        elif discriminant == 0:
            conic_type = "parabola"
        else:
            conic_type = "ellipse"
            
        question = f"Calculate the discriminant $B^2 - 4AC$ for the conic ${A}x^2 + {sp.latex(B)}xy + {C}y^2 + x + y = 5$."
        answer = discriminant
        difficulty = 1700
        
    elif problem_type == 'tangent_line_conic':
        a_sq = random.randint(9, 25)
        b_sq = random.randint(4, 16)
        x0 = random.randint(2, 4)
        y0_sq = b_sq * (1 - sp.Rational(x0**2, a_sq))
        
        if y0_sq > 0:
            y0_num = int(sp.sqrt(y0_sq * b_sq))
            y0_denom = int(sp.sqrt(b_sq))
            
            slope_num = -b_sq * x0 * y0_denom
            slope_denom = a_sq * y0_num
            slope = sp.Rational(slope_num, slope_denom)
            
            question = f"Find the slope of the tangent line to the ellipse $\\frac{{x^2}}{{{a_sq}}} + \\frac{{y^2}}{{{b_sq}}} = 1$ at the point $({x0}, \\frac{{{y0_num}}}{{{y0_denom}}})$."
            answer = slope
            difficulty = 1820
        else:
            x0 = 2
            y0 = 2
            r_sq = x0**2 + y0**2
            slope = sp.Rational(-x0, y0)
            
            question = f"Find the slope of the tangent line to the circle $x^2 + y^2 = {r_sq}$ at the point $({x0}, {y0})$."
            answer = slope
            difficulty = 1650
            
    else:
        a = random.randint(3, 6)
        b = random.randint(2, 5)
        c = a * b
        
        question = f"A hyperbola $\\frac{{x^2}}{{{a**2}}} - \\frac{{y^2}}{{{b**2}}} = 1$ has focal distance $2c$ where $c^2 = a^2 + b^2$. Find $c^2$."
        answer = a**2 + b**2
        difficulty = 1630
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "conic_sections",
        "grading_mode": "equivalent"
    }

problem = generate_conic_problem()
print(json.dumps(problem))