"""
linear_algebra - determinants (easy)
Generated: 2026-02-11T22:09:41.338624
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'det_2x2_integers',
        'det_2x2_simple',
        'det_3x3_diagonal',
        'det_3x3_triangular',
        'det_2x2_with_variable'
    ])
    
    if problem_type == 'det_2x2_integers':
        # ELO 1000-1100: Simple 2x2 determinant with small integers
        det_value = random.randint(-10, 10)
        a = random.randint(1, 5)
        b = random.randint(-5, 5)
        c = random.randint(-5, 5)
        d = (det_value + b * c) // a if a != 0 else random.randint(-5, 5)
        
        question = f"\\text{{Calculate the determinant of }} \\begin{{pmatrix}} {a} & {b} \\\\ {c} & {d} \\end{{pmatrix}}"
        answer = a * d - b * c
        difficulty = 1050
        
    elif problem_type == 'det_2x2_simple':
        # ELO 1100-1150: 2x2 determinant with slightly larger numbers
        a = random.randint(-8, 8)
        b = random.randint(-8, 8)
        c = random.randint(-8, 8)
        d = random.randint(-8, 8)
        
        question = f"\\text{{Find }} \\det\\begin{{pmatrix}} {a} & {b} \\\\ {c} & {d} \\end{{pmatrix}}"
        answer = a * d - b * c
        difficulty = 1100
        
    elif problem_type == 'det_3x3_diagonal':
        # ELO 1150-1200: 3x3 diagonal matrix determinant
        a = random.randint(1, 6)
        b = random.randint(1, 6)
        c = random.randint(1, 6)
        
        question = f"\\text{{Compute the determinant of }} \\begin{{pmatrix}} {a} & 0 & 0 \\\\ 0 & {b} & 0 \\\\ 0 & 0 & {c} \\end{{pmatrix}}"
        answer = a * b * c
        difficulty = 1150
        
    elif problem_type == 'det_3x3_triangular':
        # ELO 1200-1250: 3x3 upper or lower triangular matrix
        is_upper = random.choice([True, False])
        a = random.randint(1, 5)
        b = random.randint(1, 5)
        c = random.randint(1, 5)
        
        if is_upper:
            x = random.randint(-4, 4)
            y = random.randint(-4, 4)
            z = random.randint(-4, 4)
            question = f"\\text{{Find the determinant of }} \\begin{{pmatrix}} {a} & {x} & {y} \\\\ 0 & {b} & {z} \\\\ 0 & 0 & {c} \\end{{pmatrix}}"
        else:
            x = random.randint(-4, 4)
            y = random.randint(-4, 4)
            z = random.randint(-4, 4)
            question = f"\\text{{Find the determinant of }} \\begin{{pmatrix}} {a} & 0 & 0 \\\\ {x} & {b} & 0 \\\\ {y} & {z} & {c} \\end{{pmatrix}}"
        
        answer = a * b * c
        difficulty = 1220
        
    else:  # det_2x2_with_variable
        # ELO 1250-1300: 2x2 determinant with one variable
        x = sp.Symbol('x')
        det_value = random.randint(-5, 5)
        a = random.randint(1, 4)
        b = random.randint(-4, 4)
        c = random.randint(-4, 4)
        
        # ad - bc = det_value, where d = x
        # ax - bc = det_value
        # x = (det_value + bc) / a
        
        question = f"\\text{{If }} \\det\\begin{{pmatrix}} {a} & {b} \\\\ {c} & x \\end{{pmatrix}} = {det_value}, \\text{{ find }} x"
        answer_expr = sp.solve(a * x - b * c - det_value, x)[0]
        answer = answer_expr
        difficulty = 1280
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "linear_algebra",
        "subtopic": "determinants",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))