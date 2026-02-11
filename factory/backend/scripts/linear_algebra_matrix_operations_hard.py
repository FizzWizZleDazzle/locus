"""
linear_algebra - matrix_operations (hard)
Generated: 2026-02-11T22:09:24.552694
"""

import sympy as sp
import random
import json

def generate_hard_matrix_problem():
    problem_type = random.choice([
        'determinant_with_parameter',
        'inverse_with_calculation',
        'matrix_equation_solve',
        'eigenvalue_problem',
        'power_of_matrix',
        'rank_nullity',
        'simultaneous_matrix_equations'
    ])
    
    if problem_type == 'determinant_with_parameter':
        x = sp.Symbol('x')
        a, b, c = random.randint(-3, 3), random.randint(-3, 3), random.randint(1, 4)
        d, e, f = random.randint(-3, 3), random.randint(-3, 3), random.randint(1, 4)
        
        M = sp.Matrix([
            [x, a, b],
            [c, x, d],
            [e, f, x]
        ])
        
        det = M.det().expand()
        
        question = f"Find the determinant of the matrix $\\begin{{bmatrix}} x & {a} & {b} \\\\ {c} & x & {d} \\\\ {e} & {f} & x \\end{{bmatrix}}$ as a polynomial in $x$."
        answer = str(det)
        difficulty = random.randint(1650, 1750)
        
    elif problem_type == 'inverse_with_calculation':
        while True:
            a, b = random.randint(-4, 4), random.randint(-4, 4)
            c, d = random.randint(-4, 4), random.randint(-4, 4)
            det_val = a*d - b*c
            if det_val != 0 and abs(det_val) <= 10:
                break
        
        M = sp.Matrix([[a, b], [c, d]])
        inv = M.inv()
        
        e, f = random.randint(-3, 3), random.randint(-3, 3)
        g, h = random.randint(-3, 3), random.randint(-3, 3)
        N = sp.Matrix([[e, f], [g, h]])
        
        result = inv * N
        
        question = f"Let $A = \\begin{{bmatrix}} {a} & {b} \\\\ {c} & {d} \\end{{bmatrix}}$ and $B = \\begin{{bmatrix}} {e} & {f} \\\\ {g} & {h} \\end{{bmatrix}}$. Find $A^{{-1}}B$."
        answer = str(result)
        difficulty = random.randint(1600, 1680)
        
    elif problem_type == 'matrix_equation_solve':
        x, y = sp.symbols('x y')
        a, b = random.randint(1, 4), random.randint(1, 4)
        c, d = random.randint(1, 4), random.randint(1, 4)
        
        target_x = random.randint(-3, 3)
        target_y = random.randint(-3, 3)
        
        result_val1 = a * target_x + b * target_y
        result_val2 = c * target_x + d * target_y
        
        question = f"Solve for $x$ and $y$ if $\\begin{{bmatrix}} {a} & {b} \\\\ {c} & {d} \\end{{bmatrix}} \\begin{{bmatrix}} x \\\\ y \\end{{bmatrix}} = \\begin{{bmatrix}} {result_val1} \\\\ {result_val2} \\end{{bmatrix}}$. Express your answer as an ordered pair $(x, y)$."
        answer = f"({target_x}, {target_y})"
        difficulty = random.randint(1620, 1700)
        
    elif problem_type == 'eigenvalue_problem':
        lambda_val = random.choice([-2, -1, 1, 2, 3])
        trace_val = random.randint(-6, 6)
        other_lambda = trace_val - lambda_val
        
        det_val = lambda_val * other_lambda
        
        a = random.randint(-3, 3)
        d = trace_val - a
        
        while True:
            b = random.randint(-4, 4)
            if b != 0:
                c = (det_val - a*d) / b
                if c == int(c) and abs(c) <= 5:
                    c = int(c)
                    break
        
        question = f"Find the eigenvalues of the matrix $\\begin{{bmatrix}} {a} & {b} \\\\ {c} & {d} \\end{{bmatrix}}$. Express your answer as a set."
        eigenvals = sorted([lambda_val, other_lambda])
        answer = str(set(eigenvals))
        difficulty = random.randint(1700, 1800)
        
    elif problem_type == 'power_of_matrix':
        diag1, diag2 = random.choice([1, 2, -1]), random.choice([1, 2, -1])
        n = random.randint(3, 5)
        
        a, d = random.randint(-2, 2), random.randint(-2, 2)
        b = random.randint(1, 3)
        
        M = sp.Matrix([[diag1, b], [0, diag2]])
        M_power = M ** n
        
        question = f"Compute $\\begin{{bmatrix}} {diag1} & {b} \\\\ 0 & {diag2} \\end{{bmatrix}}^{{{n}}}$."
        answer = str(M_power)
        difficulty = random.randint(1650, 1750)
        
    elif problem_type == 'rank_nullity':
        rows = random.randint(3, 4)
        cols = random.randint(3, 4)
        rank = random.randint(2, min(rows, cols))
        
        if rank == 2:
            v1 = [random.randint(-2, 2) for _ in range(cols)]
            v2 = [random.randint(-2, 2) for _ in range(cols)]
            
            M_rows = [v1, v2]
            for _ in range(rows - 2):
                coef1, coef2 = random.randint(-1, 1), random.randint(-1, 1)
                new_row = [coef1 * v1[i] + coef2 * v2[i] for i in range(cols)]
                M_rows.append(new_row)
        else:
            M_rows = [[random.randint(-2, 2) for _ in range(cols)] for _ in range(rows)]
        
        M = sp.Matrix(M_rows)
        actual_rank = M.rank()
        nullity = cols - actual_rank
        
        M_latex = "\\begin{bmatrix} " + " \\\\ ".join([" & ".join(map(str, row)) for row in M_rows]) + " \\end{bmatrix}"
        
        question = f"Find the rank and nullity of the matrix ${M_latex}$. Express your answer as an ordered pair (rank, nullity)."
        answer = f"({actual_rank}, {nullity})"
        difficulty = random.randint(1750, 1850)
        
    else:  # simultaneous_matrix_equations
        target_a, target_b = random.randint(-2, 2), random.randint(-2, 2)
        target_c, target_d = random.randint(-2, 2), random.randint(-2, 2)