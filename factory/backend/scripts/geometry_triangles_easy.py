"""
geometry - triangles (easy)
Generated: 2026-02-11T21:40:06.027079
"""

import sympy as sp
import random
import json

def generate_triangle_problem():
    problem_type = random.choice([
        'angle_sum_one_missing',
        'angle_sum_two_given',
        'right_angle_find_third',
        'isosceles_equal_angles',
        'pythagorean_basic',
        'pythagorean_given_both_legs'
    ])
    
    if problem_type == 'angle_sum_one_missing':
        angle1 = random.choice([30, 40, 45, 50, 60, 70, 80])
        angle2 = random.choice([30, 40, 45, 50, 60, 70, 80])
        if angle1 + angle2 >= 180:
            angle2 = 180 - angle1 - random.choice([20, 30, 40, 50])
        answer = 180 - angle1 - angle2
        question = f"In a triangle, two angles measure ${angle1}^\\circ$ and ${angle2}^\\circ$. Find the measure of the third angle in degrees."
        elo = 1050
        
    elif problem_type == 'angle_sum_two_given':
        angle1 = random.choice([25, 35, 45, 55, 65])
        answer = random.choice([30, 40, 50, 60, 70])
        angle2 = 180 - angle1 - answer
        question = f"A triangle has angles measuring ${angle1}^\\circ$, ${angle2}^\\circ$, and $x^\\circ$. What is $x$?"
        elo = 1100
        
    elif problem_type == 'right_angle_find_third':
        angle1 = random.choice([30, 40, 45, 50, 60])
        answer = 90 - angle1
        question = f"In a right triangle, one acute angle measures ${angle1}^\\circ$. Find the other acute angle in degrees."
        elo = 1150
        
    elif problem_type == 'isosceles_equal_angles':
        base_angle = random.choice([50, 55, 60, 65, 70])
        answer = 180 - 2 * base_angle
        question = f"An isosceles triangle has two equal base angles, each measuring ${base_angle}^\\circ$. Find the vertex angle in degrees."
        elo = 1200
        
    elif problem_type == 'pythagorean_basic':
        a = random.choice([3, 5, 6, 8])
        b = random.choice([4, 12])
        if a == 3 and b == 4:
            answer = 5
        elif a == 5 and b == 12:
            answer = 13
        elif a == 6 and b == 8:
            answer = 10
        elif a == 8 and b == 6:
            answer = 10
        else:
            a, b = 3, 4
            answer = 5
        question = f"In a right triangle, the two legs have lengths ${a}$ and ${b}$. Find the length of the hypotenuse."
        elo = 1250
        
    elif problem_type == 'pythagorean_given_both_legs':
        pairs = [(3, 4, 5), (5, 12, 13), (8, 15, 17), (7, 24, 25)]
        a, b, c = random.choice(pairs)
        question = f"A right triangle has legs of length ${a}$ and ${b}$. What is the length of the hypotenuse?"
        answer = c
        elo = 1280
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "geometry",
        "subtopic": "triangles",
        "grading_mode": "equivalent"
    }

problem = generate_triangle_problem()
print(json.dumps(problem))