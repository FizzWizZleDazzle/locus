"""
precalculus - vectors (medium)
Generated: 2026-02-11T21:57:10.311989
"""

import random
import json
from sympy import symbols, sqrt, simplify, latex, Rational, cos, sin, pi, acos, N
from sympy.vector import CoordSys3D

def generate_vector_problem():
    problem_type = random.choice([
        'magnitude_2d',
        'magnitude_3d',
        'dot_product',
        'unit_vector',
        'angle_between',
        'scalar_mult_add',
        'component_form'
    ])
    
    if problem_type == 'magnitude_2d':
        # ELO: 1300 - Two-step: square components, take square root
        mag_squared = random.choice([25, 36, 49, 64, 100, 169])
        mag = int(sqrt(mag_squared))
        
        # Find integer pairs that give this magnitude
        pairs = [(a, b) for a in range(-15, 16) for b in range(-15, 16) if a*a + b*b == mag_squared]
        if pairs:
            x, y = random.choice(pairs)
            question = f"Find the magnitude of the vector $\\vec{{v}} = \\langle {x}, {y} \\rangle$."
            answer = str(mag)
            elo = 1300
        else:
            x, y = 3, 4
            answer = "5"
            question = f"Find the magnitude of the vector $\\vec{{v}} = \\langle {x}, {y} \\rangle$."
            elo = 1300
    
    elif problem_type == 'magnitude_3d':
        # ELO: 1350 - Similar to 2D but with three components
        mag_squared = random.choice([9, 16, 25, 36, 49])
        mag = int(sqrt(mag_squared))
        
        triples = [(a, b, c) for a in range(-10, 11) for b in range(-10, 11) for c in range(-10, 11) 
                   if a*a + b*b + c*c == mag_squared and (a != 0 or b != 0 or c != 0)]
        if triples:
            x, y, z = random.choice(triples)
            question = f"Find the magnitude of the vector $\\vec{{v}} = \\langle {x}, {y}, {z} \\rangle$."
            answer = str(mag)
            elo = 1350
        else:
            x, y, z = 2, 2, 1
            answer = "3"
            question = f"Find the magnitude of the vector $\\vec{{v}} = \\langle {x}, {y}, {z} \\rangle$."
            elo = 1350
    
    elif problem_type == 'dot_product':
        # ELO: 1400 - Multi-step: multiply components, add results
        result = random.randint(-20, 20)
        
        a1 = random.randint(-5, 5)
        a2 = random.randint(-5, 5)
        b1 = random.randint(-5, 5)
        
        if a1 != 0:
            b2 = (result - a1*b1) // a2 if a2 != 0 else random.randint(-5, 5)
            if a2 != 0 and a1*b1 + a2*b2 == result:
                question = f"Find the dot product of $\\vec{{u}} = \\langle {a1}, {a2} \\rangle$ and $\\vec{{v}} = \\langle {b1}, {b2} \\rangle$."
                answer = str(result)
                elo = 1400
            else:
                a1, a2, b1, b2 = 2, 3, 4, -1
                answer = str(2*4 + 3*(-1))
                question = f"Find the dot product of $\\vec{{u}} = \\langle {a1}, {a2} \\rangle$ and $\\vec{{v}} = \\langle {b1}, {b2} \\rangle$."
                elo = 1400
        else:
            a1, a2, b1, b2 = 2, 3, 4, -1
            answer = str(2*4 + 3*(-1))
            question = f"Find the dot product of $\\vec{{u}} = \\langle {a1}, {a2} \\rangle$ and $\\vec{{v}} = \\langle {b1}, {b2} \\rangle$."
            elo = 1400
    
    elif problem_type == 'unit_vector':
        # ELO: 1500 - Requires finding magnitude first, then dividing each component
        mag_squared = random.choice([25, 36, 49, 100])
        mag = int(sqrt(mag_squared))
        
        pairs = [(a, b) for a in range(-15, 16) for b in range(-15, 16) if a*a + b*b == mag_squared]
        if pairs:
            x, y = random.choice(pairs)
            ux = Rational(x, mag)
            uy = Rational(y, mag)
            question = f"Find the unit vector in the direction of $\\vec{{v}} = \\langle {x}, {y} \\rangle$. Express as $\\langle a, b \\rangle$."
            answer = f"({ux}, {uy})"
            elo = 1500
        else:
            x, y = 3, 4
            answer = "(3/5, 4/5)"
            question = f"Find the unit vector in the direction of $\\vec{{v}} = \\langle {x}, {y} \\rangle$. Express as $\\langle a, b \\rangle$."
            elo = 1500
    
    elif problem_type == 'angle_between':
        # ELO: 1600 - Complex: dot product, magnitudes, inverse cosine
        # Use vectors that give clean angles
        angle_choices = [
            (60, [(1, 0), (1, sqrt(3))]),
            (90, [(1, 0), (0, 1)]),
            (45, [(1, 0), (1, 1)])
        ]
        
        angle_deg, vecs = random.choice(angle_choices)
        v1, v2 = vecs
        
        x1, y1 = v1
        x2, y2 = v2
        
        question = f"Find the angle in degrees between vectors $\\vec{{u}} = \\langle {x1}, {y1} \\rangle$ and $\\vec{{v}} = \\langle {latex(x2)}, {latex(y2)} \\rangle$."
        answer = str(angle_deg)
        elo = 1600
    
    elif problem_type == 'scalar_mult_add':
        # ELO: 1350 - Two operations: scalar multiplication and addition
        k = random.randint(2, 5)
        
        x1, y1 = random.randint(-5, 5), random.randint(-5, 5)
        x2, y2 = random.randint(-5, 5), random.randint(-5, 5)
        
        rx = k * x1 + x2
        ry = k * y1 + y2
        
        question = f"If $\\vec{{u}} = \\langle {x1}, {y1} \\rangle$ and $\\vec{{v}} = \\langle {x2}, {y2} \\rangle$, find ${k}\\vec{{u}} + \\vec{{v}}$."
        answer = f"({rx}, {ry})"
        elo = 1350
    
    else:  # component_form
        # E