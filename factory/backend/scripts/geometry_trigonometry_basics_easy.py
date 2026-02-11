"""
geometry - trigonometry_basics (easy)
Generated: 2026-02-11T21:44:26.250279
"""

import random
import json
from sympy import *

def generate_problem():
    problem_type = random.choice([
        'basic_sine',
        'basic_cosine', 
        'basic_tangent',
        'special_angle_sine',
        'special_angle_cosine',
        'special_angle_tangent',
        'simple_right_triangle_side',
        'simple_right_triangle_angle'
    ])
    
    if problem_type == 'basic_sine':
        angle = random.choice([30, 45, 60])
        question = f"Find \\sin({angle}^\\circ)"
        
        if angle == 30:
            answer = "1/2"
        elif angle == 45:
            answer = "sqrt(2)/2"
        else:
            answer = "sqrt(3)/2"
        
        difficulty = 1050
        
    elif problem_type == 'basic_cosine':
        angle = random.choice([30, 45, 60])
        question = f"Find \\cos({angle}^\\circ)"
        
        if angle == 30:
            answer = "sqrt(3)/2"
        elif angle == 45:
            answer = "sqrt(2)/2"
        else:
            answer = "1/2"
        
        difficulty = 1050
        
    elif problem_type == 'basic_tangent':
        angle = random.choice([30, 45, 60])
        question = f"Find \\tan({angle}^\\circ)"
        
        if angle == 30:
            answer = "sqrt(3)/3"
        elif angle == 45:
            answer = "1"
        else:
            answer = "sqrt(3)"
        
        difficulty = 1100
        
    elif problem_type == 'special_angle_sine':
        angle = random.choice([0, 90])
        question = f"Find \\sin({angle}^\\circ)"
        
        if angle == 0:
            answer = "0"
        else:
            answer = "1"
        
        difficulty = 1000
        
    elif problem_type == 'special_angle_cosine':
        angle = random.choice([0, 90])
        question = f"Find \\cos({angle}^\\circ)"
        
        if angle == 0:
            answer = "1"
        else:
            answer = "0"
        
        difficulty = 1000
        
    elif problem_type == 'special_angle_tangent':
        angle = 0
        question = f"Find \\tan({angle}^\\circ)"
        answer = "0"
        difficulty = 1000
        
    elif problem_type == 'simple_right_triangle_side':
        hypotenuse = random.choice([10, 12, 15, 20])
        angle = random.choice([30, 60])
        side_type = random.choice(['opposite', 'adjacent'])
        
        if angle == 30:
            if side_type == 'opposite':
                answer_val = hypotenuse // 2
                question = f"In a right triangle, the hypotenuse is {hypotenuse} and one angle is {angle}^\\circ. Find the length of the side opposite to the {angle}^\\circ angle."
                answer = str(answer_val)
            else:
                answer_val = hypotenuse * sqrt(3) / 2
                question = f"In a right triangle, the hypotenuse is {hypotenuse} and one angle is {angle}^\\circ. Find the length of the side adjacent to the {angle}^\\circ angle."
                answer = str(simplify(answer_val))
        else:
            if side_type == 'opposite':
                answer_val = hypotenuse * sqrt(3) / 2
                question = f"In a right triangle, the hypotenuse is {hypotenuse} and one angle is {angle}^\\circ. Find the length of the side opposite to the {angle}^\\circ angle."
                answer = str(simplify(answer_val))
            else:
                answer_val = hypotenuse // 2
                question = f"In a right triangle, the hypotenuse is {hypotenuse} and one angle is {angle}^\\circ. Find the length of the side adjacent to the {angle}^\\circ angle."
                answer = str(answer_val)
        
        difficulty = 1250
        
    elif problem_type == 'simple_right_triangle_angle':
        opposite = random.choice([3, 4, 5, 6])
        hypotenuse = opposite * 2
        
        question = f"In a right triangle, the side opposite to angle \\theta is {opposite} and the hypotenuse is {hypotenuse}. Find \\sin(\\theta)."
        answer = f"{opposite}/{hypotenuse}"
        answer = str(simplify(sympify(answer)))
        difficulty = 1200
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "trigonometry_basics",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))