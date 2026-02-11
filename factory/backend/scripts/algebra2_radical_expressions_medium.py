"""
algebra2 - radical_expressions (medium)
Generated: 2026-02-11T21:47:33.777756
"""

import random
import json
from sympy import symbols, sqrt, simplify, expand, latex, Rational

def generate_problem():
    x = symbols('x')
    problem_type = random.choice([
        'simplify_sqrt',
        'add_subtract_radicals',
        'multiply_radicals',
        'rationalize_denominator',
        'solve_radical_equation'
    ])
    
    if problem_type == 'simplify_sqrt':
        perfect_square = random.choice([4, 9, 16, 25, 36, 49])
        non_perfect = random.choice([2, 3, 5, 6, 7, 10, 11, 13])
        radicand = perfect_square * non_perfect
        
        question = f"\\sqrt{{{radicand}}}"
        answer = simplify(sqrt(radicand))
        difficulty = 1300
        
    elif problem_type == 'add_subtract_radicals':
        radical_base = random.choice([2, 3, 5, 6, 7])
        coef1 = random.randint(2, 7)
        coef2 = random.randint(2, 7)
        operation = random.choice(['+', '-'])
        
        question = f"{coef1}\\sqrt{{{radical_base}}} {operation} {coef2}\\sqrt{{{radical_base}}}"
        if operation == '+':
            answer = simplify((coef1 + coef2) * sqrt(radical_base))
        else:
            answer = simplify((coef1 - coef2) * sqrt(radical_base))
        difficulty = 1350
        
    elif problem_type == 'multiply_radicals':
        num1 = random.randint(2, 6)
        num2 = random.randint(2, 6)
        
        question = f"\\sqrt{{{num1}}} \\cdot \\sqrt{{{num2}}}"
        answer = simplify(sqrt(num1 * num2))
        difficulty = 1400
        
    elif problem_type == 'rationalize_denominator':
        numerator = random.randint(2, 8)
        denominator_radical = random.choice([2, 3, 5, 6, 7])
        
        question = f"\\frac{{{numerator}}}{{\\sqrt{{{denominator_radical}}}}}"
        answer = simplify(numerator * sqrt(denominator_radical) / denominator_radical)
        difficulty = 1500
        
    else:  # solve_radical_equation
        solution = random.randint(2, 8)
        constant = random.randint(1, 5)
        radicand_value = solution + constant
        
        question = f"\\sqrt{{x + {constant}}} = {solution}"
        answer = solution**2 - constant
        difficulty = 1550
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "radical_expressions",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))