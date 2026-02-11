"""
geometry - area_perimeter (hard)
Generated: 2026-02-11T21:42:32.496128
"""

import sympy as sp
import random
import json

def generate_problem():
    random.seed()
    problem_type = random.choice([
        'composite_with_cutout',
        'optimization_fence',
        'inscribed_shapes',
        'irregular_polygon',
        'path_area'
    ])
    
    if problem_type == 'composite_with_cutout':
        outer_length = random.randint(12, 20)
        outer_width = random.randint(10, 16)
        cutout_length = random.randint(4, outer_length // 2)
        cutout_width = random.randint(3, outer_width // 2)
        
        answer = outer_length * outer_width - cutout_length * cutout_width
        
        question = f"A rectangular garden measures ${outer_length}$ meters by ${outer_width}$ meters. A rectangular pond measuring ${cutout_length}$ meters by ${cutout_width}$ meters is built inside the garden. What is the area of the remaining garden space (in square meters)?"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": random.randint(1600, 1700),
            "main_topic": "geometry",
            "subtopic": "area_perimeter",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'optimization_fence':
        area = random.choice([120, 144, 180, 200, 240])
        factors = [(i, area // i) for i in range(1, int(area**0.5) + 1) if area % i == 0]
        
        min_perimeter = float('inf')
        best_l, best_w = 0, 0
        for l, w in factors:
            perim = 2 * (l + w)
            if perim < min_perimeter:
                min_perimeter = perim
                best_l, best_w = l, w
        
        answer = int(min_perimeter)
        
        question = f"A farmer wants to enclose a rectangular area of ${area}$ square meters using the minimum amount of fencing. The dimensions must be whole numbers. What is the minimum perimeter (in meters) required?"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": random.randint(1650, 1750),
            "main_topic": "geometry",
            "subtopic": "area_perimeter",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'inscribed_shapes':
        radius = random.randint(6, 12)
        
        side = radius * sp.sqrt(2)
        area = side ** 2
        answer = sp.simplify(area)
        
        question = f"A square is inscribed in a circle with radius ${radius}$ units. What is the area of the square? Express your answer in simplest form."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": random.randint(1700, 1800),
            "main_topic": "geometry",
            "subtopic": "area_perimeter",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'irregular_polygon':
        base1 = random.randint(8, 15)
        base2 = random.randint(12, 20)
        height = random.randint(6, 12)
        
        answer = sp.Rational(base1 + base2, 2) * height
        
        question = f"A trapezoid has parallel sides of lengths ${base1}$ cm and ${base2}$ cm, with a perpendicular height of ${height}$ cm. Find the area of the trapezoid in square centimeters."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": random.randint(1620, 1720),
            "main_topic": "geometry",
            "subtopic": "area_perimeter",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'path_area':
        inner_length = random.randint(15, 25)
        inner_width = random.randint(10, 18)
        path_width = random.randint(2, 4)
        
        outer_length = inner_length + 2 * path_width
        outer_width = inner_width + 2 * path_width
        
        answer = outer_length * outer_width - inner_length * inner_width
        
        question = f"A rectangular swimming pool measures ${inner_length}$ feet by ${inner_width}$ feet. A uniform walkway of width ${path_width}$ feet surrounds the pool. What is the area of the walkway (in square feet)?"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": random.randint(1680, 1780),
            "main_topic": "geometry",
            "subtopic": "area_perimeter",
            "grading_mode": "equivalent"
        }

problem = generate_problem()
print(json.dumps(problem))