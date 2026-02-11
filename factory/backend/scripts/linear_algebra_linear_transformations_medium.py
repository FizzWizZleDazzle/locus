"""
linear_algebra - linear_transformations (medium)
Generated: 2026-02-11T22:13:08.658302
"""

import random
import json
from sympy import Matrix, symbols, latex, simplify, eye, zeros, sqrt, Rational

def generate_problem():
    problem_type = random.choice([
        'matrix_of_transformation',
        'image_of_vector',
        'kernel_basis',
        'composition',
        'inverse_transform'
    ])
    
    if problem_type == 'matrix_of_transformation':
        # Find matrix representation given transformation rule
        x, y = symbols('x y')
        
        # Pick a clean answer matrix first
        a, b, c, d = random.choice([
            (2, 0, 0, 3),
            (1, 1, 0, 1),
            (2, 1, 1, 2),
            (3, 0, 0, 2),
            (1, 2, 2, 1),
            (0, 1, 1, 0)
        ])
        
        answer_matrix = Matrix([[a, b], [c, d]])
        
        # Create transformation description
        transform_x = a*x + b*y
        transform_y = c*x + d*y
        
        question = f"Find the matrix representation of the linear transformation $T: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ defined by $T\\begin{{pmatrix}} x \\\\ y \\end{{pmatrix}} = \\begin{{pmatrix}} {latex(transform_x)} \\\\ {latex(transform_y)} \\end{{pmatrix}}$."
        
        answer_key = str(answer_matrix.tolist())
        difficulty = 1350
        
    elif problem_type == 'image_of_vector':
        # Apply transformation to a vector
        
        # Pick clean transformation matrix
        matrices = [
            Matrix([[2, 0], [0, 3]]),
            Matrix([[1, 1], [0, 1]]),
            Matrix([[2, 1], [1, 2]]),
            Matrix([[3, 0], [0, 2]]),
            Matrix([[0, -1], [1, 0]])
        ]
        
        T = random.choice(matrices)
        
        # Pick simple input vector
        v = Matrix([[random.randint(1, 5)], [random.randint(1, 5)]])
        
        # Calculate answer
        answer = T * v
        
        question = f"Let $T: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ be the linear transformation with matrix $A = {latex(T)}$. Find $T\\left({latex(v)}\\right)$."
        
        answer_key = str(answer.tolist())
        difficulty = 1300
        
    elif problem_type == 'kernel_basis':
        # Find kernel of a transformation
        
        # Create matrix with known kernel
        rank_choice = random.choice([1])  # rank 1 for non-trivial kernel in 2D
        
        if rank_choice == 1:
            # Matrix with 1D kernel
            a = random.randint(1, 3)
            b = random.randint(1, 3)
            T = Matrix([[a, b], [2*a, 2*b]])
            
            # Kernel basis: solve Tx = 0
            # For this matrix: ax + by = 0, so kernel is span of (-b, a)
            kernel_basis = Matrix([[-b], [a]])
            
            question = f"Find a basis for the kernel of the linear transformation $T: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ with matrix $A = {latex(T)}$."
            
            answer_key = str(kernel_basis.tolist())
            difficulty = 1550
        
    elif problem_type == 'composition':
        # Composition of two transformations
        
        # Pick two simple matrices
        S = Matrix([[2, 0], [0, 1]])
        T = Matrix([[1, random.randint(1, 2)], [0, 1]])
        
        # Answer is product
        answer = S * T
        
        question = f"Let $S: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ and $T: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ be linear transformations with matrices $A_S = {latex(S)}$ and $A_T = {latex(T)}$ respectively. Find the matrix of the composition $S \\circ T$."
        
        answer_key = str(answer.tolist())
        difficulty = 1450
        
    else:  # inverse_transform
        # Find inverse transformation
        
        # Pick invertible matrix with clean inverse
        matrices_with_inverses = [
            Matrix([[2, 0], [0, 3]]),
            Matrix([[1, 1], [0, 1]]),
            Matrix([[2, 1], [1, 1]]),
            Matrix([[3, 1], [2, 1]])
        ]
        
        T = random.choice(matrices_with_inverses)
        answer = T.inv()
        
        question = f"Find the matrix of the inverse transformation $T^{{-1}}$ if $T: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ has matrix $A = {latex(T)}$."
        
        answer_key = str(answer.tolist())
        difficulty = 1500
    
    return {
        "question_latex": question,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "linear_algebra",
        "subtopic": "linear_transformations",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))