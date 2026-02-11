"""
arithmetic - fractions (easy)
Generated: 2026-02-11T21:23:46.656187
"""

import random
import json
from sympy import *

def generate_fraction_problem():
    problem_type = random.choice([
        'simplify_single',
        'add_same_denom',
        'add_diff_denom_easy',
        'subtract_same_denom',
        'multiply_simple',
        'compare_fractions'
    ])
    
    if problem_type == 'simplify_single':
        # Pick a simple answer, then multiply to create reducible fraction
        answer_num = random.randint(1, 5)
        answer_den = random.randint(2, 6)
        while gcd(answer_num, answer_den) != 1:
            answer_den = random.randint(2, 6)
        
        multiplier = random.randint(2, 4)
        num = answer_num * multiplier
        den = answer_den * multiplier
        
        question = f"\\text{{Simplify: }} \\frac{{{num}}}{{{den}}}"
        answer = Rational(answer_num, answer_den)
        difficulty = 1050
        
    elif problem_type == 'add_same_denom':
        # Pick answer first
        answer_num = random.randint(2, 8)
        den = random.randint(3, 10)
        
        # Split into two addends
        num1 = random.randint(1, answer_num - 1)
        num2 = answer_num - num1
        
        question = f"\\frac{{{num1}}}{{{den}}} + \\frac{{{num2}}}{{{den}}}"
        answer = Rational(answer_num, den)
        difficulty = 1100
        
    elif problem_type == 'add_diff_denom_easy':
        # Use denominators where one divides the other
        small_den = random.randint(2, 5)
        multiplier = random.randint(2, 3)
        large_den = small_den * multiplier
        
        # Pick answer
        answer_num = random.randint(3, 10)
        while gcd(answer_num, large_den) > 1 and Rational(answer_num, large_den) > 1:
            answer_num = random.randint(3, 10)
        
        # Work backward
        num2 = random.randint(1, 3)
        num1_over_large = answer_num - num2 * multiplier
        num1 = num1_over_large // multiplier if num1_over_large % multiplier == 0 else random.randint(1, 3)
        
        # Recalculate answer based on actual values
        answer = Rational(num1, small_den) + Rational(num2, large_den)
        
        question = f"\\frac{{{num1}}}{{{small_den}}} + \\frac{{{num2}}}{{{large_den}}}"
        difficulty = 1250
        
    elif problem_type == 'subtract_same_denom':
        # Pick answer first
        den = random.randint(4, 12)
        answer_num = random.randint(1, 6)
        
        # Create subtraction
        num1 = answer_num + random.randint(1, 5)
        num2 = num1 - answer_num
        
        question = f"\\frac{{{num1}}}{{{den}}} - \\frac{{{num2}}}{{{den}}}"
        answer = Rational(answer_num, den)
        difficulty = 1150
        
    elif problem_type == 'multiply_simple':
        # Pick simple answer
        answer_num = random.randint(1, 4)
        answer_den = random.randint(2, 5)
        while gcd(answer_num, answer_den) != 1:
            answer_den = random.randint(2, 5)
        
        # Create factors
        num1 = random.randint(1, 4)
        den1 = random.randint(2, 5)
        
        num2 = answer_num * den1
        den2 = answer_den * num1
        
        question = f"\\frac{{{num1}}}{{{den1}}} \\times \\frac{{{num2}}}{{{den2}}}"
        answer = Rational(answer_num, answer_den)
        difficulty = 1200
        
    else:  # compare_fractions
        den = random.randint(5, 12)
        num1 = random.randint(1, den - 1)
        num2 = random.randint(1, den - 1)
        while num1 == num2:
            num2 = random.randint(1, den - 1)
        
        if num1 < num2:
            symbol = "<"
            answer_val = "True"
        else:
            symbol = ">"
            answer_val = "True"
        
        question = f"\\text{{Is }} \\frac{{{num1}}}{{{den}}} {symbol} \\frac{{{num2}}}{{{den}}}\\text{{? (True/False)}}"
        answer = sympify(answer_val)
        difficulty = 1080
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "arithmetic",
        "subtopic": "fractions",
        "grading_mode": "equivalent"
    }

problem = generate_fraction_problem()
print(json.dumps(problem))