"""
algebra1 - exponents (medium)
Generated: 2026-02-11T21:36:09.176442
"""

import random
import json
from sympy import symbols, simplify, latex, sympify, Integer, Rational

def generate_exponent_problem():
    problem_type = random.choice([
        'simplify_product',
        'simplify_quotient',
        'power_of_power',
        'negative_exponent',
        'mixed_operations',
        'zero_exponent'
    ])
    
    x = symbols('x')
    
    if problem_type == 'simplify_product':
        exp1 = random.randint(2, 5)
        exp2 = random.randint(2, 5)
        answer = x**(exp1 + exp2)
        question = f"x^{{{exp1}}} \\cdot x^{{{exp2}}}"
        difficulty = 1300
        
    elif problem_type == 'simplify_quotient':
        exp1 = random.randint(5, 9)
        exp2 = random.randint(2, 4)
        answer = x**(exp1 - exp2)
        question = f"\\frac{{x^{{{exp1}}}}}{{x^{{{exp2}}}}}"
        difficulty = 1350
        
    elif problem_type == 'power_of_power':
        exp1 = random.randint(2, 4)
        exp2 = random.randint(2, 4)
        answer = x**(exp1 * exp2)
        question = f"\\left(x^{{{exp1}}}\\right)^{{{exp2}}}"
        difficulty = 1400
        
    elif problem_type == 'negative_exponent':
        exp = random.randint(2, 5)
        answer = x**(-exp)
        question = f"\\frac{{1}}{{x^{{{exp}}}}}"
        difficulty = 1380
        
    elif problem_type == 'zero_exponent':
        base = random.randint(2, 9)
        answer = Integer(1)
        question = f"{base}^{{0}}"
        difficulty = 1250
        
    elif problem_type == 'mixed_operations':
        exp1 = random.randint(2, 4)
        exp2 = random.randint(2, 3)
        exp3 = random.randint(1, 3)
        
        inner_exp = exp1 * exp2
        final_exp = inner_exp - exp3
        answer = x**final_exp
        question = f"\\frac{{\\left(x^{{{exp1}}}\\right)^{{{exp2}}}}}{{x^{{{exp3}}}}}"
        difficulty = 1550
    
    answer_str = str(answer)
    
    result = {
        "question_latex": question,
        "answer_key": answer_str,
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "exponents",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_exponent_problem()
print(json.dumps(problem))