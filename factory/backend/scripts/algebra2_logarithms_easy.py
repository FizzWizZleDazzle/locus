"""
algebra2 - logarithms (easy)
Generated: 2026-02-11T21:48:54.376384
"""

import random
import json
from sympy import symbols, log, simplify, latex, sympify

def generate_logarithm_problem():
    problem_type = random.choice([
        'evaluate_basic',
        'evaluate_with_base',
        'solve_simple_log',
        'log_property_product',
        'log_property_power'
    ])
    
    if problem_type == 'evaluate_basic':
        base = random.choice([2, 3, 5, 10])
        exponent = random.randint(2, 4)
        answer = exponent
        value = base ** exponent
        
        if base == 10:
            question = f"\\log({value})"
        else:
            question = f"\\log_{{{base}}}({value})"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1050,
            "main_topic": "algebra2",
            "subtopic": "logarithms",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'evaluate_with_base':
        base = random.choice([2, 3, 4, 5])
        answer = random.randint(1, 3)
        value = base ** answer
        
        question = f"\\log_{{{base}}}({value})"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1100,
            "main_topic": "algebra2",
            "subtopic": "logarithms",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'solve_simple_log':
        x = symbols('x')
        base = random.choice([2, 3, 5, 10])
        answer = random.randint(2, 5)
        result_value = base ** answer
        
        if base == 10:
            question = f"\\text{{Solve for }} x: \\log(x) = {answer}"
        else:
            question = f"\\text{{Solve for }} x: \\log_{{{base}}}(x) = {answer}"
        
        return {
            "question_latex": question,
            "answer_key": str(result_value),
            "difficulty": 1200,
            "main_topic": "algebra2",
            "subtopic": "logarithms",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'log_property_product':
        base = random.choice([2, 3, 5])
        a = random.randint(2, 4)
        b = random.randint(2, 4)
        
        val_a = base ** a
        val_b = base ** b
        answer = a + b
        
        question = f"\\log_{{{base}}}({val_a}) + \\log_{{{base}}}({val_b})"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1250,
            "main_topic": "algebra2",
            "subtopic": "logarithms",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'log_property_power':
        base = random.choice([2, 3, 5])
        inner_exp = random.randint(2, 3)
        outer_coeff = random.randint(2, 3)
        
        value = base ** inner_exp
        answer = inner_exp * outer_coeff
        
        question = f"{outer_coeff} \\cdot \\log_{{{base}}}({value})"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1280,
            "main_topic": "algebra2",
            "subtopic": "logarithms",
            "grading_mode": "equivalent"
        }

problem = generate_logarithm_problem()
print(json.dumps(problem))