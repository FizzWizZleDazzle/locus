"""
precalculus - matrices (hard)
Generated: 2026-02-11T21:58:45.889775
"""

import sympy as sp
import random
import json

def generate_matrix_problem():
    problem_type = random.choice([
        'determinant_parametric',
        'inverse_with_parameter',
        'matrix_equation_solve',
        'eigenvalue_computation',
        'matrix_power',
        'system_matrix_form'
    ])
    
    if problem_type == 'determinant_parametric':
        # Find x such that det(A) = k, where A contains x
        target_det = random.randint(-5, 5)
        # Reverse engineer: pick x value, construct matrix with that det
        x_val = random.randint(-3, 3)
        
        # 3x3 matrix with parameter x
        a11 = random.randint(1, 3)
        a13 = random.randint(-2, 2)
        a21 = random.randint(-2, 2)
        a22 = random.randint(1, 3)
        a31 = random.randint(-2, 2)
        a32 = random.randint(1, 3)
        a33 = random.randint(1, 3)
        
        # Place x in a12 and a23 positions
        # det = a11*(a22*a33 - x*a32) - x*(a21*a33 - a23*a31) + a13*(a21*a32 - a22*a31)
        # Construct so det has specific value at x_val
        
        x = sp.Symbol('x')
        A = sp.Matrix([
            [a11, x, a13],
            [a21, a22, x],
            [a31, a32, a33]
        ])
        
        det_expr = A.det()
        # Adjust to make solution cleaner
        target_det = det_expr.subs(x, x_val)
        
        question = f"Find all values of $x$ for which $\\det\\begin{{pmatrix}} {a11} & x & {a13} \\\\ {a21} & {a22} & x \\\\ {a31} & {a32} & {a33} \\end{{pmatrix}} = {target_det}$"
        answer = str(x_val)
        difficulty = 1650
        
    elif problem_type == 'inverse_with_parameter':
        # Find (A + kI)^(-1) for specific k
        k = random.randint(1, 3)
        
        # Start with invertible result matrix
        inv_a = random.randint(-2, 2)
        inv_b = random.randint(-2, 2)
        inv_c = random.randint(-2, 2)
        inv_d = random.randint(-2, 2)
        while inv_a * inv_d - inv_b * inv_c == 0:
            inv_d = random.randint(-2, 2)
        
        B_inv = sp.Matrix([[inv_a, inv_b], [inv_c, inv_d]])
        B = B_inv.inv()
        
        # A = B - kI
        A = B - k * sp.eye(2)
        
        question = f"Find $(A + {k}I)^{{-1}}$ where $A = {sp.latex(A)}$"
        answer = sp.latex(B_inv)
        difficulty = 1620
        
    elif problem_type == 'matrix_equation_solve':
        # Solve AXB = C for X (2x2 matrices)
        # Pick X first
        x11, x12 = random.randint(-2, 2), random.randint(-2, 2)
        x21, x22 = random.randint(-2, 2), random.randint(-2, 2)
        X = sp.Matrix([[x11, x12], [x21, x22]])
        
        # Pick invertible A and B
        a11, a12 = random.randint(1, 2), random.randint(-1, 1)
        a21, a22 = random.randint(-1, 1), random.randint(1, 2)
        A = sp.Matrix([[a11, a12], [a21, a22]])
        while A.det() == 0:
            a22 = random.randint(1, 3)
            A = sp.Matrix([[a11, a12], [a21, a22]])
        
        b11, b12 = random.randint(1, 2), random.randint(-1, 1)
        b21, b22 = random.randint(-1, 1), random.randint(1, 2)
        B = sp.Matrix([[b11, b12], [b21, b22]])
        while B.det() == 0:
            b22 = random.randint(1, 3)
            B = sp.Matrix([[b11, b12], [b21, b22]])
        
        C = A * X * B
        
        question = f"Solve for matrix $X$: ${sp.latex(A)} X {sp.latex(B)} = {sp.latex(C)}$"
        answer = sp.latex(X)
        difficulty = 1750
        
    elif problem_type == 'eigenvalue_computation':
        # Find eigenvalues of 2x2 matrix
        # Pick eigenvalues first
        lambda1 = random.randint(-3, 3)
        lambda2 = random.randint(-3, 3)
        while lambda1 == lambda2:
            lambda2 = random.randint(-3, 3)
        
        # Construct matrix with these eigenvalues (diagonal form rotated)
        # Simple form: upper triangular
        a = random.randint(-2, 2)
        A = sp.Matrix([[lambda1, a], [0, lambda2]])
        
        question = f"Find all eigenvalues of ${sp.latex(A)}$"
        answer = f"{{{min(lambda1, lambda2)}, {max(lambda1, lambda2)}}}"
        difficulty = 1680
        
    elif problem_type == 'matrix_power':
        # Compute A^n for diagonal or simple matrix
        n = random.choice([3, 4])
        
        # Diagonalizable 2x2 with integer eigenvalues
        d1 = random.choice([2, -1, 3])
        d2 = random.choice([1, -2, 2])
        
        # Simple case: A = [[a, 0], [0, b]]
        A = sp.Matrix([[d1, 0], [0, d2]])
        A_n = A**n
        
        question = f"Compute ${sp.latex(A)}^{{{n}}}$"
        answer = sp.latex(A_n)
        difficulty = 1620
        
    else:  # system_matrix_form
        # Write system as matrix equation and solve
        # Pick solution first
        x_sol = random.randint(-2, 3)
        y_sol = random.randint(-2, 3)
        z_sol = random.randint(-2, 3)
        
        # Construct system
        a11, a12, a13 = random.randint(1, 2), random.randint(-1, 2), random.randint(-1, 2)
        a21, a22, a23 = random.randint(-1, 2), random.randint(1, 2), random.randint(-1, 2)
        a31, a32, a33 = random.randint(-1, 2), random.randint(-1, 2), random.randint(1, 2)
        
        A = sp.Matrix([[a11, a12, a13], [a21, a22, a23], [a31, a32, a33]])
        while A.det() == 0:
            a33 = random.randint(2, 3)
            A