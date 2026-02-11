"""
precalculus - matrices (easy)
Generated: 2026-02-11T21:57:58.334889
"""

import sympy as sp
import random
import json

def generate_matrix_problem():
    problem_type = random.choice([
        'matrix_addition',
        'scalar_multiplication',
        'matrix_element',
        'matrix_subtraction',
        'simple_multiplication_2x2'
    ])
    
    if problem_type == 'matrix_addition':
        # Pick clean answer matrix first
        ans_a = random.randint(-5, 5)
        ans_b = random.randint(-5, 5)
        ans_c = random.randint(-5, 5)
        ans_d = random.randint(-5, 5)
        
        # Construct two matrices that add to answer
        a1 = random.randint(-5, 5)
        a2 = ans_a - a1
        b1 = random.randint(-5, 5)
        b2 = ans_b - b1
        c1 = random.randint(-5, 5)
        c2 = ans_c - c1
        d1 = random.randint(-5, 5)
        d2 = ans_d - d1
        
        M1 = sp.Matrix([[a1, b1], [c1, d1]])
        M2 = sp.Matrix([[a2, b2], [c2, d2]])
        answer = sp.Matrix([[ans_a, ans_b], [ans_c, ans_d]])
        
        question = f"\\text{{Add the matrices: }} {sp.latex(M1)} + {sp.latex(M2)}"
        difficulty = 1100
        
    elif problem_type == 'scalar_multiplication':
        # Pick scalar and answer
        scalar = random.choice([2, 3, 4, 5, -2, -3])
        ans_a = random.randint(-10, 10)
        ans_b = random.randint(-10, 10)
        ans_c = random.randint(-10, 10)
        ans_d = random.randint(-10, 10)
        
        # Reverse engineer the original matrix
        if ans_a % scalar == 0 and ans_b % scalar == 0 and ans_c % scalar == 0 and ans_d % scalar == 0:
            orig_a = ans_a // scalar
            orig_b = ans_b // scalar
            orig_c = ans_c // scalar
            orig_d = ans_d // scalar
        else:
            orig_a = random.randint(-5, 5)
            orig_b = random.randint(-5, 5)
            orig_c = random.randint(-5, 5)
            orig_d = random.randint(-5, 5)
            ans_a = scalar * orig_a
            ans_b = scalar * orig_b
            ans_c = scalar * orig_c
            ans_d = scalar * orig_d
        
        M = sp.Matrix([[orig_a, orig_b], [orig_c, orig_d]])
        answer = sp.Matrix([[ans_a, ans_b], [ans_c, ans_d]])
        
        question = f"\\text{{Multiply the matrix by the scalar: }} {scalar} \\cdot {sp.latex(M)}"
        difficulty = 1050
        
    elif problem_type == 'matrix_element':
        # Pick answer (a single element)
        ans = random.randint(-10, 10)
        
        # Create a 2x3 or 3x2 matrix
        rows = random.choice([2, 3])
        cols = 3 if rows == 2 else 2
        
        row_idx = random.randint(1, rows)
        col_idx = random.randint(1, cols)
        
        elements = [[random.randint(-10, 10) for _ in range(cols)] for _ in range(rows)]
        elements[row_idx-1][col_idx-1] = ans
        
        M = sp.Matrix(elements)
        answer = sp.sympify(ans)
        
        question = f"\\text{{Find the element in row }} {row_idx} \\text{{ and column }} {col_idx} \\text{{ of matrix }} {sp.latex(M)}"
        difficulty = 1000
        
    elif problem_type == 'matrix_subtraction':
        # Pick clean answer matrix first
        ans_a = random.randint(-5, 5)
        ans_b = random.randint(-5, 5)
        ans_c = random.randint(-5, 5)
        ans_d = random.randint(-5, 5)
        
        # Construct two matrices where M1 - M2 = answer
        a2 = random.randint(-5, 5)
        a1 = ans_a + a2
        b2 = random.randint(-5, 5)
        b1 = ans_b + b2
        c2 = random.randint(-5, 5)
        c1 = ans_c + c2
        d2 = random.randint(-5, 5)
        d1 = ans_d + d2
        
        M1 = sp.Matrix([[a1, b1], [c1, d1]])
        M2 = sp.Matrix([[a2, b2], [c2, d2]])
        answer = sp.Matrix([[ans_a, ans_b], [ans_c, ans_d]])
        
        question = f"\\text{{Subtract the matrices: }} {sp.latex(M1)} - {sp.latex(M2)}"
        difficulty = 1150
        
    else:  # simple_multiplication_2x2
        # Pick answer matrix
        ans_a = random.randint(-10, 10)
        ans_b = random.randint(-10, 10)
        ans_c = random.randint(-10, 10)
        ans_d = random.randint(-10, 10)
        
        # Create simple 2x2 matrices
        a1, a2 = random.randint(-3, 3), random.randint(-3, 3)
        b1, b2 = random.randint(-3, 3), random.randint(-3, 3)
        c1, c2 = random.randint(-3, 3), random.randint(-3, 3)
        d1, d2 = random.randint(-3, 3), random.randint(-3, 3)
        
        M1 = sp.Matrix([[a1, a2], [b1, b2]])
        M2 = sp.Matrix([[c1, c2], [d1, d2]])
        answer = M1 * M2
        
        question = f"\\text{{Multiply the matrices: }} {sp.latex(M1)} \\cdot {sp.latex(M2)}"
        difficulty = 1300
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "precalculus",
        "subtopic": "matrices",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_matrix_problem()
print(json.dumps(problem))