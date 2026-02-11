"""
precalculus - trigonometric_functions (easy)
Generated: 2026-02-11T21:52:28.188451
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'evaluate_special_angle',
        'find_angle_from_value',
        'basic_identity',
        'reference_angle',
        'period_basic'
    ])
    
    if problem_type == 'evaluate_special_angle':
        angles = [0, 30, 45, 60, 90, 120, 135, 150, 180, 210, 225, 240, 270, 300, 315, 330, 360]
        angle = random.choice(angles)
        func = random.choice(['sin', 'cos', 'tan'])
        
        if func == 'tan' and angle in [90, 270]:
            angle = random.choice([0, 30, 45, 60, 120, 135, 150, 180, 210, 225, 240, 300, 315, 330, 360])
        
        angle_rad = sp.rad(angle)
        
        if func == 'sin':
            answer = sp.sin(angle_rad)
            func_latex = r'\sin'
        elif func == 'cos':
            answer = sp.cos(angle_rad)
            func_latex = r'\cos'
        else:
            answer = sp.tan(angle_rad)
            func_latex = r'\tan'
        
        question = f"{func_latex}({angle}^\\circ)"
        difficulty = 1050
        
    elif problem_type == 'find_angle_from_value':
        func = random.choice(['sin', 'cos'])
        
        if func == 'sin':
            values = {0: [0, 180], 1: [90], sp.Rational(1, 2): [30, 150], 
                     sp.sqrt(2)/2: [45, 135], sp.sqrt(3)/2: [60, 120],
                     sp.Rational(-1, 2): [210, 330], -sp.sqrt(2)/2: [225, 315],
                     -sp.sqrt(3)/2: [240, 300], -1: [270]}
            value = random.choice(list(values.keys()))
            angles = values[value]
            angle = random.choice(angles)
            func_latex = r'\sin'
        else:
            values = {1: [0, 360], sp.sqrt(3)/2: [30, 330], sp.sqrt(2)/2: [45, 315],
                     sp.Rational(1, 2): [60, 300], 0: [90, 270],
                     sp.Rational(-1, 2): [120, 240], -sp.sqrt(2)/2: [135, 225],
                     -sp.sqrt(3)/2: [150, 210], -1: [180]}
            value = random.choice(list(values.keys()))
            angles = values[value]
            angle = random.choice(angles)
            func_latex = r'\cos'
        
        question = f"Find \\theta in [0^\\circ, 360^\\circ) where {func_latex}(\\theta) = {sp.latex(value)}"
        answer = sp.Integer(angle)
        difficulty = 1250
        
    elif problem_type == 'basic_identity':
        angle = random.choice([30, 45, 60])
        identity_type = random.choice(['sin_squared_plus_cos_squared', 'tan_equals_sin_over_cos'])
        
        if identity_type == 'sin_squared_plus_cos_squared':
            angle_rad = sp.rad(angle)
            answer = sp.Integer(1)
            question = f"\\sin^2({angle}^\\circ) + \\cos^2({angle}^\\circ)"
            difficulty = 1100
        else:
            angle_rad = sp.rad(angle)
            answer = sp.tan(angle_rad)
            question = f"\\frac{{\\sin({angle}^\\circ)}}{{\\cos({angle}^\\circ)}}"
            difficulty = 1200
    
    elif problem_type == 'reference_angle':
        quadrant = random.randint(2, 4)
        ref_angle = random.choice([30, 45, 60])
        
        if quadrant == 2:
            angle = 180 - ref_angle
        elif quadrant == 3:
            angle = 180 + ref_angle
        else:
            angle = 360 - ref_angle
        
        answer = sp.Integer(ref_angle)
        question = f"Find the reference angle for {angle}^\\circ"
        difficulty = 1150
        
    else:
        func = random.choice(['sin', 'cos', 'tan'])
        
        if func == 'sin':
            answer = sp.Integer(360)
            func_latex = r'\sin'
        elif func == 'cos':
            answer = sp.Integer(360)
            func_latex = r'\cos'
        else:
            answer = sp.Integer(180)
            func_latex = r'\tan'
        
        question = f"What is the period (in degrees) of {func_latex}(\\theta)?"
        difficulty = 1150
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "precalculus",
        "subtopic": "trigonometric_functions",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))