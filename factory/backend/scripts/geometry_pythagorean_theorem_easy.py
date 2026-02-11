"""
geometry - pythagorean_theorem (easy)
Generated: 2026-02-11T21:43:35.265246
"""

import sympy as sp
import random
import json

def generate_pythagorean_problem():
    problem_type = random.choice(['find_hypotenuse', 'find_leg', 'verify_triangle'])
    
    if problem_type == 'find_hypotenuse':
        # Pick a Pythagorean triple or simple values
        triples = [(3, 4, 5), (5, 12, 13), (8, 15, 17), (7, 24, 25), (6, 8, 10), (9, 12, 15), (12, 16, 20)]
        a, b, c = random.choice(triples)
        
        # Decide difficulty
        if random.random() < 0.5:
            # 1100: Direct application with whole numbers
            question = f"In a right triangle, the two legs have lengths {a} and {b}. Find the length of the hypotenuse."
            answer = sp.sympify(c)
            difficulty = 1100
        else:
            # 1200: With units or slight context
            units = random.choice(['cm', 'm', 'ft', 'inches'])
            question = f"A right triangle has legs of length {a} {units} and {b} {units}. What is the length of the hypotenuse in {units}?"
            answer = sp.sympify(c)
            difficulty = 1200
    
    elif problem_type == 'find_leg':
        # Pick a Pythagorean triple
        triples = [(3, 4, 5), (5, 12, 13), (8, 15, 17), (6, 8, 10), (9, 12, 15), (12, 16, 20)]
        a, b, c = random.choice(triples)
        
        if random.random() < 0.5:
            # Find first leg given second leg and hypotenuse
            question = f"In a right triangle, one leg has length {b} and the hypotenuse has length {c}. Find the length of the other leg."
            answer = sp.sympify(a)
            difficulty = 1250
        else:
            # Find second leg given first leg and hypotenuse
            question = f"A right triangle has a hypotenuse of length {c} and one leg of length {a}. What is the length of the other leg?"
            answer = sp.sympify(b)
            difficulty = 1250
    
    else:  # verify_triangle
        triples = [(3, 4, 5), (5, 12, 13), (6, 8, 10)]
        a, b, c = random.choice(triples)
        
        # 1150: Verify if it's a right triangle
        question = f"Is a triangle with sides {a}, {b}, and {c} a right triangle? Answer 'yes' or 'no'."
        answer = sp.sympify(1)  # We'll use 1 for yes
        difficulty = 1150
        
        # Modify to make output consistent
        question = f"A right triangle has legs of length {a} and {b}. Find the hypotenuse."
        answer = sp.sympify(c)
        difficulty = 1150
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "pythagorean_theorem",
        "grading_mode": "equivalent"
    }

problem = generate_pythagorean_problem()
print(json.dumps(problem))