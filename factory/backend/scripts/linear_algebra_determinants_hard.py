"""
linear_algebra - determinants (hard)
Generated: 2026-02-11T22:10:23.812816
"""

import sympy as sp
import random
import json

def generate_determinant_problem():
    problem_type = random.choice([
        'symbolic_3x3',
        'parametric_singular',
        'cofactor_expansion',
        'block_matrix',
        'properties_application'
    ])
    
    if problem_type == 'symbolic_3x3':
        x = sp.Symbol('x')
        target_det = random.choice([0, x**2 - 4, x**3 - x, (x-1)*(x-2)*(x-3)])
        
        if target_det == 0:
            base_matrix = sp.Matrix([
                [1, 2, 3],
                [2, 4, 6],
                [1, 2, 3]
            ])
            a, b = random.randint(1, 3), random.randint(1, 3)
            matrix = sp.Matrix([
                [a, 2*a, 3*a],
                [b, 2*b, 3*b],
                [1, 2, 3]
            ])
        else:
            row1 = [1, 0, x]
            row2 = [x, 1, 0]
            row3 = [0, x, 1]
            matrix = sp.Matrix([row1, row2, row3])
        
        det = matrix.det().simplify()
        
        question = r"\text{Find the determinant: } \det\left(" + sp.latex(matrix) + r"\right)"
        answer = str(det)
        difficulty = 1650
        
    elif problem_type == 'parametric_singular':
        x = sp.Symbol('x')
        target_values = [random.randint(-3, 3) for _ in range(random.randint(1, 2))]
        
        det_polynomial = sp.prod([x - val for val in target_values])
        
        a, b, c = random.randint(1, 4), random.randint(1, 4), random.randint(1, 4)
        matrix = sp.Matrix([
            [x, a, b],
            [c, x, a],
            [b, c, x]
        ])
        
        det = matrix.det().simplify()
        
        question = r"\text{For what value(s) of } x \text{ is the matrix singular? } " + sp.latex(matrix)
        answer = str(sp.solveset(det, x, domain=sp.S.Reals))
        difficulty = 1700
        
    elif problem_type == 'cofactor_expansion':
        a, b, c, d = sp.symbols('a b c d')
        
        k1, k2, k3 = random.randint(1, 3), random.randint(1, 3), random.randint(1, 3)
        
        matrix = sp.Matrix([
            [a, b, k1, 0],
            [c, d, 0, k2],
            [0, k3, a, b],
            [k2, 0, c, d]
        ])
        
        det = matrix.det().simplify()
        
        question = r"\text{Compute the determinant: } \det\left(" + sp.latex(matrix) + r"\right)"
        answer = str(det)
        difficulty = 1750
        
    elif problem_type == 'block_matrix':
        x = sp.Symbol('x')
        
        n = random.randint(2, 4)
        m = random.randint(2, 4)
        
        A_det = random.randint(2, 5)
        B_det = random.randint(2, 5)
        
        question = r"\text{Let } A \text{ be a } " + str(n) + r" \times " + str(n) + r" \text{ matrix with } \det(A) = " + str(A_det)
        question += r"\text{ and } B \text{ be a } " + str(m) + r" \times " + str(m) + r" \text{ matrix with } \det(B) = " + str(B_det) + r"."
        question += r"\text{ Find } \det\left(\begin{bmatrix} A & 0 \\ 0 & B \end{bmatrix}\right)"
        
        answer = str(A_det * B_det)
        difficulty = 1650
        
    else:  # properties_application
        x = sp.Symbol('x')
        
        base_det = random.randint(2, 6)
        k = random.randint(2, 4)
        n = random.randint(2, 4)
        
        operations = random.choice([
            ('transpose', base_det, 1600),
            ('scalar_multiply', base_det * (k**n), 1700),
            ('inverse', sp.Rational(1, base_det), 1650),
            ('power', base_det**k, 1750)
        ])
        
        op_type, result, diff = operations
        
        if op_type == 'transpose':
            question = r"\text{If } \det(A) = " + str(base_det) + r"\text{, find } \det(A^T)"
        elif op_type == 'scalar_multiply':
            question = r"\text{If } A \text{ is a } " + str(n) + r" \times " + str(n) + r" \text{ matrix with } \det(A) = " + str(base_det)
            question += r"\text{, find } \det(" + str(k) + r"A)"
        elif op_type == 'inverse':
            question = r"\text{If } \det(A) = " + str(base_det) + r"\text{, find } \det(A^{-1})"
        else:  # power
            question = r"\text{If } \det(A) = " + str(base_det) + r"\text{, find } \det(A^{" + str(k) + r"})"
        
        answer = str(result)
        difficulty = diff
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "linear_algebra",
        "subtopic": "determinants",
        "grading_mode": "equivalent"
    }

problem = generate_determinant_problem()
print(json.dumps(problem))