"""
algebra2 - logarithms (medium)
Generated: 2026-02-11T21:49:12.014497
"""

import random
import json
from sympy import symbols, log, expand, simplify, latex, sympify

def generate_logarithm_problem():
    x = symbols('x')
    problem_type = random.randint(1, 6)
    
    if problem_type == 1:
        # Two-step: log_b(x) = c, solve for x (1200-1300)
        base = random.choice([2, 3, 5, 10])
        exponent = random.randint(2, 4)
        answer = base ** exponent
        question = f"\\text{{Solve for }} x: \\log_{{{base}}}(x) = {exponent}"
        difficulty = 1250
        
    elif problem_type == 2:
        # Two-step: log(a) + log(b) = ? using product rule (1300-1400)
        a = random.choice([2, 3, 4, 5])
        b = random.choice([2, 3, 4, 5, 8])
        answer = a * b
        base = random.choice([10, 2])
        if base == 10:
            question = f"\\text{{Simplify: }} \\log({a}) + \\log({b})"
        else:
            question = f"\\text{{Simplify: }} \\log_{{{base}}}({a}) + \\log_{{{base}}}({b})"
        answer = f"log({answer}, {base})"
        difficulty = 1350
        
    elif problem_type == 3:
        # Multi-step: log_b(x) + log_b(y) = c, solve for product (1400-1500)
        base = random.choice([2, 3, 5])
        exponent = random.randint(3, 5)
        product = base ** exponent
        factor1 = random.choice([2, 3, 4, 5])
        if product % factor1 == 0:
            factor2 = product // factor1
        else:
            factor1 = random.choice([d for d in range(2, product) if product % d == 0])
            factor2 = product // factor1
        question = f"\\text{{If }} \\log_{{{base}}}(x) + \\log_{{{base}}}({factor2}) = {exponent}, \\text{{ find }} x"
        answer = factor1
        difficulty = 1450
        
    elif problem_type == 4:
        # Multi-step: log(a) - log(b) using quotient rule (1350-1450)
        b = random.choice([2, 3, 4, 5])
        multiplier = random.choice([2, 3, 4, 5, 6])
        a = b * multiplier
        base = random.choice([10, 2])
        if base == 10:
            question = f"\\text{{Simplify: }} \\log({a}) - \\log({b})"
        else:
            question = f"\\text{{Simplify: }} \\log_{{{base}}}({a}) - \\log_{{{base}}}({b})"
        answer = f"log({multiplier}, {base})"
        difficulty = 1380
        
    elif problem_type == 5:
        # Multi-step: Solve log_b(x^2) = c (1450-1550)
        base = random.choice([2, 3, 5, 10])
        exponent = random.choice([2, 4, 6])
        x_value = base ** (exponent // 2)
        question = f"\\text{{Solve for }} x \\text{{ (positive value): }} \\log_{{{base}}}(x^2) = {exponent}"
        answer = x_value
        difficulty = 1500
        
    else:
        # Multi-step: 2*log_b(x) = c, solve for x (1400-1500)
        base = random.choice([2, 3, 5, 10])
        coefficient = 2
        result_exponent = random.choice([2, 4, 6])
        exponent = result_exponent // coefficient
        x_value = base ** exponent
        question = f"\\text{{Solve for }} x: {coefficient}\\log_{{{base}}}(x) = {result_exponent}"
        answer = x_value
        difficulty = 1480
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "logarithms",
        "grading_mode": "equivalent"
    }

problem = generate_logarithm_problem()
print(json.dumps(problem))