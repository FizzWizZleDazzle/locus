"""
arithmetic - multiplication_division (easy)
Generated: 2026-02-11T21:22:46.471551
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'single_digit_mult',
        'single_digit_div',
        'two_digit_mult',
        'two_digit_div',
        'simple_mult_chain',
        'division_with_remainder'
    ])
    
    if problem_type == 'single_digit_mult':
        a = random.randint(2, 9)
        b = random.randint(2, 9)
        answer = a * b
        question = f"{a} \\times {b}"
        elo = random.randint(1000, 1100)
        
    elif problem_type == 'single_digit_div':
        answer = random.randint(2, 9)
        divisor = random.randint(2, 9)
        dividend = answer * divisor
        question = f"{dividend} \\div {divisor}"
        elo = random.randint(1050, 1150)
        
    elif problem_type == 'two_digit_mult':
        a = random.randint(10, 20)
        b = random.randint(2, 9)
        answer = a * b
        question = f"{a} \\times {b}"
        elo = random.randint(1100, 1200)
        
    elif problem_type == 'two_digit_div':
        answer = random.randint(5, 15)
        divisor = random.randint(4, 9)
        dividend = answer * divisor
        question = f"{dividend} \\div {divisor}"
        elo = random.randint(1150, 1250)
        
    elif problem_type == 'simple_mult_chain':
        a = random.randint(2, 5)
        b = random.randint(2, 5)
        c = random.randint(2, 4)
        answer = a * b * c
        question = f"{a} \\times {b} \\times {c}"
        elo = random.randint(1200, 1300)
        
    elif problem_type == 'division_with_remainder':
        quotient = random.randint(5, 12)
        divisor = random.randint(3, 8)
        remainder = random.randint(1, divisor - 1)
        dividend = quotient * divisor + remainder
        question = f"\\text{{What is the quotient when }} {dividend} \\text{{ is divided by }} {divisor}\\text{{?}}"
        answer = quotient
        elo = random.randint(1250, 1300)
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "arithmetic",
        "subtopic": "multiplication_division",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_problem()
print(json.dumps(problem))