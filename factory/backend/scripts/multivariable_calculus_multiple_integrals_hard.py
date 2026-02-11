"""
multivariable_calculus - multiple_integrals (hard)
Generated: 2026-02-11T22:04:42.923834
"""

import random
import json
from sympy import symbols, cos, sin, exp, log, pi, sqrt, simplify, integrate, Rational

def generate_problem():
    x, y, z, r, theta = symbols('x y z r theta', real=True)
    problem_type = random.choice([
        'rectangular_with_trig',
        'polar_conversion',
        'change_of_order',
        'cylindrical',
        'volume_between_surfaces',
        'triple_integral_spherical'
    ])
    
    if problem_type == 'rectangular_with_trig':
        a = random.randint(1, 3)
        b = random.randint(1, 3)
        c = random.randint(1, 4)
        
        answer = simplify(a * b * c / 2)
        integrand = a * cos(c * x)
        
        question = f"\\int_0^{{\\pi/{c}}} \\int_0^{{{b}}} {a}\\cos({c}x) \\, dy \\, dx"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1650,
            "main_topic": "multivariable_calculus",
            "subtopic": "multiple_integrals",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'polar_conversion':
        a = random.randint(2, 5)
        
        answer = simplify(pi * a**2 / 4)
        
        question = f"\\int_0^{{{a}}} \\int_0^{{\\sqrt{{{a}^2 - x^2}}}} 1 \\, dy \\, dx"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1700,
            "main_topic": "multivariable_calculus",
            "subtopic": "multiple_integrals",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'change_of_order':
        a = random.randint(2, 4)
        b = random.randint(1, 3)
        
        answer = simplify(b * a**3 / 3)
        
        question = f"\\int_0^{{{a}}} \\int_0^{{x}} {b}x \\, dy \\, dx"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1620,
            "main_topic": "multivariable_calculus",
            "subtopic": "multiple_integrals",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'cylindrical':
        a = random.randint(1, 3)
        h = random.randint(2, 4)
        
        answer = simplify(pi * a**2 * h)
        
        question = f"\\int_0^{{2\\pi}} \\int_0^{{{a}}} \\int_0^{{{h}}} r \\, dz \\, dr \\, d\\theta"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1680,
            "main_topic": "multivariable_calculus",
            "subtopic": "multiple_integrals",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'volume_between_surfaces':
        a = random.randint(1, 3)
        b = random.randint(1, 2)
        
        answer = simplify(4 * a * b / 3)
        
        question = f"\\int_{{-{a}}}^{{{a}}} \\int_0^{{\\sqrt{{{a}^2 - x^2}}}} {b} \\, dy \\, dx"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1750,
            "main_topic": "multivariable_calculus",
            "subtopic": "multiple_integrals",
            "grading_mode": "equivalent"
        }
    
    else:  # triple_integral_spherical
        a = random.randint(2, 4)
        
        answer = simplify(4 * pi * a**3 / 3)
        
        question = f"\\int_0^{{2\\pi}} \\int_0^{{\\pi}} \\int_0^{{{a}}} \\rho^2 \\sin(\\phi) \\, d\\rho \\, d\\phi \\, d\\theta"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1850,
            "main_topic": "multivariable_calculus",
            "subtopic": "multiple_integrals",
            "grading_mode": "equivalent"
        }

problem = generate_problem()
print(json.dumps(problem))