"""
algebra1 - factoring (medium)
Generated: 2026-02-11T21:37:46.995360
"""

import random
import json
from sympy import symbols, expand, factor, latex

def generate_factoring_problem():
    x = symbols('x')
    difficulty_type = random.choice(['simple_gcf', 'difference_of_squares', 'trinomial_a1', 'trinomial_a_not_1', 'grouping'])
    
    if difficulty_type == 'simple_gcf':
        # GCF factoring: 1200-1300 ELO
        gcf = random.randint(2, 8)
        coef1 = random.randint(2, 6)
        coef2 = random.randint(2, 6)
        exp1 = random.randint(2, 4)
        exp2 = random.randint(1, exp1 - 1)
        
        problem = gcf * coef1 * x**exp1 + gcf * coef2 * x**exp2
        answer = factor(problem)
        difficulty = 1250
        
    elif difficulty_type == 'difference_of_squares':
        # a^2 - b^2 = (a+b)(a-b): 1300-1400 ELO
        a = random.randint(2, 8)
        b = random.randint(2, 8)
        
        problem = (a*x)**2 - b**2
        answer = factor(problem)
        difficulty = 1350
        
    elif difficulty_type == 'trinomial_a1':
        # x^2 + bx + c where a=1: 1400-1500 ELO
        r1 = random.randint(-8, 8)
        r2 = random.randint(-8, 8)
        while r1 == 0 or r2 == 0:
            r1 = random.randint(-8, 8)
            r2 = random.randint(-8, 8)
        
        problem = expand((x - r1) * (x - r2))
        answer = factor(problem)
        difficulty = 1450
        
    elif difficulty_type == 'trinomial_a_not_1':
        # ax^2 + bx + c where a!=1: 1500-1600 ELO
        a = random.randint(2, 5)
        r1 = random.randint(-6, 6)
        r2 = random.randint(-6, 6)
        while r1 == 0 or r2 == 0:
            r1 = random.randint(-6, 6)
            r2 = random.randint(-6, 6)
        
        problem = expand(a * (x - r1) * (x - r2))
        answer = factor(problem)
        difficulty = 1550
        
    else:  # grouping
        # Factor by grouping: 1400-1500 ELO
        a = random.randint(2, 5)
        b = random.randint(2, 5)
        c = random.randint(2, 5)
        d = random.randint(2, 5)
        
        problem = expand((a*x + b) * (c*x + d))
        answer = factor(problem)
        difficulty = 1480
    
    question_latex = f"\\text{{Factor: }} {latex(problem)}"
    answer_key = str(answer)
    
    return {
        "question_latex": question_latex,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "factoring",
        "grading_mode": "equivalent"
    }

problem = generate_factoring_problem()
print(json.dumps(problem))