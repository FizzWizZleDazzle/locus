"""
geometry - circles (easy)
Generated: 2026-02-11T21:41:12.730360
"""

import sympy as sp
import random
import json

def generate_circle_problem():
    problem_type = random.choice([
        'radius_to_diameter',
        'diameter_to_radius', 
        'radius_to_circumference',
        'diameter_to_circumference',
        'radius_to_area',
        'circumference_to_radius',
        'area_to_radius'
    ])
    
    if problem_type == 'radius_to_diameter':
        radius = random.randint(2, 20)
        diameter = 2 * radius
        question = f"A circle has a radius of {radius} units. What is its diameter?"
        answer = str(diameter)
        difficulty = 1050
        
    elif problem_type == 'diameter_to_radius':
        diameter = random.randint(4, 40)
        if diameter % 2 == 1:
            diameter += 1
        radius = diameter // 2
        question = f"A circle has a diameter of {diameter} units. What is its radius?"
        answer = str(radius)
        difficulty = 1050
        
    elif problem_type == 'radius_to_circumference':
        radius = random.randint(2, 15)
        circumference = 2 * sp.pi * radius
        question = f"A circle has a radius of {radius} units. What is its circumference? (Use \\pi in your answer)"
        answer = str(circumference)
        difficulty = 1150
        
    elif problem_type == 'diameter_to_circumference':
        diameter = random.randint(3, 20)
        circumference = sp.pi * diameter
        question = f"A circle has a diameter of {diameter} units. What is its circumference? (Use \\pi in your answer)"
        answer = str(circumference)
        difficulty = 1150
        
    elif problem_type == 'radius_to_area':
        radius = random.randint(2, 12)
        area = sp.pi * radius**2
        question = f"A circle has a radius of {radius} units. What is its area? (Use \\pi in your answer)"
        answer = str(area)
        difficulty = 1200
        
    elif problem_type == 'circumference_to_radius':
        radius = random.randint(3, 12)
        circumference = 2 * sp.pi * radius
        question = f"A circle has a circumference of ${sp.latex(circumference)}$ units. What is its radius?"
        answer = str(radius)
        difficulty = 1250
        
    elif problem_type == 'area_to_radius':
        radius = random.randint(2, 10)
        area = sp.pi * radius**2
        question = f"A circle has an area of ${sp.latex(area)}$ square units. What is its radius?"
        answer = str(radius)
        difficulty = 1300
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "circles",
        "grading_mode": "equivalent"
    }

problem = generate_circle_problem()
print(json.dumps(problem))