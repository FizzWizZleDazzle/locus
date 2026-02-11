"""
geometry - triangles (medium)
Generated: 2026-02-11T21:40:34.470462
"""

import sympy as sp
import random
import json

def generate_triangle_problem():
    problem_type = random.choice([
        'pythagorean_given_two_sides',
        'angle_sum_two_unknowns',
        'isosceles_angles',
        'perimeter_with_variable',
        'pythagorean_with_expression',
        'triangle_inequality'
    ])
    
    if problem_type == 'pythagorean_given_two_sides':
        # ELO 1300-1400: Standard Pythagorean theorem
        # Pick clean hypotenuse, work backward
        hypotenuse = random.choice([5, 10, 13, 15, 17, 20, 25])
        
        # Common Pythagorean triples
        triples = {
            5: [(3, 4)],
            10: [(6, 8)],
            13: [(5, 12)],
            15: [(9, 12)],
            17: [(8, 15)],
            20: [(12, 16)],
            25: [(7, 24), (15, 20)]
        }
        
        if hypotenuse in triples:
            a, b = random.choice(triples[hypotenuse])
            give_hypotenuse = random.choice([True, False])
            
            if give_hypotenuse:
                question = f"In a right triangle, the hypotenuse is {hypotenuse} and one leg is {a}. Find the length of the other leg."
                answer = b
            else:
                question = f"In a right triangle, the two legs are {a} and {b}. Find the length of the hypotenuse."
                answer = hypotenuse
            
            return {
                "question_latex": question,
                "answer_key": str(answer),
                "difficulty": 1350,
                "main_topic": "geometry",
                "subtopic": "triangles",
                "grading_mode": "equivalent"
            }
    
    elif problem_type == 'angle_sum_two_unknowns':
        # ELO 1300-1400: Two-step angle problem
        # Pick answer angles first
        angle1 = random.randint(30, 80)
        angle2 = 180 - angle1 - random.randint(20, 70)
        angle3 = 180 - angle1 - angle2
        
        # Express one angle in terms of variable
        x_val = random.randint(5, 20)
        coeff = random.randint(2, 4)
        
        # angle1 = coeff*x + const
        const = angle1 - coeff * x_val
        
        question = f"In a triangle, one angle measures {coeff}x + {const} degrees, another angle measures {angle2} degrees, and the third angle measures {angle3} degrees. Find the value of x."
        
        return {
            "question_latex": question,
            "answer_key": str(x_val),
            "difficulty": 1400,
            "main_topic": "geometry",
            "subtopic": "triangles",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'isosceles_angles':
        # ELO 1350-1450: Isosceles triangle angle problem
        # Pick vertex angle, calculate base angles
        vertex_angle = random.choice([40, 50, 60, 70, 80, 100, 120])
        base_angle = (180 - vertex_angle) // 2
        
        question = f"An isosceles triangle has a vertex angle of {vertex_angle} degrees. Find the measure of each base angle in degrees."
        
        return {
            "question_latex": question,
            "answer_key": str(base_angle),
            "difficulty": 1350,
            "main_topic": "geometry",
            "subtopic": "triangles",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'perimeter_with_variable':
        # ELO 1450-1550: Multi-step with expressions
        x_val = random.randint(3, 10)
        side1_coeff = random.randint(2, 4)
        side2_coeff = random.randint(2, 4)
        side3 = random.randint(5, 15)
        
        perimeter = side1_coeff * x_val + side2_coeff * x_val + side3
        
        question = f"A triangle has sides of length {side1_coeff}x, {side2_coeff}x, and {side3}. If the perimeter is {perimeter}, find the value of x."
        
        return {
            "question_latex": question,
            "answer_key": str(x_val),
            "difficulty": 1500,
            "main_topic": "geometry",
            "subtopic": "triangles",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'pythagorean_with_expression':
        # ELO 1500-1600: Pythagorean with algebraic expressions
        x_val = random.randint(3, 8)
        
        # Use a Pythagorean triple: 3, 4, 5
        scale = random.choice([1, 2, 3])
        a_base = 3 * scale
        b_base = 4 * scale
        c_base = 5 * scale
        
        # Express sides with x: a = a_base*x, b = b_base*x, c = c_base*x
        # Or a = x+k, b = x+m, c = ?
        
        # Simpler: a = x, b = x+k, where we know the answer
        # Let's use: legs are x and x+2, hypotenuse is known
        # x^2 + (x+2)^2 = c^2
        # For x=3: 9 + 25 = 34, sqrt(34) not clean
        # For x=6: 36 + 64 = 100, c=10 (clean!)
        
        if x_val == 6:
            question = f"In a right triangle, one leg is x and the other leg is x+2. If the hypotenuse is 10, find the value of x."
            answer = 6
        else:
            # Fallback to simpler expression
            a = x_val
            b = random.randint(4, 12)
            c_squared = a**2 + b**2
            c = int(sp.sqrt(c_squared))
            if c**2 == c_squared:
                question = f"In a right triangle, one leg is x, the other leg is {b}, and the hypotenuse is {c}. Find the value of x."
                answer = a
            else:
                # Force a clean answer
                a, b, c = 5, 12, 13
                question = f"In a right triangle, one leg is x, the other leg is {b}, and the hypotenuse is {c}. Find the value of x."
                answer = a
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1550,
            "main_topic": "geometry",
            "subtopic": "triangles",
            "grading_mode": "equivalent"
        }
    
    else:  # triangle_inequality
        # ELO 1400-1500: Triangle inequality application
        x_val = random.randint(5, 15)
        side1 = random.randint(8, 20)
        side2 = random.randint(8, 20)
        
        # Ensure x_val satisfies triangle inequality
        # |side1 - side2| < x < side1 + side2
        lower = abs(side1 - side2) +