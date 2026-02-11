"""
geometry - circles (medium)
Generated: 2026-02-11T21:41:25.903122
"""

import sympy as sp
import random
import json

def generate_circle_problem():
    problem_type = random.choice([
        'circumference_from_radius',
        'area_from_radius',
        'radius_from_circumference',
        'radius_from_area',
        'arc_length',
        'sector_area',
        'chord_length',
        'area_from_diameter'
    ])
    
    if problem_type == 'circumference_from_radius':
        radius = random.choice([3, 4, 5, 6, 7, 8, 9, 10, 12, 15])
        answer = 2 * sp.pi * radius
        question = f"Find the circumference of a circle with radius {radius} units."
        difficulty = 1300
        
    elif problem_type == 'area_from_radius':
        radius = random.choice([3, 4, 5, 6, 7, 8, 10, 12])
        answer = sp.pi * radius**2
        question = f"Find the area of a circle with radius {radius} units."
        difficulty = 1300
        
    elif problem_type == 'radius_from_circumference':
        radius = random.choice([4, 5, 6, 7, 8, 10, 12])
        circumference = 2 * radius
        answer = sp.Integer(radius)
        question = f"Find the radius of a circle with circumference {circumference}\\pi units."
        difficulty = 1350
        
    elif problem_type == 'radius_from_area':
        radius = random.choice([3, 4, 5, 6, 7, 8, 9, 10])
        area = radius**2
        answer = sp.Integer(radius)
        question = f"Find the radius of a circle with area {area}\\pi square units."
        difficulty = 1350
        
    elif problem_type == 'arc_length':
        radius = random.choice([6, 8, 9, 10, 12, 15, 18])
        angle_degrees = random.choice([30, 45, 60, 90, 120, 135, 150])
        angle_radians = sp.pi * angle_degrees / 180
        answer = radius * angle_radians
        question = f"Find the arc length of a circle with radius {radius} units subtended by a central angle of {angle_degrees}^\\circ."
        difficulty = 1450
        
    elif problem_type == 'sector_area':
        radius = random.choice([6, 8, 9, 10, 12])
        angle_degrees = random.choice([30, 45, 60, 90, 120, 150])
        angle_radians = sp.pi * angle_degrees / 180
        answer = sp.Rational(1, 2) * radius**2 * angle_radians
        question = f"Find the area of a sector of a circle with radius {radius} units and central angle {angle_degrees}^\\circ."
        difficulty = 1500
        
    elif problem_type == 'chord_length':
        radius = random.choice([5, 6, 8, 10, 12, 13])
        angle_degrees = random.choice([60, 90, 120])
        angle_radians = sp.pi * angle_degrees / 180
        answer = 2 * radius * sp.sin(angle_radians / 2)
        question = f"Find the length of a chord in a circle with radius {radius} units, where the chord subtends a central angle of {angle_degrees}^\\circ."
        difficulty = 1550
        
    elif problem_type == 'area_from_diameter':
        diameter = random.choice([6, 8, 10, 12, 14, 16, 18, 20])
        radius = diameter // 2
        answer = sp.pi * radius**2
        question = f"Find the area of a circle with diameter {diameter} units."
        difficulty = 1400
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "circles",
        "grading_mode": "equivalent"
    }

problem = generate_circle_problem()
print(json.dumps(problem))