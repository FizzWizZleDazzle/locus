"""
arithmetic - decimals (easy)
Generated: 2026-02-11T21:24:35.933941
"""

import random
import json
from sympy import sympify, nsimplify

def generate_decimal_problem():
    problem_type = random.choice(['addition', 'subtraction', 'multiplication', 'division'])
    
    if problem_type == 'addition':
        # Pick clean answer first (1-2 decimal places)
        answer = round(random.uniform(0.5, 20), random.choice([1, 2]))
        
        # Create two addends that sum to answer
        addend1 = round(random.uniform(0.1, answer - 0.1), random.choice([1, 2]))
        addend2 = round(answer - addend1, 2)
        
        question = f"{addend1} + {addend2}"
        difficulty = 1050  # Single-step addition
        
    elif problem_type == 'subtraction':
        # Pick clean answer first
        answer = round(random.uniform(0.5, 15), random.choice([1, 2]))
        
        # Create minuend larger than answer
        minuend = round(answer + random.uniform(1, 10), random.choice([1, 2]))
        subtrahend = round(minuend - answer, 2)
        
        question = f"{minuend} - {subtrahend}"
        difficulty = 1100  # Single-step subtraction
        
    elif problem_type == 'multiplication':
        # Pick clean answer and simple multipliers
        multiplier1 = round(random.uniform(0.1, 5), 1)
        multiplier2 = random.choice([2, 3, 4, 5, 10])
        answer = round(multiplier1 * multiplier2, 2)
        
        question = f"{multiplier1} \\times {multiplier2}"
        difficulty = 1150  # Single-step multiplication with whole number
        
    else:  # division
        # Pick clean divisor and quotient to get clean answer
        divisor = random.choice([2, 4, 5, 10])
        quotient = round(random.uniform(0.5, 10), 1)
        dividend = round(quotient * divisor, 2)
        answer = round(dividend / divisor, 2)
        
        question = f"{dividend} \\div {divisor}"
        difficulty = 1200  # Single-step division
    
    # Ensure answer is properly formatted
    answer_str = str(nsimplify(answer, rational=False))
    
    problem = {
        "question_latex": question,
        "answer_key": answer_str,
        "difficulty": difficulty,
        "main_topic": "arithmetic",
        "subtopic": "decimals",
        "grading_mode": "equivalent"
    }
    
    return problem

problem = generate_decimal_problem()
print(json.dumps(problem))