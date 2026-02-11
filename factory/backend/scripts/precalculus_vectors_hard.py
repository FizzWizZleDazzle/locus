"""
precalculus - vectors (hard)
Generated: 2026-02-11T21:57:37.165610
"""

import sympy as sp
import random
import json

def generate_vector_problem():
    problem_type = random.choice([
        'projection_magnitude',
        'vector_equation_plane',
        'cross_product_area',
        'triple_scalar_product',
        'vector_parametric_intersection',
        'dot_cross_combination'
    ])
    
    if problem_type == 'projection_magnitude':
        # Generate clean projection: proj_v(u) = ((u·v)/(v·v))v
        # Pick answer first: a clean vector
        proj_coords = [random.randint(-3, 3) for _ in range(3)]
        while all(c == 0 for c in proj_coords):
            proj_coords = [random.randint(-3, 3) for _ in range(3)]
        
        # Pick v with small integer components
        v = [random.randint(1, 4) for _ in range(3)]
        v_dot_v = sum(a*a for a in v)
        
        # Reverse engineer u: proj = ((u·v)/(v·v))v, so u·v = proj·v * (v·v) / |v|^2
        # Actually: proj = k*v where k = (u·v)/(v·v)
        k = sp.Rational(random.randint(1, 5), random.randint(1, 4))
        proj_coords = [k * vc for vc in v]
        
        # u·v = k * v·v
        u_dot_v = k * v_dot_v
        
        # Create u such that u·v has this value
        # u = proj + component perpendicular to v
        perp = [random.randint(-2, 2) for _ in range(3)]
        # Make perp actually perpendicular: adjust
        dot_perp_v = sum(perp[i] * v[i] for i in range(3))
        perp[0] = perp[0] - dot_perp_v / v_dot_v * v[0] if v[0] != 0 else perp[0]
        
        u = [proj_coords[i] + perp[i] for i in range(3)]
        
        magnitude = sp.sqrt(sum((proj_coords[i])**2 for i in range(3)))
        magnitude = sp.simplify(magnitude)
        
        question = f"Let \\mathbf{{u}} = \\langle {sp.latex(u[0])}, {sp.latex(u[1])}, {sp.latex(u[2])} \\rangle and \\mathbf{{v}} = \\langle {v[0]}, {v[1]}, {v[2]} \\rangle. Find the magnitude of the projection of \\mathbf{{u}} onto \\mathbf{{v}}."
        
        return {
            "question_latex": question,
            "answer_key": str(magnitude),
            "difficulty": random.randint(1650, 1750),
            "main_topic": "precalculus",
            "subtopic": "vectors",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'vector_equation_plane':
        # Find scalar t: given point on plane and normal vector
        # Plane: n·(r - r0) = 0, parametric line r = a + tb
        # Solve: n·(a + tb - r0) = 0 for t
        
        n = [random.randint(-3, 3) for _ in range(3)]
        while sum(abs(x) for x in n) < 3:
            n = [random.randint(-3, 3) for _ in range(3)]
        
        # Pick clean answer t
        t_ans = sp.Rational(random.randint(-5, 5), random.randint(1, 4))
        
        # Pick direction vector b
        b = [random.randint(-3, 3) for _ in range(3)]
        while all(x == 0 for x in b):
            b = [random.randint(-3, 3) for _ in range(3)]
        
        # Pick point on plane r0
        r0 = [random.randint(-4, 4) for _ in range(3)]
        
        # Reverse: n·(a + t*b - r0) = 0
        # n·a + t(n·b) - n·r0 = 0
        # t = (n·r0 - n·a)/(n·b)
        
        n_dot_b = sum(n[i]*b[i] for i in range(3))
        if n_dot_b == 0:
            n[0] = n[0] + b[0]
            n_dot_b = sum(n[i]*b[i] for i in range(3))
        
        n_dot_r0 = sum(n[i]*r0[i] for i in range(3))
        # t_ans = (n·r0 - n·a)/(n·b)
        # n·a = n·r0 - t_ans * n·b
        
        n_dot_a = n_dot_r0 - t_ans * n_dot_b
        
        # Create a such that n·a = n_dot_a
        a = [random.randint(-3, 3) for _ in range(3)]
        current_dot = sum(n[i]*a[i] for i in range(3))
        a[0] = a[0] + (n_dot_a - current_dot) / n[0] if n[0] != 0 else a[0]
        
        question = f"A line passes through point \\mathbf{{a}} = \\langle {sp.latex(a[0])}, {sp.latex(a[1])}, {sp.latex(a[2])} \\rangle in the direction of \\mathbf{{b}} = \\langle {b[0]}, {b[1]}, {b[2]} \\rangle. At what value of the parameter t does this line intersect the plane with normal vector \\mathbf{{n}} = \\langle {n[0]}, {n[1]}, {n[2]} \\rangle passing through \\mathbf{{r}}_0 = \\langle {r0[0]}, {r0[1]}, {r0[2]} \\rangle?"
        
        return {
            "question_latex": question,
            "answer_key": str(t_ans),
            "difficulty": random.randint(1700, 1800),
            "main_topic": "precalculus",
            "subtopic": "vectors",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'cross_product_area':
        # Area of parallelogram = |a × b|
        # Pick clean magnitude answer
        area_squared = random.choice([4, 9, 16, 25, 36, 49, 64, 100])
        area = sp.sqrt(area_squared)
        
        a = [random.randint(-4, 4) for _ in range(3)]
        b = [random.randint(-4, 4) for _ in range(3)]
        
        # Calculate cross product
        cross = [
            a[1]*b[2] - a[2]*b[1],
            a[2]*b[0] - a[0]*b[2],
            a[0]*b[1] - a[1]*b[0]
        ]
        
        mag_cross = sp.sqrt(sum(c**2 for c in cross))
        
        question = f"Find the area of the parallelogram with adjacent sides \\mathbf{{a}} = \\langle {a[0]}, {a[1]}, {a[2]} \\rangle and \\mathbf{{b}} = \\langle {b[