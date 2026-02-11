"""
geometry - volume_surface_area (medium)
Generated: 2026-02-11T21:42:58.478132
"""

import random
import json
from sympy import symbols, pi, sqrt, simplify, latex

def generate_problem():
    problem_type = random.choice([
        'rectangular_prism_volume',
        'rectangular_prism_surface',
        'cylinder_volume',
        'cylinder_surface',
        'sphere_volume',
        'sphere_surface',
        'cone_volume',
        'cone_surface',
        'composite_shape'
    ])
    
    if problem_type == 'rectangular_prism_volume':
        l = random.randint(3, 12)
        w = random.randint(3, 12)
        h = random.randint(3, 12)
        answer = l * w * h
        question = f"Find the volume of a rectangular prism with length {l} units, width {w} units, and height {h} units."
        difficulty = 1300
        
    elif problem_type == 'rectangular_prism_surface':
        l = random.randint(3, 10)
        w = random.randint(3, 10)
        h = random.randint(3, 10)
        answer = 2 * (l*w + l*h + w*h)
        question = f"Find the surface area of a rectangular prism with length {l} units, width {w} units, and height {h} units."
        difficulty = 1400
        
    elif problem_type == 'cylinder_volume':
        r = random.randint(2, 8)
        h = random.randint(4, 15)
        answer = pi * r**2 * h
        question = f"Find the volume of a cylinder with radius {r} units and height {h} units. Express your answer in terms of \\pi."
        difficulty = 1350
        
    elif problem_type == 'cylinder_surface':
        r = random.randint(2, 7)
        h = random.randint(4, 12)
        answer = 2 * pi * r * (r + h)
        question = f"Find the total surface area of a cylinder with radius {r} units and height {h} units. Express your answer in terms of \\pi."
        difficulty = 1450
        
    elif problem_type == 'sphere_volume':
        r = random.randint(3, 9)
        answer = simplify(4 * pi * r**3 / 3)
        question = f"Find the volume of a sphere with radius {r} units. Express your answer in terms of \\pi."
        difficulty = 1400
        
    elif problem_type == 'sphere_surface':
        r = random.randint(3, 10)
        answer = 4 * pi * r**2
        question = f"Find the surface area of a sphere with radius {r} units. Express your answer in terms of \\pi."
        difficulty = 1350
        
    elif problem_type == 'cone_volume':
        r = random.randint(3, 9)
        h = random.randint(6, 15)
        answer = simplify(pi * r**2 * h / 3)
        question = f"Find the volume of a cone with radius {r} units and height {h} units. Express your answer in terms of \\pi."
        difficulty = 1450
        
    elif problem_type == 'cone_surface':
        r = random.randint(3, 8)
        h = random.randint(4, 12)
        l = sqrt(r**2 + h**2)
        answer = simplify(pi * r * (r + l))
        question = f"Find the total surface area of a cone with radius {r} units and height {h} units. Express your answer in simplified radical form with \\pi."
        difficulty = 1550
        
    elif problem_type == 'composite_shape':
        shape_combo = random.choice(['cylinder_hemisphere', 'prism_pyramid'])
        
        if shape_combo == 'cylinder_hemisphere':
            r = random.randint(3, 7)
            h = random.randint(5, 12)
            cyl_vol = pi * r**2 * h
            hem_vol = simplify(2 * pi * r**3 / 3)
            answer = simplify(cyl_vol + hem_vol)
            question = f"A solid consists of a cylinder with radius {r} units and height {h} units, topped with a hemisphere of the same radius. Find the total volume. Express your answer in terms of \\pi."
            difficulty = 1550
        else:
            side = random.randint(4, 10)
            h1 = random.randint(6, 12)
            h2 = random.randint(3, 8)
            prism_vol = side**2 * h1
            pyramid_vol = simplify(side**2 * h2 / 3)
            answer = simplify(prism_vol + pyramid_vol)
            question = f"A solid consists of a square prism with base side length {side} units and height {h1} units, topped with a square pyramid with the same base and height {h2} units. Find the total volume."
            difficulty = 1500
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "volume_surface_area",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))