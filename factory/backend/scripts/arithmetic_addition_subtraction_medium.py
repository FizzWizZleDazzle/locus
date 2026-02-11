"""
arithmetic - addition_subtraction (medium)
Generated: 2026-02-11T21:22:15.334417
"""

import random
import json
from sympy import symbols, sympify, latex

def generate_problem():
    difficulty_target = random.randint(1300, 1600)
    
    if difficulty_target < 1350:
        # Two-step: a + b - c where one operation is needed
        answer = random.randint(10, 50)
        b = random.randint(5, 20)
        a = answer + b
        question = f"{a} - {b}"
        difficulty = 1300
        
    elif difficulty_target < 1450:
        # Three numbers with parentheses: (a + b) - c or a + (b - c)
        answer = random.randint(5, 40)
        if random.choice([True, False]):
            b = random.randint(10, 30)
            c = random.randint(5, 15)
            a = answer + c - b
            question = f"({a} + {b}) - {c}"
        else:
            c = random.randint(5, 20)
            b = answer + c
            a = random.randint(10, 50)
            question = f"{a} + ({b} - {c})"
        difficulty = 1400
        
    elif difficulty_target < 1550:
        # Four numbers: a + b - c + d requiring multiple steps
        answer = random.randint(10, 60)
        b = random.randint(5, 25)
        c = random.randint(5, 20)
        d = random.randint(5, 15)
        a = answer - b + c - d
        question = f"{a} + {b} - {c} + {d}"
        difficulty = 1500
        
    else:
        # Complex with nested operations: a - (b + c) + (d - e)
        answer = random.randint(5, 50)
        b = random.randint(5, 20)
        c = random.randint(5, 15)
        d = random.randint(10, 30)
        e = random.randint(5, 15)
        a = answer + b + c - d + e
        question = f"{a} - ({b} + {c}) + ({d} - {e})"
        difficulty = 1600
    
    question_latex = question
    answer_key = str(answer)
    
    problem = {
        "question_latex": question_latex,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "arithmetic",
        "subtopic": "addition_subtraction",
        "grading_mode": "equivalent"
    }
    
    return problem

problem = generate_problem()
print(json.dumps(problem))