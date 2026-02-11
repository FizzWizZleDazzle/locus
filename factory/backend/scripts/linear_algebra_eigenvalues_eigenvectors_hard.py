"""
linear_algebra - eigenvalues_eigenvectors (hard)
Generated: 2026-02-11T22:12:25.989924
"""

import sympy as sp
import random
import json

def generate_problem():
    random.seed()
    
    problem_type = random.choice([
        'characteristic_polynomial_complex',
        'diagonalization_with_defect',
        'generalized_eigenvector',
        'similar_matrix_eigenvalues',
        'eigenspace_dimension'
    ])
    
    if problem_type == 'characteristic_polynomial_complex':
        # Complex eigenvalues, find eigenvectors
        # Pick eigenvalues: a ± bi
        a = random.randint(-2, 2)
        b = random.choice([1, 2])
        
        # Create 2x2 matrix with eigenvalues a+bi, a-bi
        # Use [[a, -b], [b, a]] rotation-scaling form
        matrix = sp.Matrix([[a, -b], [b, a]])
        
        question = r"\text{Find the eigenvalues and one eigenvector for the eigenvalue with positive imaginary part for } A = " + sp.latex(matrix)
        
        eigenval = a + b*sp.I
        # Eigenvector for a+bi: solve (A - (a+bi)I)v = 0
        # This gives v = [1, i] typically
        eigenvec = sp.Matrix([1, sp.I])
        
        answer = f"eigenvalue: {eigenval}, eigenvector: {eigenvec}"
        difficulty = 1650
        
    elif problem_type == 'diagonalization_with_defect':
        # Check if matrix is diagonalizable when it has repeated eigenvalue
        lam = random.randint(1, 4)
        
        # Create non-diagonalizable matrix (Jordan block)
        matrix = sp.Matrix([[lam, 1], [0, lam]])
        
        question = r"\text{Determine if the matrix } A = " + sp.latex(matrix) + r"\text{ is diagonalizable. If yes, find } P \text{ and } D \text{ such that } A = PDP^{-1}\text{. If no, state 'not diagonalizable'.}"
        
        answer = "not diagonalizable"
        difficulty = 1700
        
    elif problem_type == 'generalized_eigenvector':
        # Find eigenvalues and algebraic vs geometric multiplicity
        lam = random.randint(2, 4)
        a = random.randint(1, 3)
        
        # Matrix with repeated eigenvalue but defective
        matrix = sp.Matrix([[lam, a], [0, lam]])
        
        question = r"\text{For matrix } A = " + sp.latex(matrix) + r"\text{, find all eigenvalues and determine the geometric multiplicity of each.}"
        
        answer = f"eigenvalue: {lam}, algebraic multiplicity: 2, geometric multiplicity: 1"
        difficulty = 1750
        
    elif problem_type == 'similar_matrix_eigenvalues':
        # Given characteristic polynomial, find matrix satisfying conditions
        # Pick eigenvalues
        lam1 = random.randint(1, 3)
        lam2 = random.randint(-2, 0)
        
        # Create diagonal matrix first
        D = sp.Matrix([[lam1, 0], [0, lam2]])
        
        # Create random invertible matrix P
        while True:
            p11, p12 = random.randint(1, 3), random.randint(-2, 2)
            p21, p22 = random.randint(-2, 2), random.randint(1, 3)
            P = sp.Matrix([[p11, p12], [p21, p22]])
            if P.det() != 0:
                break
        
        # A = PDP^(-1)
        A = P * D * P.inv()
        A = sp.simplify(A)
        
        question = r"\text{Find all eigenvalues of } A = " + sp.latex(A)
        
        eigenvals = sorted([lam1, lam2], reverse=True)
        answer = f"{eigenvals[0]}, {eigenvals[1]}"
        difficulty = 1650
        
    else:  # eigenspace_dimension
        # 3x3 matrix, find eigenspace dimensions
        lam = random.randint(1, 3)
        
        # Create matrix with eigenvalue lam of multiplicity 2, geometric multiplicity 2
        # Use block structure
        a = random.randint(1, 2)
        b = random.randint(1, 2)
        c = random.randint(-2, -1)
        
        matrix = sp.Matrix([
            [lam, 0, 0],
            [0, lam, 0],
            [0, 0, c]
        ])
        
        question = r"\text{For matrix } A = " + sp.latex(matrix) + r"\text{, find all eigenvalues and the dimension of each eigenspace.}"
        
        answer = f"eigenvalue {lam}: dimension 2, eigenvalue {c}: dimension 1"
        difficulty = 1600
    
    result = {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "linear_algebra",
        "subtopic": "eigenvalues_eigenvectors",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_problem()
print(json.dumps(problem))