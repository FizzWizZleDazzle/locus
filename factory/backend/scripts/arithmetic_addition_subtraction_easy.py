"""
arithmetic - addition_subtraction (easy)
Generated: 2026-02-11T21:22:04.568638
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'single_digit_addition',
        'single_digit_subtraction',
        'two_digit_addition',
        'two_digit_subtraction',
        'mixed_two_step',
        'three_number_simple'
    ])
    
    if problem_type == 'single_digit_addition':
        answer = random.randint(2, 18)
        a = random.randint(1, min(9, answer - 1))
        b = answer - a
        question = f"{a} + {b}"
        elo = random.randint(1000, 1100)
        
    elif problem_type == 'single_digit_subtraction':
        a = random.randint(2, 18)
        answer = random.randint(1, min(9, a - 1))
        b = a - answer
        question = f"{a} - {b}"
        elo = random.randint(1000, 1100)
        
    elif problem_type == 'two_digit_addition':
        answer = random.randint(20, 99)
        a = random.randint(10, answer - 10)
        b = answer - a
        question = f"{a} + {b}"
        elo = random.randint(1100, 1200)
        
    elif problem_type == 'two_digit_subtraction':
        a = random.randint(20, 99)
        answer = random.randint(10, a - 10)
        b = a - answer
        question = f"{a} - {b}"
        elo = random.randint(1100, 1200)
        
    elif problem_type == 'mixed_two_step':
        answer = random.randint(5, 50)
        a = random.randint(10, 40)
        b = random.randint(5, 20)
        c = a + b - answer
        question = f"{a} + {b} - {c}"
        elo = random.randint(1200, 1280)
        
    elif problem_type == 'three_number_simple':
        answer = random.randint(10, 60)
        a = random.randint(5, 25)
        b = random.randint(5, 25)
        c = answer - a - b
        if c < 1:
            c = random.randint(1, 10)
            answer = a + b + c
        question = f"{a} + {b} + {c}"
        elo = random.randint(1220, 1300)
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "arithmetic",
        "subtopic": "addition_subtraction",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_problem()
print(json.dumps(problem))