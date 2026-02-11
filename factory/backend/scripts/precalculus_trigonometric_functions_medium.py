"""
precalculus - trigonometric_functions (medium)
Generated: 2026-02-11T21:52:52.579553
"""

import random
import json
from sympy import *

def generate_problem():
    x = symbols('x')
    problem_type = random.randint(1, 8)
    
    if problem_type == 1:
        # Evaluate trig function at special angle (1200-1300)
        angles = [0, pi/6, pi/4, pi/3, pi/2, 2*pi/3, 3*pi/4, 5*pi/6, pi]
        angle = random.choice(angles)
        func = random.choice([sin, cos, tan])
        
        if func == tan and angle in [pi/2, 3*pi/2]:
            angle = pi/4
        
        answer = func(angle)
        
        angle_display = latex(angle)
        func_name = func.__name__
        
        question = f"\\text{{Evaluate: }} {func_name}\\left({angle_display}\\right)"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1250,
            "main_topic": "precalculus",
            "subtopic": "trigonometric_functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 2:
        # Solve basic trig equation (1300-1400)
        coeff = random.choice([1, 2])
        value = random.choice([0, 1, -1, S(1)/2, -S(1)/2, sqrt(2)/2, -sqrt(2)/2, sqrt(3)/2, -sqrt(3)/2])
        func = random.choice([sin, cos])
        
        if func == sin:
            if value == 0:
                answer = 0
            elif value == S(1)/2:
                answer = pi/6
            elif value == -S(1)/2:
                answer = 7*pi/6
            elif value == sqrt(2)/2:
                answer = pi/4
            elif value == sqrt(3)/2:
                answer = pi/3
            elif value == 1:
                answer = pi/2
            elif value == -1:
                answer = 3*pi/2
            else:
                answer = 5*pi/4
        else:
            if value == 0:
                answer = pi/2
            elif value == S(1)/2:
                answer = pi/3
            elif value == -S(1)/2:
                answer = 2*pi/3
            elif value == sqrt(2)/2:
                answer = pi/4
            elif value == sqrt(3)/2:
                answer = pi/6
            elif value == 1:
                answer = 0
            elif value == -1:
                answer = pi
            else:
                answer = 3*pi/4
        
        func_name = func.__name__
        question = f"\\text{{Solve for }} x \\text{{ in }} [0, 2\\pi): {func_name}(x) = {latex(value)}"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1350,
            "main_topic": "precalculus",
            "subtopic": "trigonometric_functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 3:
        # Simplify using Pythagorean identity (1400-1500)
        choice = random.choice(['sin_to_cos', 'cos_to_sin'])
        
        if choice == 'sin_to_cos':
            angle = random.choice([pi/6, pi/4, pi/3])
            sin_val = sin(angle)
            answer = sqrt(1 - sin_val**2)
            question = f"\\text{{If }} \\sin(\\theta) = {latex(sin_val)} \\text{{ and }} \\theta \\text{{ is in Quadrant I, find }} \\cos(\\theta)"
        else:
            angle = random.choice([pi/6, pi/4, pi/3])
            cos_val = cos(angle)
            answer = sqrt(1 - cos_val**2)
            question = f"\\text{{If }} \\cos(\\theta) = {latex(cos_val)} \\text{{ and }} \\theta \\text{{ is in Quadrant I, find }} \\sin(\\theta)"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1450,
            "main_topic": "precalculus",
            "subtopic": "trigonometric_functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 4:
        # Amplitude and period identification (1300-1400)
        A = random.randint(2, 5)
        B = random.randint(2, 4)
        func = random.choice(['sin', 'cos'])
        
        period = 2*pi/B
        
        question = f"\\text{{Find the period of }} f(x) = {A}\\{func}({B}x)"
        
        return {
            "question_latex": question,
            "answer_key": str(period),
            "difficulty": 1320,
            "main_topic": "precalculus",
            "subtopic": "trigonometric_functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 5:
        # Double angle formula (1500-1600)
        angle = random.choice([pi/8, pi/12, pi/6])
        formula_type = random.choice(['sin', 'cos'])
        
        if formula_type == 'sin':
            answer = sin(2*angle)
            question = f"\\text{{Use the double angle formula to find }} \\sin\\left({latex(2*angle)}\\right) \\text{{ given that the angle is }} {latex(angle)}"
        else:
            answer = cos(2*angle)
            question = f"\\text{{Use the double angle formula to find }} \\cos\\left({latex(2*angle)}\\right) \\text{{ given that the angle is }} {latex(angle)}"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1550,
            "main_topic": "precalculus",
            "subtopic": "trigonometric_functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 6:
        # Find exact value using sum/difference formula (1500-1600)
        angle1 = random.choice([pi/3, pi/4, pi/6])
        angle2 = random.choice([pi/6, pi/4])
        operation = random.choice(['+', '-'])
        func = random.choice(['sin', 'cos'])
        
        if operation == '+':
            result_angle = angle1 + angle2
            op_text = '+'
        else:
            result_angle = angle1 - angle2
            op_text = '-'
        
        if func == 'sin':
            answer = sin(result_angle)
            question = f"\\text{{Find the exact value: }} \\sin\\left({latex(angle1)} {op_text} {latex(angle2)}\\right)"
        else:
            answer = cos(result_angle)
            question = f"\\text{{Find the exact value: }} \\cos\\left({latex(angle1)} {op_text} {latex(angle2)}\\right)"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1520,
            "main_topic": "precalculus",
            "subtopic": "trigonometric_functions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 7:
        # Two-step trig equation (1