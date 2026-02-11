"""
linear_algebra - linear_transformations (hard)
Generated: 2026-02-11T22:13:34.632873
"""

import sympy as sp
import random
import json

def generate_linear_transformation_problem():
    random.seed()
    problem_type = random.choice([
        'kernel_dimension',
        'image_basis',
        'inverse_transformation',
        'composition_properties',
        'eigenspace_transformation',
        'matrix_representation_nonstandard'
    ])
    
    if problem_type == 'kernel_dimension':
        # 1600-1700: Find dimension of kernel given transformation
        m = random.randint(3, 4)
        n = random.randint(3, 4)
        rank = random.randint(2, min(m, n) - 1)
        
        A = sp.Matrix([[random.randint(-2, 2) for _ in range(n)] for _ in range(m)])
        # Adjust to get desired rank by making some rows dependent
        while sp.Matrix(A).rank() != rank:
            A = sp.Matrix([[random.randint(-2, 2) for _ in range(n)] for _ in range(m)])
        
        nullity = n - rank
        
        question = f"Let $T: \\mathbb{{R}}^{{{n}}} \\to \\mathbb{{R}}^{{{m}}}$ be the linear transformation defined by the matrix $A = {sp.latex(A)}$. Find $\\dim(\\ker(T))$."
        answer = str(nullity)
        difficulty = 1650
        
    elif problem_type == 'image_basis':
        # 1700-1800: Find basis for image
        n = 3
        m = 3
        # Create matrix with known column space
        basis_vecs = [[1, 0, 1], [0, 1, 1]]
        col3 = [basis_vecs[0][i] + 2*basis_vecs[1][i] for i in range(3)]
        A = sp.Matrix([basis_vecs[0], basis_vecs[1], col3]).T
        
        question = f"Let $T: \\mathbb{{R}}^{{{n}}} \\to \\mathbb{{R}}^{{{m}}}$ be defined by $T(\\mathbf{{x}}) = A\\mathbf{{x}}$ where $A = {sp.latex(A)}$. Find the dimension of the image of $T$."
        answer = "2"
        difficulty = 1750
        
    elif problem_type == 'inverse_transformation':
        # 1650-1750: Determine if transformation is invertible and find inverse
        # Start with invertible matrix
        while True:
            A = sp.Matrix([[random.randint(-2, 3) for _ in range(3)] for _ in range(3)])
            if A.det() != 0:
                break
        
        det_val = A.det()
        
        question = f"Let $T: \\mathbb{{R}}^3 \\to \\mathbb{{R}}^3$ be the linear transformation with matrix $A = {sp.latex(A)}$. Is $T$ invertible? If yes, compute $\\det(A^{{-1}})$. If no, enter 0."
        answer = str(sp.Rational(1, det_val))
        difficulty = 1680
        
    elif problem_type == 'composition_properties':
        # 1700-1800: Properties of composition
        A = sp.Matrix([[2, 1], [0, 3]])
        B = sp.Matrix([[1, -1], [2, 1]])
        
        composition = B * A
        trace_val = composition.trace()
        
        question = f"Let $S: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ be given by $S(\\mathbf{{x}}) = {sp.latex(A)}\\mathbf{{x}}$ and $T: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ be given by $T(\\mathbf{{x}}) = {sp.latex(B)}\\mathbf{{x}}$. Find $\\text{{tr}}(T \\circ S)$."
        answer = str(trace_val)
        difficulty = 1720
        
    elif problem_type == 'eigenspace_transformation':
        # 1750-1850: Transformation restricted to eigenspace
        # Matrix with known eigenvalues
        lambda1 = random.randint(2, 4)
        lambda2 = random.randint(-2, -1)
        A = sp.Matrix([[lambda1, 0, 0], [0, lambda2, 0], [1, 0, lambda1]])
        
        # Find dimension of eigenspace for lambda1
        eigenspace_matrix = A - lambda1 * sp.eye(3)
        nullity = 3 - eigenspace_matrix.rank()
        
        question = f"Consider the linear transformation $T: \\mathbb{{R}}^3 \\to \\mathbb{{R}}^3$ with matrix $A = {sp.latex(A)}$. Find the dimension of the eigenspace corresponding to eigenvalue $\\lambda = {lambda1}$."
        answer = str(nullity)
        difficulty = 1800
        
    else:  # matrix_representation_nonstandard
        # 1800-1900: Find matrix in non-standard basis
        # T maps polynomial space, find matrix representation
        v1 = sp.Symbol('a')
        v2 = sp.Symbol('b')
        
        question = f"Let $T: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ be defined by $T\\begin{{pmatrix}} x \\\\ y \\end{{pmatrix}} = \\begin{{pmatrix}} 2x + y \\\\ x - y \\end{{pmatrix}}$. With respect to the basis $\\mathcal{{B}} = \\{{\\begin{{pmatrix}} 1 \\\\ 1 \\end{{pmatrix}}, \\begin{{pmatrix}} 1 \\\\ -1 \\end{{pmatrix}}\\}}$, find the trace of $[T]_\\mathcal{{B}}$."
        
        # T applied to basis vectors
        # T([1,1]) = [3, 0] = 3/2[1,1] + 3/2[1,-1]
        # T([1,-1]) = [1, 2] = 3/2[1,1] - 1/2[1,-1]
        # Matrix is [[3/2, 3/2], [3/2, -1/2]]
        # Trace is 1
        answer = "1"
        difficulty = 1850
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "linear_algebra",
        "subtopic": "linear_transformations",
        "grading_mode": "equivalent"
    }

problem = generate_linear_transformation_problem()
print(json.dumps(problem))