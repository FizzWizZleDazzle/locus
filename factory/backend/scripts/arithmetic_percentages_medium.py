"""
arithmetic - percentages (medium)
Generated: 2026-02-11T21:25:30.900121
"""

import sympy as sp
import random
import json

def generate_percentage_problem():
    problem_type = random.choice(['find_percentage', 'find_whole', 'find_percent_value', 'percent_change', 'successive_percentages'])
    
    if problem_type == 'find_percentage':
        whole = random.choice([50, 80, 120, 150, 200, 250, 300, 400, 500])
        percent = random.choice([10, 15, 20, 25, 30, 40, 50, 60, 75, 80])
        answer = sp.Rational(percent * whole, 100)
        question = f"What is ${percent}\\%$ of ${whole}$?"
        difficulty = 1300
        
    elif problem_type == 'find_whole':
        answer = random.choice([40, 50, 60, 80, 100, 120, 150, 200])
        percent = random.choice([20, 25, 40, 50, 60, 75, 80])
        part = sp.Rational(percent * answer, 100)
        question = f"${part}$ is ${percent}\\%$ of what number?"
        difficulty = 1400
        
    elif problem_type == 'find_percent_value':
        whole = random.choice([80, 100, 120, 150, 200, 250, 300, 400])
        part = random.choice([i for i in range(10, whole, 5) if i < whole])
        answer = sp.Rational(part * 100, whole)
        question = f"${part}$ is what percent of ${whole}$?"
        difficulty = 1350
        
    elif problem_type == 'percent_change':
        original = random.choice([50, 80, 100, 120, 150, 200, 250])
        change_percent = random.choice([10, 15, 20, 25, 30, 40, 50])
        direction = random.choice(['increase', 'decrease'])
        
        if direction == 'increase':
            answer = sp.Rational(original * (100 + change_percent), 100)
            question = f"A quantity increases from ${original}$ by ${change_percent}\\%$. What is the new value?"
        else:
            answer = sp.Rational(original * (100 - change_percent), 100)
            question = f"A quantity decreases from ${original}$ by ${change_percent}\\%$. What is the new value?"
        difficulty = 1450
        
    elif problem_type == 'successive_percentages':
        original = random.choice([100, 200, 500, 1000])
        percent1 = random.choice([10, 20, 25, 50])
        percent2 = random.choice([10, 20, 25, 50])
        
        after_first = sp.Rational(original * (100 + percent1), 100)
        answer = sp.Rational(after_first * (100 + percent2), 100)
        question = f"A value of $${original}$ increases by ${percent1}\\%$ and then increases by another ${percent2}\\%$. What is the final value?"
        difficulty = 1550
    
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