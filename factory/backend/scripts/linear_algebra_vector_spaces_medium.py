"""
linear_algebra - vector_spaces (medium)
Generated: 2026-02-11T22:11:03.347808
"""

import sympy as sp
import random
import json

def generate_vector_space_problem():
    problem_type = random.choice([
        'linear_combination',
        'span_check',
        'linear_independence',
        'basis_verification',
        'dimension'
    ])
    
    if problem_type == 'linear_combination':
        # Express vector as linear combination
        # ELO: 1300-1400 (two-step: set up system, solve)
        dim = random.choice([2, 3])
        
        if dim == 2:
            # Pick clean coefficients (answer)
            c1, c2 = random.randint(-3, 3), random.randint(-3, 3)
            while c1 == 0 and c2 == 0:
                c1, c2 = random.randint(-3, 3), random.randint(-3, 3)
            
            # Pick basis vectors
            v1 = [random.randint(-3, 3) for _ in range(2)]
            v2 = [random.randint(-3, 3) for _ in range(2)]
            while v1[0] * v2[1] == v1[1] * v2[0]:  # Ensure independence
                v2 = [random.randint(-3, 3) for _ in range(2)]
            
            # Construct target vector
            target = [c1 * v1[0] + c2 * v2[0], c1 * v1[1] + c2 * v2[1]]
            
            question = f"Express \\(\\begin{{pmatrix}} {target[0]} \\\\ {target[1]} \\end{{pmatrix}}\\) as a linear combination of \\(\\begin{{pmatrix}} {v1[0]} \\\\ {v1[1]} \\end{{pmatrix}}\\) and \\(\\begin{{pmatrix}} {v2[0]} \\\\ {v2[1]} \\end{{pmatrix}}\\). Enter your answer as an ordered pair \\((c_1, c_2)\\) where the combination is \\(c_1 v_1 + c_2 v_2\\)."
            
            answer = f"({c1}, {c2})"
            difficulty = 1350
        
        else:  # dim == 3
            c1, c2, c3 = random.randint(-2, 2), random.randint(-2, 2), random.randint(-2, 2)
            while c1 == 0 and c2 == 0 and c3 == 0:
                c1, c2, c3 = random.randint(-2, 2), random.randint(-2, 2), random.randint(-2, 2)
            
            v1 = [1, 0, 0]
            v2 = [0, 1, 0]
            v3 = [0, 0, 1]
            
            target = [c1, c2, c3]
            
            question = f"Express \\(\\begin{{pmatrix}} {target[0]} \\\\ {target[1]} \\\\ {target[2]} \\end{{pmatrix}}\\) as a linear combination of the standard basis vectors in \\(\\mathbb{{R}}^3\\). Enter your answer as an ordered triple \\((c_1, c_2, c_3)\\)."
            
            answer = f"({c1}, {c2}, {c3})"
            difficulty = 1300
    
    elif problem_type == 'span_check':
        # Check if vector is in span
        # ELO: 1400-1500 (requires understanding span and solving system)
        dim = 2
        
        in_span = random.choice([True, False])
        
        v1 = [random.randint(1, 3), random.randint(-2, 2)]
        v2 = [random.randint(-2, 2), random.randint(1, 3)]
        while v1[0] * v2[1] == v1[1] * v2[0]:
            v2 = [random.randint(-2, 2), random.randint(1, 3)]
        
        if in_span:
            c1, c2 = random.randint(-2, 2), random.randint(-2, 2)
            target = [c1 * v1[0] + c2 * v2[0], c1 * v1[1] + c2 * v2[1]]
            answer = "True"
        else:
            target = [random.randint(-5, 5), random.randint(-5, 5)]
            # Verify not in span
            det = v1[0] * v2[1] - v1[1] * v2[0]
            if det != 0:
                calc_c1 = (target[0] * v2[1] - target[1] * v2[0]) / det
                if calc_c1 == int(calc_c1):
                    target[0] += 1
            answer = "False"
        
        question = f"Is the vector \\(\\begin{{pmatrix}} {target[0]} \\\\ {target[1]} \\end{{pmatrix}}\\) in the span of \\(\\left\\{{\\begin{{pmatrix}} {v1[0]} \\\\ {v1[1]} \\end{{pmatrix}}, \\begin{{pmatrix}} {v2[0]} \\\\ {v2[1]} \\end{{pmatrix}}\\right\\}}\\)? Answer True or False."
        
        difficulty = 1450
    
    elif problem_type == 'linear_independence':
        # Check linear independence
        # ELO: 1500-1600 (requires understanding determinant or row reduction)
        dim = 3
        
        independent = random.choice([True, False])
        
        if independent:
            # Create independent vectors using identity base
            v1 = [random.randint(1, 2), random.randint(-1, 1), random.randint(-1, 1)]
            v2 = [random.randint(-1, 1), random.randint(1, 2), random.randint(-1, 1)]
            v3 = [random.randint(-1, 1), random.randint(-1, 1), random.randint(1, 2)]
            
            # Verify determinant non-zero
            mat = sp.Matrix([v1, v2, v3]).T
            if mat.det() == 0:
                v3[2] += 1
            
            answer = "True"
        else:
            v1 = [random.randint(1, 3), random.randint(-2, 2), random.randint(-2, 2)]
            v2 = [random.randint(-2, 2), random.randint(1, 3), random.randint(-2, 2)]
            c1, c2 = random.randint(1, 2), random.randint(1, 2)
            v3 = [c1 * v1[0] + c2 * v2[0], c1 * v1[1] + c2 * v2[1], c1 * v1[2] + c2 * v2[2]]
            
            answer = "False"
        
        question = f"Are the vectors \\(\\begin{{pmatrix}} {v1[0]} \\\\ {v1[1]} \\\\ {v1[2]} \\end{{pmatrix}}\\), \\(\\begin{{pmatrix}} {v2[0]} \\\\ {v2[1]} \\\\ {v2[2]} \\end{{pmatrix}}\\), and \\(\\begin{{pmatrix}} {v3[0]} \\\\ {v3[1]} \\\\ {v3[2]} \\end{{pmat