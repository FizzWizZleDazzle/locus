"""
precalculus - trigonometric_functions (hard)
Generated: 2026-02-11T21:53:16.767956
"""

import sympy as sp
import random
import json

def generate_hard_trig_problem():
    x, t, theta = sp.symbols('x t theta', real=True)
    
    problem_type = random.choice([
        'inverse_composition',
        'trig_equation_multiple_angles',
        'sum_to_product',
        'amplitude_phase_shift',
        'parametric_elimination',
        'double_angle_proof',
        'compound_angle_simplification'
    ])
    
    if problem_type == 'inverse_composition':
        # arcsin(sin(...)) or arctan(tan(...)) with strategic values
        target_answer = random.choice([sp.pi/6, sp.pi/4, sp.pi/3, -sp.pi/6, -sp.pi/4])
        offset = random.choice([2*sp.pi, 4*sp.pi, -2*sp.pi])
        inner_value = target_answer + offset
        
        func_choice = random.choice(['sin', 'cos', 'tan'])
        if func_choice == 'sin':
            question = f"\\text{{Simplify: }} \\arcsin(\\sin({sp.latex(inner_value)}))"
        elif func_choice == 'cos':
            question = f"\\text{{Simplify: }} \\arccos(\\cos({sp.latex(inner_value)}))"
        else:
            question = f"\\text{{Simplify: }} \\arctan(\\tan({sp.latex(inner_value)}))"
        
        answer = target_answer
        difficulty = random.randint(1650, 1750)
    
    elif problem_type == 'trig_equation_multiple_angles':
        # Equations like sin(2x) = cos(x) for x in [0, 2pi]
        k = random.randint(1, 3)
        solutions = []
        
        if random.choice([True, False]):
            # sin(2x) = sin(x)
            # Solutions: x = 0, pi/3, pi, 5pi/3, 2pi
            question = f"\\text{{Find the smallest positive solution to }} \\sin(2x) = \\sin(x)"
            answer = sp.pi / 3
            difficulty = 1680
        else:
            # cos(2x) = cos(x)
            # Solutions: x = 0, 2pi/3, 4pi/3, 2pi
            question = f"\\text{{Find the smallest positive solution to }} \\cos(2x) = \\cos(x)"
            answer = sp.Rational(2, 3) * sp.pi
            difficulty = 1700
    
    elif problem_type == 'sum_to_product':
        # sin(A) + sin(B) = 2sin((A+B)/2)cos((A-B)/2)
        a_num = random.choice([1, 2, 3, 5, 7])
        b_num = random.choice([1, 2, 3, 5, 7])
        a_den = random.choice([6, 4, 3, 2])
        b_den = random.choice([6, 4, 3, 2])
        
        A = sp.Rational(a_num, a_den) * sp.pi
        B = sp.Rational(b_num, b_den) * sp.pi
        
        sum_val = sp.sin(A) + sp.sin(B)
        answer = sp.simplify(sum_val)
        
        question = f"\\text{{Simplify: }} \\sin\\left({sp.latex(A)}\\right) + \\sin\\left({sp.latex(B)}\\right)"
        difficulty = random.randint(1620, 1720)
    
    elif problem_type == 'amplitude_phase_shift':
        # Convert a*sin(x) + b*cos(x) to R*sin(x + phi)
        a = random.choice([1, 2, 3, sp.sqrt(3)])
        b = random.choice([1, 2, 3, sp.sqrt(3)])
        
        R = sp.sqrt(a**2 + b**2)
        answer = sp.simplify(R)
        
        question = f"\\text{{Find the amplitude of }} {sp.latex(a)}\\sin(x) + {sp.latex(b)}\\cos(x)"
        difficulty = random.randint(1680, 1780)
    
    elif problem_type == 'parametric_elimination':
        # Given x = cos(t), y = sin(t), find relationship
        a = random.choice([2, 3, 4, 5])
        b = random.choice([2, 3, 4, 5])
        
        question = f"\\text{{If }} x = {a}\\cos(t) \\text{{ and }} y = {b}\\sin(t), \\text{{ find }} x^2/{a**2} + y^2/{b**2}"
        answer = sp.Integer(1)
        difficulty = 1650
    
    elif problem_type == 'double_angle_proof':
        # Simplify expressions using double angle formulas
        angle = random.choice([sp.pi/8, sp.pi/12, 3*sp.pi/8])
        target = 2 * angle
        
        # cos(2θ) = 2cos²(θ) - 1
        expr = 2 * sp.cos(angle)**2 - 1
        answer = sp.cos(target)
        answer_simplified = sp.simplify(answer)
        
        question = f"\\text{{Simplify: }} 2\\cos^2\\left({sp.latex(angle)}\\right) - 1"
        difficulty = random.randint(1640, 1740)
    
    else:  # compound_angle_simplification
        # sin(a+b) or cos(a-b) with specific angles
        angles = [sp.pi/6, sp.pi/4, sp.pi/3, sp.pi/2]
        a = random.choice(angles)
        b = random.choice(angles)
        
        if random.choice([True, False]):
            answer = sp.sin(a + b)
            question = f"\\text{{Evaluate: }} \\sin\\left({sp.latex(a)} + {sp.latex(b)}\\right)"
        else:
            answer = sp.cos(a - b)
            question = f"\\text{{Evaluate: }} \\cos\\left({sp.latex(a)} - {sp.latex(b)}\\right)"
        
        answer = sp.simplify(answer)
        difficulty = random.randint(1600, 1700)
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "precalculus",
        "subtopic": "trigonometric_functions",
        "grading_mode": "equivalent"
    }

problem = generate_hard_trig_problem()
print(json.dumps(problem))