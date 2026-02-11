"""
arithmetic - decimals (medium)
Generated: 2026-02-11T21:24:48.997287
"""

import random
import json
from sympy import *

def generate_decimal_problem():
    problem_type = random.choice([
        'addition_subtraction',
        'multiplication',
        'division',
        'mixed_operations',
        'word_problem'
    ])
    
    if problem_type == 'addition_subtraction':
        answer = round(random.uniform(5, 50), 2)
        term1 = round(random.uniform(1, answer - 1), 2)
        term2 = round(answer - term1, 2)
        
        if random.choice([True, False]):
            question = f"{term1} + {term2}"
            elo = 1300
        else:
            total = round(term1 + term2, 2)
            question = f"{total} - {term1}"
            answer = term2
            elo = 1350
            
    elif problem_type == 'multiplication':
        answer = round(random.uniform(10, 100), 2)
        factor1 = round(random.uniform(1.5, 5), 1)
        factor2 = round(answer / factor1, 2)
        answer = round(factor1 * factor2, 2)
        
        question = f"{factor1} \\times {factor2}"
        elo = 1400
        
    elif problem_type == 'division':
        divisor = round(random.uniform(2, 8), 1)
        answer = round(random.uniform(5, 25), 2)
        dividend = round(divisor * answer, 2)
        
        question = f"{dividend} \\div {divisor}"
        elo = 1450
        
    elif problem_type == 'mixed_operations':
        answer = round(random.uniform(10, 50), 2)
        a = round(random.uniform(2, 10), 1)
        b = round(random.uniform(2, 10), 1)
        c = round(answer - (a * b), 2)
        answer = round(a * b + c, 2)
        
        question = f"{a} \\times {b} + {c}"
        elo = 1500
        
    else:  # word_problem
        prices = [round(random.uniform(5, 20), 2) for _ in range(2)]
        quantities = [round(random.uniform(1.5, 5), 1) for _ in range(2)]
        answer = round(prices[0] * quantities[0] + prices[1] * quantities[1], 2)
        
        items = ['apples', 'oranges', 'bananas', 'pears', 'grapes']
        selected_items = random.sample(items, 2)
        
        question = f"\\text{{If {selected_items[0]} cost }}\\${prices[0]}\\text{{ per kg and {selected_items[1]} cost }}\\${prices[1]}\\text{{ per kg, }}"
        question += f"\\text{{what is the total cost of }}{quantities[0]}\\text{{ kg of {selected_items[0]} and }}{quantities[1]}\\text{{ kg of {selected_items[1]}?}}"
        elo = 1550
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "arithmetic",
        "subtopic": "decimals",
        "grading_mode": "equivalent"
    }

problem = generate_decimal_problem()
print(json.dumps(problem))