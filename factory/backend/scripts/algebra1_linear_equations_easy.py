"""
algebra1 - linear_equations (easy)
Generated: 2026-02-11T21:26:54.274500
"""

import random
import json
from sympy import symbols, Eq, solve, latex

def generate_problem():
    x = symbols('x')
    
    problem_type = random.choice(['one_step_multiply', 'one_step_divide', 'one_step_add', 'one_step_subtract', 'two_step'])
    
    if problem_type == 'one_step_multiply':
        answer = random.randint(-10, 10)
        if answer == 0:
            answer = 1
        coef = random.choice([2, 3, 4, 5, 6, 7, 8, 9, 10])
        result = coef * answer
        question = f"{coef}x = {result}"
        difficulty = random.randint(1000, 1150)
        
    elif problem_type == 'one_step_divide':
        answer = random.randint(-10, 10)
        if answer == 0:
            answer = 1
        coef = random.choice([2, 3, 4, 5, 6, 7, 8, 9, 10])
        result = coef * answer
        question = f"\\frac{{x}}{{{coef}}} = {answer}"
        difficulty = random.randint(1050, 1200)
        
    elif problem_type == 'one_step_add':
        answer = random.randint(-10, 10)
        constant = random.randint(-15, 15)
        if constant == 0:
            constant = 1
        result = answer + constant
        question = f"x + {constant} = {result}" if constant >= 0 else f"x - {-constant} = {result}"
        difficulty = random.randint(1000, 1100)
        
    elif problem_type == 'one_step_subtract':
        answer = random.randint(-10, 10)
        constant = random.randint(-15, 15)
        if constant == 0:
            constant = 1
        result = answer - constant
        question = f"x - {constant} = {result}" if constant >= 0 else f"x + {-constant} = {result}"
        difficulty = random.randint(1000, 1100)
        
    else:  # two_step
        answer = random.randint(-8, 8)
        if answer == 0:
            answer = 1
        coef = random.choice([2, 3, 4, 5])
        constant = random.randint(-10, 10)
        if constant == 0:
            constant = 1
        result = coef * answer + constant
        if constant >= 0:
            question = f"{coef}x + {constant} = {result}"
        else:
            question = f"{coef}x - {-constant} = {result}"
        difficulty = random.randint(1200, 1300)
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "linear_equations",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))