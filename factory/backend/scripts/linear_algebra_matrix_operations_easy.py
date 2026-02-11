"""
linear_algebra - matrix_operations (easy)
Generated: 2026-02-11T22:08:38.319451
"""

import random
import json
from sympy import Matrix, symbols, simplify, latex

def generate_problem():
    problem_type = random.choice([
        'matrix_addition',
        'scalar_multiplication',
        'matrix_subtraction',
        'simple_matrix_multiplication'
    ])
    
    if problem_type == 'matrix_addition':
        # ELO 1000-1150: Direct matrix addition, 2x2 matrices with small integers
        rows, cols = 2, 2
        
        # Pick clean answer first
        answer = Matrix([[random.randint(-5, 5) for _ in range(cols)] for _ in range(rows)])
        
        # Construct problem backward
        A = Matrix([[random.randint(-5, 5) for _ in range(cols)] for _ in range(rows)])
        B = answer - A
        
        question = f"\\text{{Compute }} {latex(A)} + {latex(B)}"
        difficulty = random.randint(1000, 1150)
        
    elif problem_type == 'scalar_multiplication':
        # ELO 1050-1200: Scalar multiplication of 2x2 matrix
        rows, cols = 2, 2
        scalar = random.choice([2, 3, 4, 5, -2, -3])
        
        # Pick clean answer first
        base_matrix = Matrix([[random.randint(-3, 3) for _ in range(cols)] for _ in range(rows)])
        answer = scalar * base_matrix
        
        question = f"\\text{{Compute }} {scalar} \\cdot {latex(base_matrix)}"
        difficulty = random.randint(1050, 1200)
        
    elif problem_type == 'matrix_subtraction':
        # ELO 1100-1250: Matrix subtraction, 2x2 matrices
        rows, cols = 2, 2
        
        # Pick clean answer first
        answer = Matrix([[random.randint(-5, 5) for _ in range(cols)] for _ in range(rows)])
        
        # Construct problem backward
        A = Matrix([[random.randint(-5, 5) for _ in range(cols)] for _ in range(rows)])
        B = A - answer
        
        question = f"\\text{{Compute }} {latex(A)} - {latex(B)}"
        difficulty = random.randint(1100, 1250)
        
    else:  # simple_matrix_multiplication
        # ELO 1200-1300: 2x2 matrix multiplication with small integers
        
        # Pick clean answer first with small integers
        answer = Matrix([[random.randint(-4, 4) for _ in range(2)] for _ in range(2)])
        
        # Construct simple matrices that multiply to give answer
        # Use simple form: diagonal or simple patterns
        if random.choice([True, False]):
            # Use identity-like matrix
            A = Matrix([[1, 0], [0, 1]])
            B = answer
        else:
            # Use simple 2x2 matrices
            A = Matrix([[random.randint(1, 2), 0], [0, random.randint(1, 2)]])
            # Construct B from answer
            B = Matrix([[random.randint(-2, 2), random.randint(-2, 2)], 
                       [random.randint(-2, 2), random.randint(-2, 2)]])
            answer = A * B
        
        question = f"\\text{{Compute }} {latex(A)} \\cdot {latex(B)}"
        difficulty = random.randint(1200, 1300)
    
    result = {
        "question_latex": question,
        "answer_key": str(answer.tolist()),
        "difficulty": difficulty,
        "main_topic": "linear_algebra",
        "subtopic": "matrix_operations",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_problem()
print(json.dumps(problem))