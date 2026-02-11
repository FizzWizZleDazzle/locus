"""
geometry - pythagorean_theorem (hard)
Generated: 2026-02-11T21:44:10.367268
"""

import random
import json
from sympy import symbols, sqrt, simplify, latex, Rational

def generate_problem():
    problem_type = random.choice([
        'nested_right_triangles',
        'coordinate_distance_with_algebra',
        'three_dimensional_diagonal',
        'inverse_problem',
        'pythagorean_with_ratios'
    ])
    
    if problem_type == 'nested_right_triangles':
        # Two right triangles sharing a side - requires two applications
        a = random.randint(3, 8)
        b = random.randint(3, 8)
        c = sqrt(a**2 + b**2)
        
        d = random.randint(3, 8)
        e = sqrt(c**2 + d**2)
        
        answer = simplify(e)
        
        question = f"In triangle ABC, angle B is 90°, AB = {a}, and BC = {b}. Point D is such that CD is perpendicular to AC and CD = {d}. Find the length of AD."
        difficulty = 1650
        
    elif problem_type == 'coordinate_distance_with_algebra':
        # Point on line, distance to another point
        x_coord = random.randint(2, 10)
        y_base = random.randint(2, 8)
        k = random.randint(2, 5)
        
        # Point A at origin, Point B at (x_coord, k*x_coord + y_base)
        # Find distance - requires coordinate geometry + Pythagorean
        dist_squared = x_coord**2 + (k*x_coord + y_base)**2
        answer = sqrt(dist_squared)
        
        question = f"Point A is at the origin (0,0) and point B is at ({x_coord}, {k*x_coord + y_base}). Find the distance AB."
        difficulty = 1600
        
    elif problem_type == 'three_dimensional_diagonal':
        # Space diagonal of rectangular prism
        a = random.randint(2, 8)
        b = random.randint(2, 8)
        c = random.randint(2, 8)
        
        answer = sqrt(a**2 + b**2 + c**2)
        
        question = f"A rectangular box has dimensions {a} by {b} by {c}. Find the length of the space diagonal (the distance from one corner to the opposite corner through the interior)."
        difficulty = 1700
        
    elif problem_type == 'inverse_problem':
        # Given hypotenuse and one leg, find other leg
        # Then use that in another calculation
        leg1 = random.randint(5, 12)
        hyp = random.randint(leg1 + 3, leg1 + 8)
        
        leg2_squared = hyp**2 - leg1**2
        if leg2_squared <= 0:
            leg1 = 5
            hyp = 13
            leg2_squared = 144
        
        leg2 = sqrt(leg2_squared)
        
        # Area calculation requires finding the other leg first
        area = Rational(1, 2) * leg1 * leg2
        answer = simplify(area)
        
        question = f"A right triangle has hypotenuse {hyp} and one leg of length {leg1}. Find the area of the triangle."
        difficulty = 1650
        
    else:  # pythagorean_with_ratios
        # Legs are in ratio, hypotenuse given
        ratio_a = random.randint(3, 7)
        ratio_b = random.randint(3, 7)
        if ratio_a == ratio_b:
            ratio_b += 1
        
        k = random.randint(2, 5)
        hyp_squared = k**2 * (ratio_a**2 + ratio_b**2)
        hyp = k * sqrt(ratio_a**2 + ratio_b**2)
        
        # Find the longer leg
        longer_ratio = max(ratio_a, ratio_b)
        answer = k * longer_ratio
        
        question = f"In a right triangle, the legs are in the ratio {ratio_a}:{ratio_b} and the hypotenuse is {latex(hyp)}. Find the length of the longer leg."
        difficulty = 1750
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "pythagorean_theorem",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_problem()
print(json.dumps(problem))