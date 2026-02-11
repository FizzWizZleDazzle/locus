"""
linear_algebra - eigenvalues_eigenvectors (medium)
Generated: 2026-02-11T22:12:04.351960
"""

import sympy as sp
import random
import json

def generate_problem():
    random.seed()
    problem_type = random.choice(['2x2_eigenvalues', '2x2_eigenvectors', '2x2_both', 'triangular_2x2', 'diagonal_2x2'])
    
    if problem_type == 'diagonal_2x2':
        lambda1 = random.randint(-5, 5)
        lambda2 = random.randint(-5, 5)
        while lambda2 == lambda1:
            lambda2 = random.randint(-5, 5)
        
        matrix = sp.Matrix([[lambda1, 0], [0, lambda2]])
        
        question = f"Find the eigenvalues of the matrix $A = {sp.latex(matrix)}$."
        answer = sp.FiniteSet(lambda1, lambda2)
        difficulty = 1250
        
    elif problem_type == 'triangular_2x2':
        lambda1 = random.randint(-4, 4)
        lambda2 = random.randint(-4, 4)
        while lambda2 == lambda1:
            lambda2 = random.randint(-4, 4)
        
        off_diag = random.randint(-3, 3)
        if random.choice([True, False]):
            matrix = sp.Matrix([[lambda1, off_diag], [0, lambda2]])
        else:
            matrix = sp.Matrix([[lambda1, 0], [off_diag, lambda2]])
        
        question = f"Find the eigenvalues of the matrix $A = {sp.latex(matrix)}$."
        answer = sp.FiniteSet(lambda1, lambda2)
        difficulty = 1300
        
    elif problem_type == '2x2_eigenvalues':
        lambda1 = random.randint(-3, 3)
        lambda2 = random.randint(-3, 3)
        while lambda2 == lambda1:
            lambda2 = random.randint(-3, 3)
        
        a = random.randint(-2, 2)
        b = random.randint(1, 3)
        c = random.randint(1, 3)
        d = random.randint(-2, 2)
        
        trace = lambda1 + lambda2
        det = lambda1 * lambda2
        
        mat_a = trace - d
        mat_d = d
        mat_bc = det - mat_a * mat_d
        
        if mat_bc > 0 and b > 0:
            mat_b = b
            mat_c = mat_bc // b
            if mat_b * mat_c == mat_bc:
                matrix = sp.Matrix([[mat_a, mat_b], [mat_c, mat_d]])
                question = f"Find the eigenvalues of the matrix $A = {sp.latex(matrix)}$."
                answer = sp.FiniteSet(lambda1, lambda2)
                difficulty = 1400
            else:
                matrix = sp.Matrix([[2, 1], [1, 2]])
                question = f"Find the eigenvalues of the matrix $A = {sp.latex(matrix)}$."
                answer = sp.FiniteSet(1, 3)
                difficulty = 1400
        else:
            matrix = sp.Matrix([[2, 1], [1, 2]])
            question = f"Find the eigenvalues of the matrix $A = {sp.latex(matrix)}$."
            answer = sp.FiniteSet(1, 3)
            difficulty = 1400
            
    elif problem_type == '2x2_eigenvectors':
        lambda_val = random.randint(-3, 3)
        
        v1 = random.randint(1, 3)
        v2 = random.randint(-3, 3)
        while v2 == 0:
            v2 = random.randint(-3, 3)
        
        other_lambda = random.randint(-3, 3)
        while other_lambda == lambda_val:
            other_lambda = random.randint(-3, 3)
        
        a = random.randint(-2, 2)
        matrix = sp.Matrix([[lambda_val, 0], [0, other_lambda]])
        
        question = f"Find an eigenvector corresponding to eigenvalue $\\lambda = {lambda_val}$ for the matrix $A = {sp.latex(matrix)}$."
        answer = sp.Matrix([[1], [0]])
        difficulty = 1450
        
    else:
        lambda1 = random.randint(1, 4)
        lambda2 = random.randint(-3, 0)
        
        matrix = sp.Matrix([[lambda1, 0], [0, lambda2]])
        
        question = f"For the matrix $A = {sp.latex(matrix)}$, find all eigenvalues and an eigenvector for the largest eigenvalue."
        answer_dict = {
            'eigenvalues': sp.FiniteSet(lambda1, lambda2),
            'eigenvector': sp.Matrix([[1], [0]])
        }
        answer = str(answer_dict)
        difficulty = 1550
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "linear_algebra",
        "subtopic": "eigenvalues_eigenvectors",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))