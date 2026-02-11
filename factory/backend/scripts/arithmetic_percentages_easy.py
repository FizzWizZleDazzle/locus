"""
arithmetic - percentages (easy)
Generated: 2026-02-11T21:25:18.481623
"""

import random
import json
from sympy import symbols, sympify, nsimplify

def generate_percentage_problem():
    problem_type = random.choice([
        'find_percentage_of_number',
        'what_percent_is',
        'find_whole_from_part',
        'simple_increase_decrease'
    ])
    
    if problem_type == 'find_percentage_of_number':
        # Reverse engineer: pick a clean answer first
        answer = random.choice([5, 10, 12, 15, 20, 25, 30, 40, 50, 60, 75, 80, 100])
        
        # Choose a simple percentage
        percent = random.choice([10, 20, 25, 50, 75])
        
        # Calculate the whole number
        whole = (answer * 100) // percent
        
        question = f"What is ${percent}\\%$ of ${whole}$?"
        difficulty = 1100
        
    elif problem_type == 'what_percent_is':
        # Pick clean numbers
        whole = random.choice([20, 25, 40, 50, 80, 100, 200])
        percent = random.choice([10, 20, 25, 40, 50, 75, 80])
        part = (whole * percent) // 100
        
        question = f"${part}$ is what percent of ${whole}$?"
        answer = percent
        difficulty = 1250
        
    elif problem_type == 'find_whole_from_part':
        # Pick the whole number first
        whole = random.choice([20, 40, 50, 60, 80, 100, 120, 200])
        percent = random.choice([10, 20, 25, 50])
        part = (whole * percent) // 100
        
        question = f"${part}$ is ${percent}\\%$ of what number?"
        answer = whole
        difficulty = 1300
        
    else:  # simple_increase_decrease
        original = random.choice([20, 40, 50, 60, 80, 100, 200])
        percent = random.choice([10, 20, 25, 50])
        
        if random.choice([True, False]):
            # Increase
            increase = (original * percent) // 100
            answer = original + increase
            question = f"Increase ${original}$ by ${percent}\\%$."
        else:
            # Decrease
            decrease = (original * percent) // 100
            answer = original - decrease
            question = f"Decrease ${original}$ by ${percent}\\%$."
        
        difficulty = 1200
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "arithmetic",
        "subtopic": "percentages",
        "grading_mode": "equivalent"
    }

problem = generate_percentage_problem()
print(json.dumps(problem))