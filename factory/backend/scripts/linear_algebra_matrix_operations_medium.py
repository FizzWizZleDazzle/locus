"""
linear_algebra - matrix_operations (medium)
Generated: 2026-02-11T22:08:59.052257
"""

import sympy as sp
import random
import json

def generate_matrix_operations_problem():
    problem_type = random.choice([
        'matrix_addition',
        'matrix_subtraction', 
        'scalar_multiplication',
        'matrix_multiplication',
        'matrix_multiplication_3x3'
    ])
    
    if problem_type == 'matrix_addition':
        # ELO 1300-1350: Simple 2x2 matrix addition
        a11, a12 = random.randint(-5, 5), random.randint(-5, 5)
        a21, a22 = random.randint(-5, 5), random.randint(-5, 5)
        b11, b12 = random.randint(-5, 5), random.randint(-5, 5)
        b21, b22 = random.randint(-5, 5), random.randint(-5, 5)
        
        A = sp.Matrix([[a11, a12], [a21, a22]])
        B = sp.Matrix([[b11, b12], [b21, b22]])
        answer = A + B
        
        question = f"\\text{{Compute }} A + B \\text{{ where }} A = {sp.latex(A)} \\text{{ and }} B = {sp.latex(B)}"
        difficulty = 1320
        
    elif problem_type == 'matrix_subtraction':
        # ELO 1320-1360: Simple 2x2 matrix subtraction
        a11, a12 = random.randint(-5, 5), random.randint(-5, 5)
        a21, a22 = random.randint(-5, 5), random.randint(-5, 5)
        b11, b12 = random.randint(-5, 5), random.randint(-5, 5)
        b21, b22 = random.randint(-5, 5), random.randint(-5, 5)
        
        A = sp.Matrix([[a11, a12], [a21, a22]])
        B = sp.Matrix([[b11, b12], [b21, b22]])
        answer = A - B
        
        question = f"\\text{{Compute }} A - B \\text{{ where }} A = {sp.latex(A)} \\text{{ and }} B = {sp.latex(B)}"
        difficulty = 1330
        
    elif problem_type == 'scalar_multiplication':
        # ELO 1340-1380: Scalar multiplication with 2x2 or 3x2 matrix
        if random.choice([True, False]):
            a11, a12 = random.randint(-5, 5), random.randint(-5, 5)
            a21, a22 = random.randint(-5, 5), random.randint(-5, 5)
            A = sp.Matrix([[a11, a12], [a21, a22]])
        else:
            a11, a12 = random.randint(-5, 5), random.randint(-5, 5)
            a21, a22 = random.randint(-5, 5), random.randint(-5, 5)
            a31, a32 = random.randint(-5, 5), random.randint(-5, 5)
            A = sp.Matrix([[a11, a12], [a21, a22], [a31, a32]])
        
        scalar = random.randint(-5, 5)
        while scalar == 0 or scalar == 1:
            scalar = random.randint(-5, 5)
        
        answer = scalar * A
        
        question = f"\\text{{Compute }} {scalar} \\cdot A \\text{{ where }} A = {sp.latex(A)}"
        difficulty = 1350
        
    elif problem_type == 'matrix_multiplication':
        # ELO 1400-1500: 2x2 matrix multiplication
        a11, a12 = random.randint(-3, 3), random.randint(-3, 3)
        a21, a22 = random.randint(-3, 3), random.randint(-3, 3)
        b11, b12 = random.randint(-3, 3), random.randint(-3, 3)
        b21, b22 = random.randint(-3, 3), random.randint(-3, 3)
        
        A = sp.Matrix([[a11, a12], [a21, a22]])
        B = sp.Matrix([[b11, b12], [b21, b22]])
        answer = A * B
        
        question = f"\\text{{Compute }} A \\cdot B \\text{{ where }} A = {sp.latex(A)} \\text{{ and }} B = {sp.latex(B)}"
        difficulty = 1450
        
    else:  # matrix_multiplication_3x3
        # ELO 1500-1600: 3x3 matrix multiplication or 2x3 times 3x2
        if random.choice([True, False]):
            # 2x2 with larger numbers or 2x3 times 3x2
            a11, a12, a13 = random.randint(-2, 2), random.randint(-2, 2), random.randint(-2, 2)
            a21, a22, a23 = random.randint(-2, 2), random.randint(-2, 2), random.randint(-2, 2)
            b11, b12 = random.randint(-2, 2), random.randint(-2, 2)
            b21, b22 = random.randint(-2, 2), random.randint(-2, 2)
            b31, b32 = random.randint(-2, 2), random.randint(-2, 2)
            
            A = sp.Matrix([[a11, a12, a13], [a21, a22, a23]])
            B = sp.Matrix([[b11, b12], [b21, b22], [b31, b32]])
            answer = A * B
            
            question = f"\\text{{Compute }} A \\cdot B \\text{{ where }} A = {sp.latex(A)} \\text{{ and }} B = {sp.latex(B)}"
            difficulty = 1520
        else:
            # Combined operation: (A + B)C
            a11, a12 = random.randint(-2, 2), random.randint(-2, 2)
            a21, a22 = random.randint(-2, 2), random.randint(-2, 2)
            b11, b12 = random.randint(-2, 2), random.randint(-2, 2)
            b21, b22 = random.randint(-2, 2), random.randint(-2, 2)
            c11, c12 = random.randint(-2, 2), random.randint(-2, 2)
            c21, c22 = random.randint(-2, 2), random.randint(-2, 2)
            
            A = sp.Matrix([[a11, a12], [a21, a22]])
            B = sp.Matrix([[b11, b12], [b21, b22]])
            C = sp.Matrix([[c11, c12], [c21, c22]])
            answer = (A + B) * C
            
            question = f"\\text{{Compute }} (A + B) \\cdot C \\text{{ where }} A = {sp.latex(A)}, B = {sp.latex(B)}, \\text{{ and }} C = {sp.latex(C)}"
            difficulty = 1580
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "linear_algebra",
        "subtopic": "matrix_operations",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_