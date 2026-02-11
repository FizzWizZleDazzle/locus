"""
algebra2 - radical_expressions (easy)
Generated: 2026-02-11T21:47:22.533809
"""

import sympy as sp
import random
import json

def generate_radical_problem():
    x = sp.Symbol('x')
    problem_type = random.choice([
        'simplify_sqrt',
        'simplify_sqrt_fraction',
        'multiply_radicals',
        'add_like_radicals',
        'rationalize_denominator_simple'
    ])
    
    if problem_type == 'simplify_sqrt':
        perfect_square = random.choice([4, 9, 16, 25, 36, 49])
        other_factor = random.choice([2, 3, 5, 6, 7])
        radicand = perfect_square * other_factor
        answer = sp.sqrt(perfect_square) * sp.sqrt(other_factor)
        question = f"\\sqrt{{{radicand}}}"
        difficulty = 1050
        
    elif problem_type == 'simplify_sqrt_fraction':
        numerator_perfect = random.choice([4, 9, 16, 25])
        denominator_perfect = random.choice([4, 9, 16, 25])
        while numerator_perfect == denominator_perfect:
            denominator_perfect = random.choice([4, 9, 16, 25])
        answer = sp.sqrt(numerator_perfect) / sp.sqrt(denominator_perfect)
        question = f"\\sqrt{{\\frac{{{numerator_perfect}}}{{{denominator_perfect}}}}}"
        difficulty = 1100
        
    elif problem_type == 'multiply_radicals':
        a = random.randint(2, 5)
        b = random.randint(2, 5)
        answer = sp.sqrt(a * b)
        question = f"\\sqrt{{{a}}} \\cdot \\sqrt{{{b}}}"
        difficulty = 1150
        
    elif problem_type == 'add_like_radicals':
        coef1 = random.randint(2, 5)
        coef2 = random.randint(2, 5)
        radical_value = random.choice([2, 3, 5, 6, 7])
        answer = (coef1 + coef2) * sp.sqrt(radical_value)
        question = f"{coef1}\\sqrt{{{radical_value}}} + {coef2}\\sqrt{{{radical_value}}}"
        difficulty = 1200
        
    else:  # rationalize_denominator_simple
        numerator = random.randint(1, 5)
        denominator_radical = random.choice([2, 3, 5])
        answer = sp.Rational(numerator, 1) * sp.sqrt(denominator_radical) / denominator_radical
        question = f"\\frac{{{numerator}}}{{\\sqrt{{{denominator_radical}}}}}"
        difficulty = 1250
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "radical_expressions",
        "grading_mode": "equivalent"
    }

problem = generate_radical_problem()
print(json.dumps(problem))