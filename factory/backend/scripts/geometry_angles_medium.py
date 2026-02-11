"""
geometry - angles (medium)
Generated: 2026-02-11T21:39:25.969029
"""

import random
import json
from sympy import symbols, Eq, solve, simplify, latex, pi, rad, deg

def generate_angle_problem():
    problem_type = random.choice([
        'triangle_missing_angle',
        'supplementary_angles',
        'complementary_angles',
        'vertical_angles',
        'linear_pair',
        'triangle_exterior_angle',
        'parallel_lines_transversal',
        'polygon_interior_angles'
    ])
    
    if problem_type == 'triangle_missing_angle':
        angle3 = random.randint(40, 100)
        angle2 = random.randint(30, 180 - angle3 - 20)
        angle1 = 180 - angle2 - angle3
        
        question = f"In a triangle, two angles measure ${angle1}^\\circ$ and ${angle2}^\\circ$. Find the measure of the third angle in degrees."
        answer = str(angle3)
        difficulty = 1300
        
    elif problem_type == 'supplementary_angles':
        answer_val = random.randint(60, 120)
        other_angle = 180 - answer_val
        
        question = f"Two angles are supplementary. If one angle measures ${other_angle}^\\circ$, find the measure of the other angle in degrees."
        answer = str(answer_val)
        difficulty = 1300
        
    elif problem_type == 'complementary_angles':
        answer_val = random.randint(30, 60)
        other_angle = 90 - answer_val
        
        question = f"Two angles are complementary. If one angle measures ${other_angle}^\\circ$, find the measure of the other angle in degrees."
        answer = str(answer_val)
        difficulty = 1300
        
    elif problem_type == 'vertical_angles':
        x_val = random.randint(10, 30)
        angle_val = random.randint(50, 130)
        coeff = random.randint(2, 5)
        constant = angle_val - coeff * x_val
        
        question = f"Two vertical angles are represented by $({coeff}x + {constant})^\\circ$ and ${angle_val}^\\circ$. Find the value of $x$."
        answer = str(x_val)
        difficulty = 1400
        
    elif problem_type == 'linear_pair':
        x_val = random.randint(15, 40)
        coeff1 = random.randint(2, 4)
        coeff2 = random.randint(2, 4)
        constant1 = random.randint(10, 40)
        constant2 = 180 - coeff1 * x_val - coeff2 * x_val - constant1
        
        question = f"Two angles form a linear pair. The angles measure $({coeff1}x + {constant1})^\\circ$ and $({coeff2}x + {constant2})^\\circ$. Find the value of $x$."
        answer = str(x_val)
        difficulty = 1450
        
    elif problem_type == 'triangle_exterior_angle':
        remote1 = random.randint(40, 70)
        remote2 = random.randint(40, 70)
        exterior = remote1 + remote2
        
        question = f"An exterior angle of a triangle measures ${exterior}^\\circ$. If one of the remote interior angles measures ${remote1}^\\circ$, find the measure of the other remote interior angle in degrees."
        answer = str(remote2)
        difficulty = 1400
        
    elif problem_type == 'parallel_lines_transversal':
        angle_val = random.randint(60, 120)
        
        angle_types = [
            ('corresponding', angle_val),
            ('alternate interior', angle_val),
            ('alternate exterior', angle_val),
            ('consecutive interior', 180 - angle_val)
        ]
        
        angle_type, answer_val = random.choice(angle_types)
        
        question = f"Two parallel lines are cut by a transversal. If one angle measures ${angle_val}^\\circ$, find the measure of its {angle_type} angle in degrees."
        answer = str(answer_val)
        difficulty = 1450
        
    elif problem_type == 'polygon_interior_angles':
        n = random.randint(5, 8)
        sum_angles = (n - 2) * 180
        
        question = f"Find the sum of the interior angles of a polygon with ${n}$ sides. Give your answer in degrees."
        answer = str(sum_angles)
        difficulty = 1500
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "angles",
        "grading_mode": "equivalent"
    }

problem = generate_angle_problem()
print(json.dumps(problem))