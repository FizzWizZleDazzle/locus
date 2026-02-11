"""
precalculus - matrices (medium)
Generated: 2026-02-11T21:58:18.272313
"""

import random
import json
from sympy import Matrix, symbols, simplify, latex

def generate_matrix_problem():
    problem_type = random.choice([
        'matrix_addition',
        'scalar_multiplication',
        'matrix_multiplication',
        'determinant_2x2',
        'determinant_3x3',
        'inverse_2x2',
        'solve_system_2x2'
    ])
    
    if problem_type == 'matrix_addition':
        result = Matrix([[random.randint(-5, 5) for _ in range(2)] for _ in range(2)])
        A = Matrix([[random.randint(-5, 5) for _ in range(2)] for _ in range(2)])
        B = result - A
        
        question = f"\\text{{Find }} A + B \\text{{ where }} A = {latex(A)} \\text{{ and }} B = {latex(B)}"
        answer = result
        elo = 1250
        
    elif problem_type == 'scalar_multiplication':
        k = random.randint(2, 5)
        result = Matrix([[random.randint(-4, 4) for _ in range(2)] for _ in range(2)])
        A = Matrix([[result[i,j] // k if result[i,j] % k == 0 else random.randint(-3, 3) for j in range(2)] for i in range(2)])
        result = k * A
        
        question = f"\\text{{Find }} {k}A \\text{{ where }} A = {latex(A)}"
        answer = result
        elo = 1200
        
    elif problem_type == 'matrix_multiplication':
        result = Matrix([[random.randint(-10, 10) for _ in range(2)] for _ in range(2)])
        A = Matrix([[random.randint(-2, 2) for _ in range(2)] for _ in range(2)])
        
        if A.det() != 0:
            B = A.inv() * result
            B = Matrix([[int(round(B[i,j])) if abs(B[i,j] - round(B[i,j])) < 0.01 else B[i,j] for j in range(2)] for i in range(2)])
            result = A * B
        else:
            B = Matrix([[random.randint(-2, 2) for _ in range(2)] for _ in range(2)])
            result = A * B
        
        question = f"\\text{{Find }} AB \\text{{ where }} A = {latex(A)} \\text{{ and }} B = {latex(B)}"
        answer = result
        elo = 1400
        
    elif problem_type == 'determinant_2x2':
        det_value = random.randint(-10, 10)
        a = random.randint(-3, 3)
        b = random.randint(-3, 3)
        c = random.randint(-3, 3)
        d = (det_value + b*c) // a if a != 0 else random.randint(-3, 3)
        
        A = Matrix([[a, b], [c, d]])
        
        question = f"\\text{{Find the determinant of }} A = {latex(A)}"
        answer = A.det()
        elo = 1300
        
    elif problem_type == 'determinant_3x3':
        det_value = random.randint(-5, 5)
        A = Matrix([[random.randint(-2, 2) for _ in range(3)] for _ in range(3)])
        
        question = f"\\text{{Find the determinant of }} A = {latex(A)}"
        answer = A.det()
        elo = 1500
        
    elif problem_type == 'inverse_2x2':
        det_val = random.choice([-2, -1, 1, 2])
        a = random.randint(-2, 2)
        b = random.randint(-2, 2)
        c = random.randint(-2, 2)
        d = (det_val + b*c) // a if a != 0 and (det_val + b*c) % a == 0 else random.randint(-2, 2)
        
        A = Matrix([[a, b], [c, d]])
        
        if A.det() != 0:
            question = f"\\text{{Find the inverse of }} A = {latex(A)}"
            answer = A.inv()
            elo = 1550
        else:
            A = Matrix([[2, 1], [1, 1]])
            question = f"\\text{{Find the inverse of }} A = {latex(A)}"
            answer = A.inv()
            elo = 1550
            
    elif problem_type == 'solve_system_2x2':
        x_val = random.randint(-3, 3)
        y_val = random.randint(-3, 3)
        
        a = random.randint(1, 3)
        b = random.randint(-2, 2)
        c = random.randint(-2, 2)
        d = random.randint(1, 3)
        
        e = a * x_val + b * y_val
        f = c * x_val + d * y_val
        
        A = Matrix([[a, b], [c, d]])
        
        if A.det() != 0:
            question = f"\\text{{Solve the system using matrices: }} {a}x + {b}y = {e}, \\; {c}x + {d}y = {f}"
            answer = Matrix([x_val, y_val])
            elo = 1450
        else:
            a, b, c, d = 2, 1, 1, 1
            e = 2 * x_val + 1 * y_val
            f = 1 * x_val + 1 * y_val
            question = f"\\text{{Solve the system using matrices: }} {a}x + {b}y = {e}, \\; {c}x + {d}y = {f}"
            answer = Matrix([x_val, y_val])
            elo = 1450
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "precalculus",
        "subtopic": "matrices",
        "grading_mode": "equivalent"
    }

problem = generate_matrix_problem()
print(json.dumps(problem))