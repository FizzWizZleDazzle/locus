"""
geometry - volume_surface_area (easy)
Generated: 2026-02-11T21:42:41.960487
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'rectangular_prism_volume',
        'cube_volume',
        'cylinder_volume',
        'rectangular_prism_surface_area',
        'cube_surface_area',
        'sphere_volume',
        'sphere_surface_area'
    ])
    
    if problem_type == 'rectangular_prism_volume':
        length = random.randint(2, 10)
        width = random.randint(2, 10)
        height = random.randint(2, 10)
        
        answer = length * width * height
        question = f"Find the volume of a rectangular prism with length {length} units, width {width} units, and height {height} units."
        difficulty = 1100
        
    elif problem_type == 'cube_volume':
        side = random.randint(2, 10)
        answer = side ** 3
        question = f"Find the volume of a cube with side length {side} units."
        difficulty = 1050
        
    elif problem_type == 'cylinder_volume':
        radius = random.randint(2, 8)
        height = random.randint(3, 12)
        
        answer = sp.pi * radius**2 * height
        question = f"Find the volume of a cylinder with radius {radius} units and height {height} units. Leave your answer in terms of \\pi."
        difficulty = 1200
        
    elif problem_type == 'rectangular_prism_surface_area':
        length = random.randint(2, 8)
        width = random.randint(2, 8)
        height = random.randint(2, 8)
        
        answer = 2 * (length * width + length * height + width * height)
        question = f"Find the surface area of a rectangular prism with length {length} units, width {width} units, and height {height} units."
        difficulty = 1250
        
    elif problem_type == 'cube_surface_area':
        side = random.randint(2, 10)
        answer = 6 * side**2
        question = f"Find the surface area of a cube with side length {side} units."
        difficulty = 1150
        
    elif problem_type == 'sphere_volume':
        radius = random.randint(2, 9)
        answer = sp.Rational(4, 3) * sp.pi * radius**3
        question = f"Find the volume of a sphere with radius {radius} units. Leave your answer in terms of \\pi."
        difficulty = 1300
        
    elif problem_type == 'sphere_surface_area':
        radius = random.randint(2, 9)
        answer = 4 * sp.pi * radius**2
        question = f"Find the surface area of a sphere with radius {radius} units. Leave your answer in terms of \\pi."
        difficulty = 1250
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "volume_surface_area",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_problem()
print(json.dumps(problem))