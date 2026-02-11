"""
geometry - volume_surface_area (hard)
Generated: 2026-02-11T21:43:22.116737
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'composite_volume',
        'sphere_in_cylinder',
        'cone_frustum',
        'optimization_surface_area',
        'related_volumes',
        'surface_area_composite'
    ])
    
    if problem_type == 'composite_volume':
        # Cylinder with hemisphere on top - harder composite
        r = random.choice([3, 4, 5, 6])
        h_cylinder = random.choice([8, 10, 12, 15])
        
        # Volume = πr²h + (2/3)πr³
        volume = sp.pi * r**2 * h_cylinder + sp.Rational(2, 3) * sp.pi * r**3
        
        question = f"A composite solid consists of a cylinder with radius {r} cm and height {h_cylinder} cm, topped with a hemisphere of the same radius. Find the total volume in terms of \\pi."
        answer = sp.simplify(volume)
        difficulty = 1650
        
    elif problem_type == 'sphere_in_cylinder':
        # Sphere inscribed in cylinder, find ratio or difference
        r = random.choice([4, 5, 6, 8])
        
        # Cylinder: height = diameter = 2r, radius = r
        # Volume difference = πr²(2r) - (4/3)πr³ = 2πr³ - (4/3)πr³ = (2/3)πr³
        vol_diff = sp.Rational(2, 3) * sp.pi * r**3
        
        question = f"A sphere with radius {r} cm is inscribed in a cylinder such that it touches the top, bottom, and sides. Find the difference between the volume of the cylinder and the volume of the sphere, in terms of \\pi."
        answer = sp.simplify(vol_diff)
        difficulty = 1700
        
    elif problem_type == 'cone_frustum':
        # Frustum of a cone
        r1 = random.choice([3, 4, 5, 6])
        r2 = random.choice([6, 8, 9, 10, 12])
        if r2 <= r1:
            r2 = r1 + random.choice([3, 4, 5])
        h = random.choice([8, 9, 12])
        
        # Volume = (1/3)πh(r1² + r1*r2 + r2²)
        volume = sp.Rational(1, 3) * sp.pi * h * (r1**2 + r1*r2 + r2**2)
        
        question = f"A frustum of a cone has a lower base radius of {r2} cm, an upper base radius of {r1} cm, and a height of {h} cm. Find its volume in terms of \\pi."
        answer = sp.simplify(volume)
        difficulty = 1750
        
    elif problem_type == 'optimization_surface_area':
        # Cylinder with fixed volume, minimize surface area
        V = random.choice([128, 216, 250, 288, 324]) * sp.pi
        
        # Given V = πr²h, we have h = V/(πr²)
        # Surface area S = 2πr² + 2πrh = 2πr² + 2πr(V/(πr²)) = 2πr² + 2V/r
        # dS/dr = 4πr - 2V/r² = 0
        # 4πr = 2V/r²
        # r³ = V/(2π)
        
        r_cubed = V / (2 * sp.pi)
        r_optimal = sp.simplify(r_cubed ** sp.Rational(1, 3))
        h_optimal = V / (sp.pi * r_optimal**2)
        h_optimal = sp.simplify(h_optimal)
        
        question = f"A cylinder has a fixed volume of {sp.latex(V)} \\text{{ cm}}^3. Find the radius that minimizes the total surface area (including both circular bases)."
        answer = sp.simplify(r_optimal)
        difficulty = 1850
        
    elif problem_type == 'related_volumes':
        # Two spheres, ratio problem
        r1 = random.choice([2, 3, 4])
        ratio = random.choice([2, 3])
        r2 = r1 * ratio
        
        # Surface area ratio
        sa_ratio = (r2**2) / (r1**2)
        
        question = f"Two spheres have radii {r1} cm and {r2} cm respectively. If the volume of the larger sphere is k times the volume of the smaller sphere, find the ratio of their surface areas (larger to smaller)."
        answer = sp.simplify(sa_ratio)
        difficulty = 1680
        
    else:  # surface_area_composite
        # Rectangular prism with square pyramid on top
        base = random.choice([4, 6, 8, 10])
        h_prism = random.choice([6, 8, 10, 12])
        h_pyramid = random.choice([3, 4, 5, 6])
        
        # Calculate slant height of pyramid
        slant_height = sp.sqrt((base/2)**2 + h_pyramid**2)
        
        # Surface area = base² + 4(base × h_prism) + 4(1/2 × base × slant_height)
        # = base² + 4base×h_prism + 2base×slant_height
        sa = base**2 + 4*base*h_prism + 2*base*slant_height
        
        question = f"A composite solid consists of a rectangular prism with a square base of side {base} cm and height {h_prism} cm, topped with a square pyramid with the same base and height {h_pyramid} cm. Find the total surface area of the composite solid (excluding the interface between prism and pyramid)."
        answer = sp.simplify(sa)
        difficulty = 1720
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "volume_surface_area",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))