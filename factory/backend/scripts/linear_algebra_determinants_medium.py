"""
linear_algebra - determinants (medium)
Generated: 2026-02-11T22:09:57.759935
"""

import random
import json
from sympy import Matrix, symbols, simplify, latex

def generate_determinant_problem():
    problem_type = random.choice([
        "2x2_basic",
        "2x2_symbolic", 
        "3x3_basic",
        "3x3_row_ops",
        "2x2_equation"
    ])
    
    if problem_type == "2x2_basic":
        target_det = random.randint(-20, 20)
        a = random.randint(-5, 5)
        b = random.randint(-5, 5)
        c = random.randint(-5, 5)
        
        if a != 0:
            d = (target_det + b * c) // a if a != 0 else random.randint(-5, 5)
        else:
            a = random.randint(1, 5)
            d = (target_det + b * c) // a
        
        matrix = Matrix([[a, b], [c, d]])
        answer = matrix.det()
        
        question = f"\\text{{Calculate the determinant: }} \\begin{{vmatrix}} {a} & {b} \\\\ {c} & {d} \\end{{vmatrix}}"
        difficulty = 1300
        
    elif problem_type == "2x2_symbolic":
        x = symbols('x')
        a = random.randint(-3, 3)
        b = random.randint(1, 4)
        c = random.randint(-3, 3)
        d = random.randint(1, 4)
        
        matrix = Matrix([[a, b], [c, x]])
        answer = simplify(matrix.det())
        
        question = f"\\text{{Find the determinant in terms of }} x: \\begin{{vmatrix}} {a} & {b} \\\\ {c} & x \\end{{vmatrix}}"
        difficulty = 1400
        
    elif problem_type == "3x3_basic":
        target_det = random.randint(-30, 30)
        
        a11 = random.randint(1, 3)
        a12 = random.randint(-2, 2)
        a13 = random.randint(-2, 2)
        a21 = random.randint(-2, 2)
        a22 = random.randint(1, 3)
        a23 = random.randint(-2, 2)
        a31 = random.randint(-2, 2)
        a32 = random.randint(-2, 2)
        a33 = random.randint(1, 3)
        
        matrix = Matrix([[a11, a12, a13], [a21, a22, a23], [a31, a32, a33]])
        answer = matrix.det()
        
        question = f"\\text{{Calculate the determinant: }} \\begin{{vmatrix}} {a11} & {a12} & {a13} \\\\ {a21} & {a22} & {a23} \\\\ {a31} & {a32} & {a33} \\end{{vmatrix}}"
        difficulty = 1500
        
    elif problem_type == "3x3_row_ops":
        a = random.randint(1, 4)
        b = random.randint(-3, 3)
        c = random.randint(-3, 3)
        d = random.randint(1, 4)
        e = random.randint(-3, 3)
        f = random.randint(1, 4)
        
        matrix = Matrix([[a, b, c], [0, d, e], [0, 0, f]])
        answer = matrix.det()
        
        question = f"\\text{{Calculate the determinant: }} \\begin{{vmatrix}} {a} & {b} & {c} \\\\ 0 & {d} & {e} \\\\ 0 & 0 & {f} \\end{{vmatrix}}"
        difficulty = 1400
        
    else:
        x = symbols('x')
        a = random.randint(1, 4)
        b = random.randint(-3, 3)
        c = random.randint(-3, 3)
        d = random.randint(1, 4)
        
        target_det = random.randint(-15, 15)
        
        matrix = Matrix([[a, b], [c, d]])
        actual_det = matrix.det()
        
        if actual_det != 0:
            solution = target_det // actual_det if actual_det != 0 else 1
        else:
            solution = 0
        
        matrix_x = Matrix([[a*x, b*x], [c, d]])
        answer = solution
        
        question = f"\\text{{Find }} x \\text{{ such that }} \\begin{{vmatrix}} {a}x & {b}x \\\\ {c} & {d} \\end{{vmatrix}} = {target_det}"
        difficulty = 1550
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "linear_algebra",
        "subtopic": "determinants",
        "grading_mode": "equivalent"
    }

problem = generate_determinant_problem()
print(json.dumps(problem))