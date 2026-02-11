"""
arithmetic - fractions (medium)
Generated: 2026-02-11T21:24:04.545871
"""

import random
import json
from sympy import *

def generate_fraction_problem():
    problem_type = random.choice(['add_sub', 'multiply', 'divide', 'mixed_operations', 'simplify'])
    
    if problem_type == 'add_sub':
        # Addition or subtraction of fractions
        # Pick clean answer first
        ans_num = random.randint(1, 10)
        ans_den = random.randint(2, 12)
        
        # Create two fractions that add/subtract to this answer
        den1 = random.randint(2, 8)
        den2 = random.randint(2, 8)
        
        # Find LCM for common denominator
        lcm_val = lcm(den1, den2)
        
        operation = random.choice(['+', '-'])
        
        if operation == '+':
            # a/den1 + b/den2 = ans_num/ans_den
            num1 = random.randint(1, 10)
            # Calculate num2 to give desired result
            target_num = ans_num * lcm_val // ans_den
            num2_calc = target_num - num1 * (lcm_val // den1)
            num2 = num2_calc * den2 // lcm_val
            
            if num2 <= 0 or num2 > 20:
                num2 = random.randint(1, 10)
                
            result = Rational(num1, den1) + Rational(num2, den2)
            question = f"\\frac{{{num1}}}{{{den1}}} + \\frac{{{num2}}}{{{den2}}}"
            difficulty = 1300 if den1 == den2 else 1400
            
        else:  # subtraction
            num1 = random.randint(3, 15)
            num2 = random.randint(1, num1 - 1)
            result = Rational(num1, den1) - Rational(num2, den2)
            question = f"\\frac{{{num1}}}{{{den1}}} - \\frac{{{num2}}}{{{den2}}}"
            difficulty = 1350 if den1 == den2 else 1450
            
    elif problem_type == 'multiply':
        # Multiplication of fractions
        num1 = random.randint(1, 8)
        den1 = random.randint(2, 10)
        num2 = random.randint(1, 8)
        den2 = random.randint(2, 10)
        
        result = Rational(num1, den1) * Rational(num2, den2)
        question = f"\\frac{{{num1}}}{{{den1}}} \\times \\frac{{{num2}}}{{{den2}}}"
        difficulty = 1300
        
    elif problem_type == 'divide':
        # Division of fractions
        num1 = random.randint(1, 8)
        den1 = random.randint(2, 10)
        num2 = random.randint(1, 8)
        den2 = random.randint(2, 10)
        
        result = Rational(num1, den1) / Rational(num2, den2)
        question = f"\\frac{{{num1}}}{{{den1}}} \\div \\frac{{{num2}}}{{{den2}}}"
        difficulty = 1400
        
    elif problem_type == 'mixed_operations':
        # Multiple operations
        num1 = random.randint(1, 6)
        den1 = random.randint(2, 8)
        num2 = random.randint(1, 6)
        den2 = random.randint(2, 8)
        num3 = random.randint(1, 6)
        den3 = random.randint(2, 8)
        
        ops = random.choice([
            ('multiply_add', f"\\frac{{{num1}}}{{{den1}}} \\times \\frac{{{num2}}}{{{den2}}} + \\frac{{{num3}}}{{{den3}}}"),
            ('add_multiply', f"\\left(\\frac{{{num1}}}{{{den1}}} + \\frac{{{num2}}}{{{den2}}}\\right) \\times \\frac{{{num3}}}{{{den3}}}"),
        ])
        
        if ops[0] == 'multiply_add':
            result = Rational(num1, den1) * Rational(num2, den2) + Rational(num3, den3)
            difficulty = 1550
        else:
            result = (Rational(num1, den1) + Rational(num2, den2)) * Rational(num3, den3)
            difficulty = 1500
            
        question = ops[1]
        
    else:  # simplify
        # Simplify a fraction
        factor = random.randint(2, 6)
        simple_num = random.randint(1, 8)
        simple_den = random.randint(2, 10)
        
        num = simple_num * factor
        den = simple_den * factor
        
        result = Rational(num, den)
        question = f"\\text{{Simplify: }} \\frac{{{num}}}{{{den}}}"
        difficulty = 1300
    
    problem = {
        "question_latex": question,
        "answer_key": str(result),
        "difficulty": difficulty,
        "main_topic": "arithmetic",
        "subtopic": "fractions",
        "grading_mode": "equivalent"
    }
    
    return problem

problem = generate_fraction_problem()
print(json.dumps(problem))