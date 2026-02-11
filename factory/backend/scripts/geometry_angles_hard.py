"""
geometry - angles (hard)
Generated: 2026-02-11T21:39:52.565921
"""

import random
import json
from sympy import symbols, Eq, solve, simplify, cos, sin, tan, rad, deg, pi, sqrt, atan, asin, acos

def generate_angle_problem():
    problem_type = random.choice([
        'parallel_lines_complex',
        'triangle_angle_bisector',
        'inscribed_angle_theorem',
        'exterior_angle_multiple',
        'cyclic_quadrilateral',
        'angle_in_polygon',
        'trigonometric_angle',
        'complementary_supplementary_system'
    ])
    
    if problem_type == 'parallel_lines_complex':
        answer = random.randint(20, 80)
        ratio1 = random.randint(2, 5)
        ratio2 = random.randint(2, 5)
        total_angle = 180
        
        other_angle = total_angle - answer
        coeff1 = random.randint(1, 3)
        coeff2 = random.randint(1, 3)
        
        const_term = other_angle - coeff1 * answer
        
        question = f"Two parallel lines are cut by a transversal. One of the corresponding angles measures $({coeff1}x + {const_term})°$ and another angle on the alternate interior position measures $x°$. Find the value of $x$."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1650,
            "main_topic": "geometry",
            "subtopic": "angles",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'triangle_angle_bisector':
        answer = random.randint(30, 70)
        
        half_angle = answer / 2
        base_angle = random.randint(40, 70)
        third_angle = 180 - answer - base_angle
        
        const = base_angle - 2 * half_angle
        
        question = f"In triangle ABC, the angle bisector of angle A creates two angles. If angle B measures {base_angle}°, angle C measures $(2x + {const})°$, and one of the angles formed by the bisector is $x°$, find $x$."
        
        return {
            "question_latex": question,
            "answer_key": str(half_angle),
            "difficulty": 1700,
            "main_topic": "geometry",
            "subtopic": "angles",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'inscribed_angle_theorem':
        central_answer = random.randint(80, 160)
        inscribed = central_answer / 2
        
        coeff = random.randint(2, 4)
        const = central_answer - coeff * inscribed
        
        question = f"A circle has a central angle measuring $({coeff}x + {const})°$ and an inscribed angle intercepting the same arc measuring $x°$. Find the measure of the inscribed angle."
        
        return {
            "question_latex": question,
            "answer_key": str(inscribed),
            "difficulty": 1680,
            "main_topic": "geometry",
            "subtopic": "angles",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'exterior_angle_multiple':
        answer = random.randint(25, 60)
        
        remote1 = random.randint(30, 70)
        remote2 = answer - remote1
        
        coeff1 = random.randint(2, 3)
        coeff2 = random.randint(2, 3)
        
        const1 = remote1 - coeff1 * answer
        const2 = remote2 - coeff2 * answer
        
        question = f"An exterior angle of a triangle measures $x°$. The two non-adjacent interior angles measure $({coeff1}x + {const1})°$ and $({coeff2}x + {const2})°$. Find $x$."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1620,
            "main_topic": "geometry",
            "subtopic": "angles",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'cyclic_quadrilateral':
        answer = random.randint(60, 120)
        opposite = 180 - answer
        
        coeff1 = random.randint(2, 4)
        const1 = answer - coeff1 * 30
        adj_to_answer = random.randint(50, 100)
        
        question = f"In a cyclic quadrilateral ABCD, angle A measures $({coeff1} \\cdot 30 + {const1})°$, angle B measures {adj_to_answer}°, and angle C measures $y°$. If opposite angles in a cyclic quadrilateral are supplementary, find the measure of angle A."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1750,
            "main_topic": "geometry",
            "subtopic": "angles",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'angle_in_polygon':
        n = random.randint(6, 9)
        answer = (n - 2) * 180 / n
        
        coeff = random.randint(2, 3)
        const = answer - coeff * (answer / coeff)
        
        question = f"Each interior angle of a regular {n}-gon measures $x°$. Express the relationship and find $x$."
        
        return {
            "question_latex": question,
            "answer_key": str(int(answer) if answer == int(answer) else answer),
            "difficulty": 1670,
            "main_topic": "geometry",
            "subtopic": "angles",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'trigonometric_angle':
        answer = random.choice([30, 45, 60])
        
        complement = 90 - answer
        
        coeff1 = random.randint(2, 3)
        coeff2 = random.randint(1, 2)
        const = complement - coeff2 * answer
        
        question = f"Two angles are complementary. One angle measures $({coeff1}x - {coeff1 * answer - answer})°$ and the other measures $({coeff2}x + {const})°$. Find the measure of the first angle."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1640,
            "main_topic": "geometry",
            "subtopic": "angles",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'complementary_supplementary_system':
        answer = random.randint(40, 80)
        
        supplement = 180 - answer
        complement_of_supplement = 90 - supplement
        
        if complement_of_supplement < 0:
            answer = random.randint(40, 60)
            supplement = 180 - answer
            complement_of_supplement = 90 - supplement
        
        ratio = random.randint(2, 4)
        related_angle = answer / ratio
        const = complement_of_supplement + related_angle
        
        question = f"An angle $x$ has a supplement. The complement of this supplement equals $(x/{ratio} - {const})°$. Find $x$."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1800,
            "main_topic": "geometry",
            "subtopic": "angles",
            "gr