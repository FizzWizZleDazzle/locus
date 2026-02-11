"""
linear_algebra - linear_transformations (easy)
Generated: 2026-02-11T22:12:47.649147
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'matrix_multiplication_standard_basis',
        'find_image_of_vector',
        'matrix_from_transformation_description',
        'identify_transformation_type'
    ])
    
    if problem_type == 'matrix_multiplication_standard_basis':
        # ELO ~1050-1150: Direct application - multiply matrix by standard basis vector
        basis_idx = random.randint(0, 1)
        
        # Pick clean answer first
        answer_vals = [random.randint(-3, 3), random.randint(-3, 3)]
        
        # Construct matrix backward
        if basis_idx == 0:
            # e1 = [1, 0]
            col1 = answer_vals
            col2 = [random.randint(-3, 3), random.randint(-3, 3)]
        else:
            # e2 = [0, 1]
            col1 = [random.randint(-3, 3), random.randint(-3, 3)]
            col2 = answer_vals
        
        matrix = sp.Matrix([[col1[0], col2[0]], [col1[1], col2[1]]])
        basis_vector = sp.Matrix([1, 0] if basis_idx == 0 else [0, 1])
        
        question = f"Let $T: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ be the linear transformation with matrix $A = {sp.latex(matrix)}$. Find $T({sp.latex(basis_vector)})$."
        
        answer = sp.Matrix(answer_vals)
        elo = random.randint(1050, 1150)
        
    elif problem_type == 'find_image_of_vector':
        # ELO ~1150-1300: Two-step - multiply matrix by simple vector
        
        # Pick clean answer first
        answer_vals = [random.randint(-5, 5), random.randint(-5, 5)]
        
        # Pick simple input vector
        input_vec = [random.randint(1, 2), random.randint(1, 2)]
        
        # Construct matrix backward: A * v = answer
        # For 2x2: [[a,b],[c,d]] * [x,y] = [ax+by, cx+dy]
        # We need: a*x + b*y = answer[0], c*x + d*y = answer[1]
        
        a = random.randint(-2, 2)
        b_num = answer_vals[0] - a * input_vec[0]
        if input_vec[1] != 0:
            b = b_num // input_vec[1] if b_num % input_vec[1] == 0 else random.randint(-2, 2)
        else:
            b = random.randint(-2, 2)
        
        c = random.randint(-2, 2)
        d_num = answer_vals[1] - c * input_vec[0]
        if input_vec[1] != 0:
            d = d_num // input_vec[1] if d_num % input_vec[1] == 0 else random.randint(-2, 2)
        else:
            d = random.randint(-2, 2)
        
        matrix = sp.Matrix([[a, b], [c, d]])
        vector = sp.Matrix(input_vec)
        answer = matrix * vector
        
        question = f"Let $T: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ be the linear transformation with matrix $A = {sp.latex(matrix)}$. Find $T({sp.latex(vector)})$."
        
        elo = random.randint(1150, 1300)
        
    elif problem_type == 'matrix_from_transformation_description':
        # ELO ~1100-1250: Find matrix given images of standard basis vectors
        
        # Pick clean column vectors (these are the answers/images)
        col1 = [random.randint(-3, 3), random.randint(-3, 3)]
        col2 = [random.randint(-3, 3), random.randint(-3, 3)]
        
        answer = sp.Matrix([[col1[0], col2[0]], [col1[1], col2[1]]])
        
        v1 = sp.Matrix(col1)
        v2 = sp.Matrix(col2)
        e1 = sp.Matrix([1, 0])
        e2 = sp.Matrix([0, 1])
        
        question = f"Find the matrix of the linear transformation $T: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ such that $T({sp.latex(e1)}) = {sp.latex(v1)}$ and $T({sp.latex(e2)}) = {sp.latex(v2)}$."
        
        elo = random.randint(1100, 1250)
        
    else:  # identify_transformation_type
        # ELO ~1000-1150: Identify what simple transformation does (very basic)
        
        transform_types = [
            ('reflection_x', sp.Matrix([[1, 0], [0, -1]]), "reflection across the $x$-axis"),
            ('reflection_y', sp.Matrix([[-1, 0], [0, 1]]), "reflection across the $y$-axis"),
            ('scaling', sp.Matrix([[2, 0], [0, 2]]), "scaling by factor $2$"),
        ]
        
        t_type, matrix, description = random.choice(transform_types)
        test_vector = sp.Matrix([random.randint(1, 3), random.randint(1, 3)])
        answer = matrix * test_vector
        
        question = f"The linear transformation $T: \\mathbb{{R}}^2 \\to \\mathbb{{R}}^2$ represents {description}. Find $T({sp.latex(test_vector)})$."
        
        elo = random.randint(1000, 1150)
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "linear_algebra",
        "subtopic": "linear_transformations",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))