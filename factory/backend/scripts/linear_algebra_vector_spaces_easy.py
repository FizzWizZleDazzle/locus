"""
linear_algebra - vector_spaces (easy)
Generated: 2026-02-11T22:10:39.448389
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'check_vector_in_span',
        'linear_combination',
        'check_subspace_simple',
        'zero_vector'
    ])
    
    if problem_type == 'check_vector_in_span':
        # Pick a clean answer: yes, the vector is in the span
        # Construct: v = a*v1 + b*v2 where a, b are integers
        a = random.randint(-2, 3)
        b = random.randint(-2, 3)
        
        # Simple 2D vectors
        v1_x = random.randint(-3, 3)
        v1_y = random.randint(-3, 3)
        v2_x = random.randint(-3, 3)
        v2_y = random.randint(-3, 3)
        
        # Make sure v1 and v2 are not parallel
        if v1_x * v2_y == v1_y * v2_x:
            v2_x += 1
        
        v_x = a * v1_x + b * v2_x
        v_y = a * v1_y + b * v2_y
        
        question = f"Is the vector \\begin{{pmatrix}} {v_x} \\\\ {v_y} \\end{{pmatrix}} in the span of \\begin{{pmatrix}} {v1_x} \\\\ {v1_y} \\end{{pmatrix}} and \\begin{{pmatrix}} {v2_x} \\\\ {v2_y} \\end{{pmatrix}}? Answer 'yes' or 'no'."
        answer = "yes"
        difficulty = 1250
        
    elif problem_type == 'linear_combination':
        # Express vector as linear combination
        # Pick coefficients first
        a = random.randint(-2, 3)
        b = random.randint(-2, 3)
        
        v1_x = random.randint(1, 3)
        v1_y = random.randint(-2, 2)
        v2_x = random.randint(-2, 2)
        v2_y = random.randint(1, 3)
        
        v_x = a * v1_x + b * v2_x
        v_y = a * v1_y + b * v2_y
        
        question = f"Write \\begin{{pmatrix}} {v_x} \\\\ {v_y} \\end{{pmatrix}} as a linear combination of \\begin{{pmatrix}} {v1_x} \\\\ {v1_y} \\end{{pmatrix}} and \\begin{{pmatrix}} {v2_x} \\\\ {v2_y} \\end{{pmatrix}}. Find the coefficient of the first vector."
        answer = str(a)
        difficulty = 1200
        
    elif problem_type == 'check_subspace_simple':
        # Simple check: does it contain zero vector?
        c = random.randint(1, 5)
        question = f"Is the set of all vectors \\begin{{pmatrix}} x \\\\ y \\end{{pmatrix}} where x + y = {c} a subspace of \\mathbb{{R}}^2? Answer 'yes' or 'no'."
        answer = "no"
        difficulty = 1100
        
    elif problem_type == 'zero_vector':
        # Find the zero vector in R^n
        n = random.choice([2, 3])
        if n == 2:
            question = "What is the zero vector in \\mathbb{R}^2? Write as (x, y)."
            answer = "(0, 0)"
        else:
            question = "What is the zero vector in \\mathbb{R}^3? Write as (x, y, z)."
            answer = "(0, 0, 0)"
        difficulty = 1000
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "linear_algebra",
        "subtopic": "vector_spaces",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))