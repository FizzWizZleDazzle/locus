"""
precalculus - vectors (easy)
Generated: 2026-02-11T21:56:45.368528
"""

import sympy as sp
import random
import json

def generate_vector_problem():
    problem_type = random.choice([
        'vector_addition_2d',
        'vector_subtraction_2d',
        'scalar_multiplication',
        'magnitude_simple',
        'component_extraction',
        'vector_addition_3d'
    ])
    
    if problem_type == 'vector_addition_2d':
        # Pick clean answer first
        ans_x = random.randint(-5, 5)
        ans_y = random.randint(-5, 5)
        
        # Work backward
        v1_x = random.randint(-4, 4)
        v1_y = random.randint(-4, 4)
        v2_x = ans_x - v1_x
        v2_y = ans_y - v1_y
        
        question = f"\\text{{Find }} \\vec{{u}} + \\vec{{v}} \\text{{ where }} \\vec{{u}} = \\langle {v1_x}, {v1_y} \\rangle \\text{{ and }} \\vec{{v}} = \\langle {v2_x}, {v2_y} \\rangle"
        answer = f"({ans_x}, {ans_y})"
        difficulty = 1050
        
    elif problem_type == 'vector_subtraction_2d':
        ans_x = random.randint(-5, 5)
        ans_y = random.randint(-5, 5)
        
        v1_x = random.randint(-4, 4)
        v1_y = random.randint(-4, 4)
        v2_x = v1_x - ans_x
        v2_y = v1_y - ans_y
        
        question = f"\\text{{Find }} \\vec{{u}} - \\vec{{v}} \\text{{ where }} \\vec{{u}} = \\langle {v1_x}, {v1_y} \\rangle \\text{{ and }} \\vec{{v}} = \\langle {v2_x}, {v2_y} \\rangle"
        answer = f"({ans_x}, {ans_y})"
        difficulty = 1100
        
    elif problem_type == 'scalar_multiplication':
        scalar = random.choice([2, 3, -2, -3, 4, -4])
        base_x = random.randint(-3, 3)
        base_y = random.randint(-3, 3)
        
        ans_x = scalar * base_x
        ans_y = scalar * base_y
        
        question = f"\\text{{Find }} {scalar}\\vec{{v}} \\text{{ where }} \\vec{{v}} = \\langle {base_x}, {base_y} \\rangle"
        answer = f"({ans_x}, {ans_y})"
        difficulty = 1080
        
    elif problem_type == 'magnitude_simple':
        # Pick Pythagorean triples or simple cases
        pairs = [(3, 4, 5), (5, 12, 13), (8, 6, 10), (0, 5, 5), (3, 0, 3)]
        v_x, v_y, magnitude = random.choice(pairs)
        
        if random.choice([True, False]):
            v_x = -v_x
        if random.choice([True, False]):
            v_y = -v_y
            
        question = f"\\text{{Find the magnitude of }} \\vec{{v}} = \\langle {v_x}, {v_y} \\rangle"
        answer = str(magnitude)
        difficulty = 1200
        
    elif problem_type == 'component_extraction':
        v_x = random.randint(-8, 8)
        v_y = random.randint(-8, 8)
        component = random.choice(['x', 'y'])
        
        if component == 'x':
            question = f"\\text{{Find the x-component of }} \\vec{{v}} = \\langle {v_x}, {v_y} \\rangle"
            answer = str(v_x)
        else:
            question = f"\\text{{Find the y-component of }} \\vec{{v}} = \\langle {v_x}, {v_y} \\rangle"
            answer = str(v_y)
        difficulty = 1000
        
    elif problem_type == 'vector_addition_3d':
        ans_x = random.randint(-4, 4)
        ans_y = random.randint(-4, 4)
        ans_z = random.randint(-4, 4)
        
        v1_x = random.randint(-3, 3)
        v1_y = random.randint(-3, 3)
        v1_z = random.randint(-3, 3)
        v2_x = ans_x - v1_x
        v2_y = ans_y - v1_y
        v2_z = ans_z - v1_z
        
        question = f"\\text{{Find }} \\vec{{u}} + \\vec{{v}} \\text{{ where }} \\vec{{u}} = \\langle {v1_x}, {v1_y}, {v1_z} \\rangle \\text{{ and }} \\vec{{v}} = \\langle {v2_x}, {v2_y}, {v2_z} \\rangle"
        answer = f"({ans_x}, {ans_y}, {ans_z})"
        difficulty = 1150
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "precalculus",
        "subtopic": "vectors",
        "grading_mode": "equivalent"
    }

problem = generate_vector_problem()
print(json.dumps(problem))