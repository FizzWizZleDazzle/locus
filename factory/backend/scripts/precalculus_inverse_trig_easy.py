"""
precalculus - inverse_trig (easy)
Generated: 2026-02-11T21:54:42.343758
"""

import sympy as sp
import random
import json

def generate_inverse_trig_problem():
    problem_type = random.choice([
        'basic_evaluation',
        'basic_evaluation_degrees',
        'simple_equation',
        'composition_basic'
    ])
    
    x = sp.Symbol('x')
    
    if problem_type == 'basic_evaluation':
        # 1000-1100: Direct evaluation of inverse trig at standard values
        func_choice = random.choice(['arcsin', 'arccos', 'arctan'])
        
        if func_choice == 'arcsin':
            values = [
                (0, 0),
                (sp.Rational(1, 2), sp.pi/6),
                (sp.sqrt(2)/2, sp.pi/4),
                (sp.sqrt(3)/2, sp.pi/3),
                (1, sp.pi/2),
                (-sp.Rational(1, 2), -sp.pi/6),
                (-sp.sqrt(2)/2, -sp.pi/4),
                (-sp.sqrt(3)/2, -sp.pi/3),
                (-1, -sp.pi/2)
            ]
            val, ans = random.choice(values)
            question = f"\\arcsin\\left({sp.latex(val)}\\right)"
            answer = ans
            difficulty = 1050
            
        elif func_choice == 'arccos':
            values = [
                (1, 0),
                (sp.sqrt(3)/2, sp.pi/6),
                (sp.sqrt(2)/2, sp.pi/4),
                (sp.Rational(1, 2), sp.pi/3),
                (0, sp.pi/2),
                (-sp.Rational(1, 2), 2*sp.pi/3),
                (-sp.sqrt(2)/2, 3*sp.pi/4),
                (-sp.sqrt(3)/2, 5*sp.pi/6),
                (-1, sp.pi)
            ]
            val, ans = random.choice(values)
            question = f"\\arccos\\left({sp.latex(val)}\\right)"
            answer = ans
            difficulty = 1050
            
        else:  # arctan
            values = [
                (0, 0),
                (1, sp.pi/4),
                (sp.sqrt(3), sp.pi/3),
                (-1, -sp.pi/4),
                (-sp.sqrt(3), -sp.pi/3),
                (sp.sqrt(3)/3, sp.pi/6)
            ]
            val, ans = random.choice(values)
            question = f"\\arctan\\left({sp.latex(val)}\\right)"
            answer = ans
            difficulty = 1100
    
    elif problem_type == 'basic_evaluation_degrees':
        # 1100-1150: Same as above but answer in degrees
        func_choice = random.choice(['arcsin', 'arccos', 'arctan'])
        
        if func_choice == 'arcsin':
            values = [
                (sp.Rational(1, 2), 30),
                (sp.sqrt(2)/2, 45),
                (sp.sqrt(3)/2, 60),
                (-sp.Rational(1, 2), -30),
                (-sp.sqrt(2)/2, -45)
            ]
            val, ans_deg = random.choice(values)
            question = f"\\arcsin\\left({sp.latex(val)}\\right) \\text{{ in degrees}}"
            answer = ans_deg
            difficulty = 1100
            
        elif func_choice == 'arccos':
            values = [
                (sp.sqrt(3)/2, 30),
                (sp.sqrt(2)/2, 45),
                (sp.Rational(1, 2), 60),
                (0, 90),
                (-sp.Rational(1, 2), 120)
            ]
            val, ans_deg = random.choice(values)
            question = f"\\arccos\\left({sp.latex(val)}\\right) \\text{{ in degrees}}"
            answer = ans_deg
            difficulty = 1100
            
        else:  # arctan
            values = [
                (1, 45),
                (sp.sqrt(3), 60),
                (sp.sqrt(3)/3, 30),
                (-1, -45)
            ]
            val, ans_deg = random.choice(values)
            question = f"\\arctan\\left({sp.latex(val)}\\right) \\text{{ in degrees}}"
            answer = ans_deg
            difficulty = 1100
    
    elif problem_type == 'simple_equation':
        # 1200-1300: Simple inverse trig equation
        func_choice = random.choice(['arcsin', 'arccos', 'arctan'])
        
        if func_choice == 'arcsin':
            answers = [0, sp.Rational(1, 2), sp.sqrt(2)/2, sp.sqrt(3)/2, -sp.Rational(1, 2)]
            ans = random.choice(answers)
            question = f"\\text{{Solve for }} x: \\arcsin(x) = {sp.latex(sp.asin(ans))}"
            answer = ans
            difficulty = 1250
            
        elif func_choice == 'arccos':
            answers = [0, sp.Rational(1, 2), sp.sqrt(2)/2, sp.sqrt(3)/2]
            ans = random.choice(answers)
            question = f"\\text{{Solve for }} x: \\arccos(x) = {sp.latex(sp.acos(ans))}"
            answer = ans
            difficulty = 1250
            
        else:  # arctan
            answers = [0, 1, sp.sqrt(3), sp.sqrt(3)/3, -1]
            ans = random.choice(answers)
            question = f"\\text{{Solve for }} x: \\arctan(x) = {sp.latex(sp.atan(ans))}"
            answer = ans
            difficulty = 1250
    
    else:  # composition_basic
        # 1200-1300: Basic composition like sin(arcsin(x))
        comp_type = random.choice(['identity', 'simple_comp'])
        
        if comp_type == 'identity':
            func_pairs = [
                ('\\sin', '\\arcsin'),
                ('\\cos', '\\arccos'),
                ('\\tan', '\\arctan')
            ]
            outer, inner = random.choice(func_pairs)
            val = random.choice([sp.Rational(1, 3), sp.Rational(2, 5), sp.Rational(3, 4)])
            question = f"{outer}\\left({inner}\\left({sp.latex(val)}\\right)\\right)"
            answer = val
            difficulty = 1200
        else:
            # cos(arcsin(x)) type - requires small calculation
            val = random.choice([sp.Rational(3, 5), sp.Rational(4, 5), sp.Rational(5, 13)])
            question = f"\\cos\\left(\\arcsin\\left({sp.latex(val)}\\right)\\right)"
            answer = sp.sqrt(1 - val**2)
            difficulty = 1300
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "precalculus",
        "subtopic": "inverse_trig",
        "grading_mode": "equivalent"
    }

problem = generate_inverse_trig_problem()
print(json.dumps(problem))