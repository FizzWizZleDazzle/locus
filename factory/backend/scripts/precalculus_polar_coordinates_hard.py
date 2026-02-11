"""
precalculus - polar_coordinates (hard)
Generated: 2026-02-11T21:56:27.979540
"""

import sympy as sp
import random
import json

def generate_polar_problem():
    random.seed()
    r, theta = sp.symbols('r theta', real=True)
    x, y = sp.symbols('x y', real=True)
    
    problem_type = random.choice([
        'convert_complex_polar_to_rect',
        'convert_complex_rect_to_polar', 
        'polar_equation_convert',
        'distance_between_polar_points',
        'area_polar_curve',
        'intersection_polar_curves'
    ])
    
    if problem_type == 'convert_complex_polar_to_rect':
        # Target ELO: 1600-1700
        # Pick clean rectangular form first
        a = random.choice([1, 2, 3, 4, 5])
        b = random.choice([1, 2, 3, 4, 5])
        if random.choice([True, False]):
            b = -b
        
        # Build polar form from rectangular
        r_val = sp.sqrt(a**2 + b**2)
        theta_val = sp.atan2(b, a)
        
        # Create problem with special angle if possible
        angles_map = {
            0: (1, 0),
            sp.pi/6: (sp.sqrt(3)/2, sp.Rational(1, 2)),
            sp.pi/4: (sp.sqrt(2)/2, sp.sqrt(2)/2),
            sp.pi/3: (sp.Rational(1, 2), sp.sqrt(3)/2),
            sp.pi/2: (0, 1),
            2*sp.pi/3: (sp.Rational(-1, 2), sp.sqrt(3)/2),
            3*sp.pi/4: (-sp.sqrt(2)/2, sp.sqrt(2)/2),
            5*sp.pi/6: (-sp.sqrt(3)/2, sp.Rational(1, 2)),
        }
        
        angle = random.choice(list(angles_map.keys()))
        magnitude = random.choice([2, 3, 4, 5, 6])
        cos_val, sin_val = angles_map[angle]
        
        x_ans = sp.simplify(magnitude * cos_val)
        y_ans = sp.simplify(magnitude * sin_val)
        
        question = f"Convert the polar coordinates $\\left({magnitude}, {sp.latex(angle)}\\right)$ to rectangular coordinates $(x, y)$."
        answer = f"({sp.latex(x_ans)}, {sp.latex(y_ans)})"
        
        return {
            "question_latex": question,
            "answer_key": answer,
            "difficulty": 1650,
            "main_topic": "precalculus",
            "subtopic": "polar_coordinates",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'convert_complex_rect_to_polar':
        # Target ELO: 1650-1750
        angles_map = {
            sp.pi/6: (sp.sqrt(3), 1),
            sp.pi/4: (1, 1),
            sp.pi/3: (1, sp.sqrt(3)),
            2*sp.pi/3: (-1, sp.sqrt(3)),
            3*sp.pi/4: (-1, 1),
            5*sp.pi/6: (-sp.sqrt(3), 1),
            7*sp.pi/6: (-sp.sqrt(3), -1),
            5*sp.pi/4: (-1, -1),
            4*sp.pi/3: (-1, -sp.sqrt(3)),
        }
        
        angle = random.choice(list(angles_map.keys()))
        x_base, y_base = angles_map[angle]
        scale = random.choice([2, 3, 4])
        
        x_val = scale * x_base
        y_val = scale * y_base
        r_ans = sp.simplify(sp.sqrt(x_val**2 + y_val**2))
        theta_ans = angle
        
        question = f"Convert the rectangular coordinates $({sp.latex(x_val)}, {sp.latex(y_val)})$ to polar coordinates $(r, \\theta)$ where $r > 0$ and $0 \\leq \\theta < 2\\pi$."
        answer = f"({sp.latex(r_ans)}, {sp.latex(theta_ans)})"
        
        return {
            "question_latex": question,
            "answer_key": answer,
            "difficulty": 1700,
            "main_topic": "precalculus",
            "subtopic": "polar_coordinates",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'polar_equation_convert':
        # Target ELO: 1700-1800
        conversion_type = random.choice(['polar_to_rect', 'rect_to_polar'])
        
        if conversion_type == 'polar_to_rect':
            # Start with rectangular equation
            eq_type = random.choice(['circle', 'line', 'parabola'])
            
            if eq_type == 'circle':
                h = random.choice([2, 3, 4, 5])
                k = random.choice([0, 2, 3])
                
                # (x-h)^2 + (y-k)^2 = h^2 + k^2 passes through origin
                # In polar: r = 2h*cos(theta) + 2k*sin(theta)
                polar_eq = f"r = {2*h}\\cos(\\theta)"
                if k != 0:
                    polar_eq += f" + {2*k}\\sin(\\theta)"
                
                rect_eq = f"x^2 + y^2 = {2*h}x"
                if k != 0:
                    rect_eq += f" + {2*k}y"
                
                question = f"Convert the polar equation ${polar_eq}$ to rectangular form."
                answer = rect_eq
                elo = 1750
            
            elif eq_type == 'line':
                a = random.choice([1, 2, 3, 4])
                # r*cos(theta) = a  =>  x = a
                polar_eq = f"r\\cos(\\theta) = {a}"
                rect_eq = f"x = {a}"
                
                question = f"Convert the polar equation ${polar_eq}$ to rectangular form."
                answer = rect_eq
                elo = 1700
            
            else:  # parabola
                a = random.choice([1, 2, 3])
                # r = a/(1-cos(theta))  parabola
                polar_eq = f"r = \\frac{{{a}}}{{1 - \\cos(\\theta)}}"
                rect_eq = f"y^2 = {2*a}({a} + x)"
                
                question = f"Convert the polar equation ${polar_eq}$ to rectangular form."
                answer = rect_eq
                elo = 1850
        
        else:  # rect_to_polar
            eq_type = random.choice(['circle_origin', 'line_through_origin'])
            
            if eq_type == 'circle_origin':
                rad = random.choice([2, 3, 4, 5])
                rect_eq = f"x^2 + y^2 = {rad**2}"
                polar_eq = f"r = {rad}"
                
                question = f"Convert the rectangular equation ${rect_eq}$ to polar form."
                answer = polar_eq
                elo = 1650
            
            else:  # line through origin
                m = random.choice([1, 2, 3, sp.sqrt(3)])
                if m == sp.sqrt(3):