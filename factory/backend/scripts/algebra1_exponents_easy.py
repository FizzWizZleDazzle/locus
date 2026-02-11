"""
algebra1 - exponents (easy)
Generated: 2026-02-11T21:35:59.002619
"""

import random
import json
from sympy import symbols, simplify, latex, Integer, Rational, Pow

def generate_problem():
    problem_type = random.choice([
        'evaluate_simple',
        'evaluate_zero_exp',
        'evaluate_negative_base',
        'multiply_same_base',
        'divide_same_base',
        'power_of_power',
        'negative_exponent_simple'
    ])
    
    if problem_type == 'evaluate_simple':
        base = random.choice([2, 3, 4, 5])
        exp = random.choice([2, 3])
        answer = base ** exp
        question = f"{base}^{{{exp}}}"
        difficulty = 1000
        
    elif problem_type == 'evaluate_zero_exp':
        base = random.randint(2, 9)
        answer = 1
        question = f"{base}^{{0}}"
        difficulty = 1050
        
    elif problem_type == 'evaluate_negative_base':
        base = random.choice([-2, -3])
        exp = random.choice([2, 3])
        answer = base ** exp
        question = f"({base})^{{{exp}}}"
        difficulty = 1100
        
    elif problem_type == 'multiply_same_base':
        base = random.choice([2, 3, 5, 'x', 'y'])
        exp1 = random.randint(2, 4)
        exp2 = random.randint(2, 4)
        answer_exp = exp1 + exp2
        
        if isinstance(base, int):
            question = f"{base}^{{{exp1}}} \\cdot {base}^{{{exp2}}}"
            answer = base ** answer_exp
            difficulty = 1150
        else:
            question = f"{base}^{{{exp1}}} \\cdot {base}^{{{exp2}}}"
            answer = f"{base}**{answer_exp}"
            difficulty = 1200
            
    elif problem_type == 'divide_same_base':
        base = random.choice([2, 3, 5, 'x', 'y'])
        exp2 = random.randint(2, 4)
        exp1 = exp2 + random.randint(1, 3)
        answer_exp = exp1 - exp2
        
        if isinstance(base, int):
            question = f"\\frac{{{base}^{{{exp1}}}}}{{{base}^{{{exp2}}}}}"
            answer = base ** answer_exp
            difficulty = 1200
        else:
            question = f"\\frac{{{base}^{{{exp1}}}}}{{{base}^{{{exp2}}}}}"
            answer = f"{base}**{answer_exp}"
            difficulty = 1250
            
    elif problem_type == 'power_of_power':
        base = random.choice([2, 3, 'x', 'y'])
        exp1 = random.randint(2, 3)
        exp2 = random.randint(2, 3)
        answer_exp = exp1 * exp2
        
        if isinstance(base, int):
            question = f"\\left({base}^{{{exp1}}}\\right)^{{{exp2}}}"
            answer = base ** answer_exp
            difficulty = 1250
        else:
            question = f"\\left({base}^{{{exp1}}}\\right)^{{{exp2}}}"
            answer = f"{base}**{answer_exp}"
            difficulty = 1280
            
    elif problem_type == 'negative_exponent_simple':
        base = random.choice([2, 3, 4, 5])
        exp = random.choice([1, 2])
        answer_num = 1
        answer_den = base ** exp
        question = f"{base}^{{-{exp}}}"
        answer = f"1/{answer_den}"
        difficulty = 1300
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "exponents",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))