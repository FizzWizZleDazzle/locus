"""
linear_algebra - vector_spaces (hard)
Generated: 2026-02-11T22:11:31.090069
"""

import sympy as sp
import random
import json

def generate_vector_space_problem():
    random.seed()
    problem_type = random.choice([
        'basis_dimension',
        'linear_independence',
        'span_membership',
        'subspace_verification',
        'coordinate_vector'
    ])
    
    if problem_type == 'basis_dimension':
        # Pick dimension first
        dim = random.randint(2, 3)
        # Create a subspace defined by linear constraints
        
        if random.choice([True, False]):
            # Null space of a matrix
            rows = random.randint(1, 2)
            cols = dim + 1
            
            # Build matrix with known rank
            rank = rows
            A = sp.Matrix([[random.randint(-3, 3) for _ in range(cols)] for _ in range(rows)])
            
            # Ensure non-trivial null space
            while A.rank() >= cols - 1:
                A = sp.Matrix([[random.randint(-3, 3) for _ in range(cols)] for _ in range(rows)])
            
            null_dim = cols - A.rank()
            
            question = f"Find the dimension of the null space of the matrix $A = {sp.latex(A)}$."
            answer = str(null_dim)
            difficulty = 1650
        else:
            # Span of vectors
            num_vecs = random.randint(3, 4)
            vec_dim = 3
            
            # Create vectors with known rank
            target_rank = random.randint(2, 3)
            
            vecs = []
            M = sp.Matrix.zeros(vec_dim, 0)
            
            for i in range(num_vecs):
                if i < target_rank:
                    v = sp.Matrix([random.randint(-4, 4) for _ in range(vec_dim)])
                    while M.rank() == M.shape[1] and (M.row_join(v)).rank() == M.rank():
                        v = sp.Matrix([random.randint(-4, 4) for _ in range(vec_dim)])
                    M = M.row_join(v)
                else:
                    # Linear combination of previous
                    coeffs = [random.randint(-2, 2) for _ in range(target_rank)]
                    v = sum(coeffs[j] * M[:, j] for j in range(target_rank))
                    M = M.row_join(v)
                vecs.append(v)
            
            vec_strs = [f"\\mathbf{{v}}_{i+1} = {sp.latex(vecs[i])}" for i in range(num_vecs)]
            
            question = f"Find the dimension of the subspace $W$ spanned by the vectors {', '.join(vec_strs)}."
            answer = str(target_rank)
            difficulty = 1680
    
    elif problem_type == 'linear_independence':
        vec_dim = 3
        num_vecs = random.randint(3, 4)
        
        # Choose answer first
        is_independent = random.choice([True, False])
        
        if is_independent and num_vecs <= vec_dim:
            # Create independent vectors
            vecs = []
            M = sp.Matrix.zeros(vec_dim, 0)
            
            for i in range(num_vecs):
                v = sp.Matrix([random.randint(-4, 4) for _ in range(vec_dim)])
                while M.shape[1] > 0 and (M.row_join(v)).rank() <= M.rank():
                    v = sp.Matrix([random.randint(-4, 4) for _ in range(vec_dim)])
                M = M.row_join(v)
                vecs.append(v)
            
            answer_val = "independent"
        else:
            # Create dependent vectors
            base_vecs = random.randint(2, min(3, num_vecs - 1))
            vecs = []
            M = sp.Matrix.zeros(vec_dim, 0)
            
            for i in range(base_vecs):
                v = sp.Matrix([random.randint(-4, 4) for _ in range(vec_dim)])
                while M.shape[1] > 0 and (M.row_join(v)).rank() <= M.rank():
                    v = sp.Matrix([random.randint(-4, 4) for _ in range(vec_dim)])
                M = M.row_join(v)
                vecs.append(v)
            
            # Add dependent vectors
            for i in range(num_vecs - base_vecs):
                coeffs = [random.randint(-2, 2) for _ in range(base_vecs)]
                v = sum(coeffs[j] * vecs[j] for j in range(base_vecs))
                vecs.append(v)
            
            random.shuffle(vecs)
            answer_val = "dependent"
        
        vec_strs = [f"\\mathbf{{v}}_{i+1} = {sp.latex(vecs[i])}" for i in range(len(vecs))]
        question = f"Determine whether the vectors {', '.join(vec_strs)} are linearly independent or dependent."
        answer = answer_val
        difficulty = 1620
    
    elif problem_type == 'span_membership':
        vec_dim = 3
        num_basis = random.randint(2, 3)
        
        # Create basis vectors
        basis_vecs = []
        M = sp.Matrix.zeros(vec_dim, 0)
        
        for i in range(num_basis):
            v = sp.Matrix([random.randint(-3, 3) for _ in range(vec_dim)])
            while M.shape[1] > 0 and (M.row_join(v)).rank() <= M.rank():
                v = sp.Matrix([random.randint(-3, 3) for _ in range(vec_dim)])
            M = M.row_join(v)
            basis_vecs.append(v)
        
        # Decide if target is in span
        in_span = random.choice([True, False])
        
        if in_span:
            # Create linear combination
            coeffs = [random.randint(-3, 3) for _ in range(num_basis)]
            target = sum(coeffs[i] * basis_vecs[i] for i in range(num_basis))
            answer_val = "yes"
        else:
            # Create vector outside span
            target = sp.Matrix([random.randint(-5, 5) for _ in range(vec_dim)])
            augmented = M.row_join(target)
            while augmented.rank() == M.rank():
                target = sp.Matrix([random.randint(-5, 5) for _ in range(vec_dim)])
                augmented = M.row_join(target)
            answer_val = "no"
        
        basis_strs = [f"\\mathbf{{v}}_{i+1} = {sp.latex(basis_vecs[i])}" for i in range(num_basis)]
        question = f"Is the vector $\\mathbf{{w}} = {sp.latex(target)}$ in the span of {', '.join(basis_strs)}? Answer 'yes' or 'no'."
        answer = answer_val
        difficulty = 1700
    
    elif problem_type == 'subspace_verification':
        # Check if a set is a subspace
        vec_dim = 3
        
        subspace_type = random.choice(['plane_through_origin', 'not_closed_scalar', 'not_containing_zero'])
        
        if subspace_type == 'plane_through_origin':
            # Define plane ax + by + cz = 0
            a, b, c = [random.randint(-3, 3) for _ in range(3)]
            while a ==