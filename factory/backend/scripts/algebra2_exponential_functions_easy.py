"""
algebra2 - exponential_functions (easy)
Generated: 2026-02-11T21:48:03.338522
"""

import sympy as sp
import random
import json

def generate_problem():
    x = sp.Symbol('x')
    problem_type = random.choice([
        'evaluate_simple',
        'evaluate_negative',
        'solve_simple',
        'solve_two_step',
        'identify_base'
    ])
    
    if problem_type == 'evaluate_simple':
        base = random.choice([2, 3, 4, 5, 10])
        exponent = random.choice([2, 3, 4])
        answer = base ** exponent
        question = f"\\text{{Evaluate: }} {base}^{{{exponent}}}"
        answer_key = str(answer)
        difficulty = 1050
        
    elif problem_type == 'evaluate_negative':
        base = random.choice([2, 3, 4, 5])
        exponent = random.choice([-1, -2])
        answer = sp.Rational(1, base ** abs(exponent))
        question = f"\\text{{Evaluate: }} {base}^{{{exponent}}}"
        answer_key = str(answer)
        difficulty = 1150
        
    elif problem_type == 'solve_simple':
        base = random.choice([2, 3, 4, 5])
        result_exponent = random.choice([2, 3, 4])
        result = base ** result_exponent
        answer = result_exponent
        question = f"\\text{{Solve for }} x: {base}^x = {result}"
        answer_key = str(answer)
        difficulty = 1200
        
    elif problem_type == 'solve_two_step':
        base = random.choice([2, 3, 5])
        coefficient = random.choice([2, 3, 4, 5])
        result_exponent = random.choice([2, 3])
        base_power = base ** result_exponent
        result = coefficient * base_power
        answer = result_exponent
        question = f"\\text{{Solve for }} x: {coefficient} \\cdot {base}^x = {result}"
        answer_key = str(answer)
        difficulty = 1250
        
    elif problem_type == 'identify_base':
        base = random.choice([2, 3, 4, 5, 10])
        exponent = random.choice([2, 3])
        result = base ** exponent
        answer = base
        question = f"\\text{{If }} b^{{{exponent}}} = {result}, \\text{{ find }} b \\text{{ (where }} b > 0\\text{{)}}"
        answer_key = str(answer)
        difficulty = 1150
    
    return {
        "question_latex": question,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "exponential_functions",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))