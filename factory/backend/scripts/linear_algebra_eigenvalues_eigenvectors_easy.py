"""
linear_algebra - eigenvalues_eigenvectors (easy)
Generated: 2026-02-11T22:11:46.919586
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'eigenvalue_2x2_diagonal',
        'eigenvalue_2x2_triangular', 
        'eigenvector_2x2_simple',
        'eigenvalue_direct_check'
    ])
    
    if problem_type == 'eigenvalue_2x2_diagonal':
        # Diagonal matrix - easiest case
        lam1 = random.randint(-5, 5)
        lam2 = random.randint(-5, 5)
        while lam2 == lam1:
            lam2 = random.randint(-5, 5)
        
        matrix_latex = r"\begin{pmatrix} " + str(lam1) + r" & 0 \\ 0 & " + str(lam2) + r" \end{pmatrix}"
        question_latex = r"\text{Find the eigenvalues of } A = " + matrix_latex
        
        eigenvalues = sorted([lam1, lam2])
        answer_key = str(eigenvalues)
        difficulty = 1050
        
    elif problem_type == 'eigenvalue_2x2_triangular':
        # Upper triangular matrix
        lam1 = random.randint(-4, 4)
        lam2 = random.randint(-4, 4)
        while lam2 == lam1:
            lam2 = random.randint(-4, 4)
        off_diag = random.randint(1, 5)
        
        matrix_latex = r"\begin{pmatrix} " + str(lam1) + r" & " + str(off_diag) + r" \\ 0 & " + str(lam2) + r" \end{pmatrix}"
        question_latex = r"\text{Find the eigenvalues of } A = " + matrix_latex
        
        eigenvalues = sorted([lam1, lam2])
        answer_key = str(eigenvalues)
        difficulty = 1100
        
    elif problem_type == 'eigenvector_2x2_simple':
        # Given eigenvalue, find eigenvector for simple matrix
        lam = random.randint(1, 5)
        
        # Construct matrix where [1, 1] is eigenvector with eigenvalue lam
        # A * [1, 1] = lam * [1, 1]
        # [a, b; c, d] * [1; 1] = [lam; lam]
        # So a + b = lam and c + d = lam
        a = random.randint(0, lam)
        b = lam - a
        c = random.randint(0, lam)
        d = lam - c
        
        matrix_latex = r"\begin{pmatrix} " + str(a) + r" & " + str(b) + r" \\ " + str(c) + r" & " + str(d) + r" \end{pmatrix}"
        question_latex = r"\text{Find an eigenvector of } A = " + matrix_latex + r"\text{ corresponding to eigenvalue } \lambda = " + str(lam)
        
        answer_key = "[1, 1]"
        difficulty = 1250
        
    else:  # eigenvalue_direct_check
        # Verify if a value is an eigenvalue
        lam_true = random.randint(2, 6)
        
        # Create 2x2 matrix with eigenvalue lam_true
        # Use form [[lam_true, 0], [0, other]]
        other = random.randint(-3, 3)
        while other == lam_true:
            other = random.randint(-3, 3)
        
        matrix_latex = r"\begin{pmatrix} " + str(lam_true) + r" & 0 \\ 0 & " + str(other) + r" \end{pmatrix}"
        
        # Ask if lam_true is an eigenvalue
        question_latex = r"\text{Is } \lambda = " + str(lam_true) + r"\text{ an eigenvalue of } A = " + matrix_latex + r"? \text{ Answer 'yes' or 'no'.}"
        
        answer_key = "yes"
        difficulty = 1150
    
    return {
        "question_latex": question_latex,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "linear_algebra",
        "subtopic": "eigenvalues_eigenvectors",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))