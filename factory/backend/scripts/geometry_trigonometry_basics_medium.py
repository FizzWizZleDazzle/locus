"""
geometry - trigonometry_basics (medium)
Generated: 2026-02-11T21:44:45.891652
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'basic_trig_ratio',
        'find_angle',
        'word_problem_basic',
        'special_angles',
        'pythagorean_trig'
    ])
    
    if problem_type == 'basic_trig_ratio':
        angle_deg = random.choice([30, 45, 60])
        trig_func = random.choice(['sin', 'cos', 'tan'])
        
        if angle_deg == 30:
            values = {'sin': sp.Rational(1, 2), 'cos': sp.sqrt(3)/2, 'tan': sp.sqrt(3)/3}
        elif angle_deg == 45:
            values = {'sin': sp.sqrt(2)/2, 'cos': sp.sqrt(2)/2, 'tan': sp.Integer(1)}
        else:  # 60
            values = {'sin': sp.sqrt(3)/2, 'cos': sp.Rational(1, 2), 'tan': sp.sqrt(3)}
        
        answer = values[trig_func]
        question = f"\\text{{Find the exact value of }} \\{trig_func}({angle_deg}^\\circ)"
        difficulty = 1300
        
    elif problem_type == 'find_angle':
        trig_func = random.choice(['sin', 'cos', 'tan'])
        angle_deg = random.choice([30, 45, 60])
        
        if angle_deg == 30:
            values = {'sin': sp.Rational(1, 2), 'cos': sp.sqrt(3)/2, 'tan': sp.sqrt(3)/3}
        elif angle_deg == 45:
            values = {'sin': sp.sqrt(2)/2, 'cos': sp.sqrt(2)/2, 'tan': sp.Integer(1)}
        else:  # 60
            values = {'sin': sp.sqrt(3)/2, 'cos': sp.Rational(1, 2), 'tan': sp.sqrt(3)}
        
        value = values[trig_func]
        answer = sp.Integer(angle_deg)
        question = f"\\text{{Find }} \\theta \\text{{ in degrees if }} \\{trig_func}(\\theta) = {sp.latex(value)} \\text{{ where }} 0^\\circ < \\theta < 90^\\circ"
        difficulty = 1400
        
    elif problem_type == 'word_problem_basic':
        height = random.randint(8, 20)
        angle = random.choice([30, 45, 60])
        
        if angle == 30:
            distance = height * sp.sqrt(3)
        elif angle == 45:
            distance = height
        else:  # 60
            distance = height * sp.sqrt(3) / 3
        
        answer = distance
        question = f"\\text{{A ladder makes a }} {angle}^\\circ \\text{{ angle with the ground and reaches }} {height} \\text{{ meters up a wall. How far is the base of the ladder from the wall? (in meters)}}"
        difficulty = 1500
        
    elif problem_type == 'special_angles':
        angle1 = random.choice([30, 45, 60])
        angle2 = random.choice([30, 45, 60])
        operation = random.choice(['add', 'multiply'])
        
        trig_values = {
            30: {'sin': sp.Rational(1, 2), 'cos': sp.sqrt(3)/2},
            45: {'sin': sp.sqrt(2)/2, 'cos': sp.sqrt(2)/2},
            60: {'sin': sp.sqrt(3)/2, 'cos': sp.Rational(1, 2)}
        }
        
        func1 = random.choice(['sin', 'cos'])
        func2 = random.choice(['sin', 'cos'])
        
        val1 = trig_values[angle1][func1]
        val2 = trig_values[angle2][func2]
        
        if operation == 'add':
            answer = val1 + val2
            question = f"\\text{{Evaluate }} \\{func1}({angle1}^\\circ) + \\{func2}({angle2}^\\circ)"
            difficulty = 1350
        else:
            answer = val1 * val2
            question = f"\\text{{Evaluate }} \\{func1}({angle1}^\\circ) \\cdot \\{func2}({angle2}^\\circ)"
            difficulty = 1400
            
    else:  # pythagorean_trig
        a = random.randint(3, 8)
        b = random.randint(3, 8)
        c_squared = a**2 + b**2
        c = sp.sqrt(c_squared)
        
        trig_func = random.choice(['sin', 'cos', 'tan'])
        
        if trig_func == 'sin':
            answer = sp.Rational(a, sp.sqrt(c_squared))
        elif trig_func == 'cos':
            answer = sp.Rational(b, sp.sqrt(c_squared))
        else:  # tan
            answer = sp.Rational(a, b)
        
        question = f"\\text{{In a right triangle, the opposite side is }} {a} \\text{{ and the adjacent side is }} {b}\\text{{. Find }} \\{trig_func}(\\theta) \\text{{ where }} \\theta \\text{{ is the angle opposite to the side of length }} {a}"
        difficulty = 1550
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "trigonometry_basics",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))