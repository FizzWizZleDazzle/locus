"""
algebra1 - inequalities (easy)
Generated: 2026-02-11T21:27:42.064819
"""

import sympy as sp
import random
import json

x = sp.Symbol('x')

def generate_problem():
    problem_type = random.choice([
        'one_step_add_sub',
        'one_step_mult_div_positive',
        'two_step_simple',
        'one_step_mult_div_negative'
    ])
    
    if problem_type == 'one_step_add_sub':
        answer = random.randint(-10, 10)
        constant = random.randint(-15, 15)
        if random.choice([True, False]):
            question = f"x + {constant} < {answer + constant}"
            answer_key = f"x < {answer}"
        else:
            question = f"x - {constant} > {answer - constant}"
            answer_key = f"x > {answer}"
        difficulty = random.randint(1000, 1150)
        
    elif problem_type == 'one_step_mult_div_positive':
        answer = random.randint(-8, 8)
        coeff = random.choice([2, 3, 4, 5])
        if random.choice([True, False]):
            question = f"{coeff}x \\leq {coeff * answer}"
            answer_key = f"x <= {answer}"
        else:
            question = f"\\frac{{x}}{{{coeff}}} \\geq {answer}"
            answer_key = f"x >= {coeff * answer}"
        difficulty = random.randint(1050, 1200)
        
    elif problem_type == 'one_step_mult_div_negative':
        answer = random.randint(-8, 8)
        coeff = random.choice([-2, -3, -4, -5])
        if random.choice([True, False]):
            question = f"{coeff}x < {coeff * answer}"
            answer_key = f"x > {answer}"
        else:
            question = f"{coeff}x \\geq {coeff * answer}"
            answer_key = f"x <= {answer}"
        difficulty = random.randint(1150, 1300)
        
    else:
        answer = random.randint(-10, 10)
        coeff = random.choice([2, 3, 4, 5])
        constant = random.randint(-10, 10)
        rhs = coeff * answer + constant
        if random.choice([True, False]):
            question = f"{coeff}x + {constant} > {rhs}"
            answer_key = f"x > {answer}"
        else:
            question = f"{coeff}x - {constant} \\leq {rhs}"
            answer_key = f"x <= {answer}"
        difficulty = random.randint(1200, 1300)
    
    return {
        "question_latex": question,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "inequalities",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))