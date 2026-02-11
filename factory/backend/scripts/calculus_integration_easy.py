"""
calculus - integration (easy)
Generated: 2026-02-11T22:00:19.307885
"""

import sympy as sp
import random
import json

x = sp.Symbol('x')

def generate_problem():
    problem_type = random.choice([
        'power_rule_simple',
        'power_rule_coefficient',
        'constant',
        'sum_two_terms'
    ])
    
    if problem_type == 'power_rule_simple':
        n = random.choice([1, 2, 3, 4, 5])
        answer = x**(n+1) / (n+1)
        if random.choice([True, False]):
            answer += random.randint(-5, 5)
        integrand = sp.diff(answer, x)
        question = f"\\int {sp.latex(integrand)} \\, dx"
        elo = 1050
        
    elif problem_type == 'power_rule_coefficient':
        n = random.choice([1, 2, 3, 4])
        coeff = random.choice([2, 3, 4, 5, 6])
        answer = coeff * x**(n+1) / (n+1)
        if random.choice([True, False]):
            answer += random.randint(-5, 5)
        integrand = sp.diff(answer, x)
        question = f"\\int {sp.latex(integrand)} \\, dx"
        elo = 1100
        
    elif problem_type == 'constant':
        c = random.randint(2, 10)
        answer = c * x + random.randint(-5, 5)
        integrand = sp.diff(answer, x)
        question = f"\\int {sp.latex(integrand)} \\, dx"
        elo = 1000
        
    elif problem_type == 'sum_two_terms':
        n1 = random.choice([1, 2, 3])
        n2 = random.choice([1, 2, 3])
        while n1 == n2:
            n2 = random.choice([1, 2, 3])
        coeff1 = random.choice([1, 2, 3])
        coeff2 = random.choice([1, 2, 3])
        answer = coeff1 * x**(n1+1) / (n1+1) + coeff2 * x**(n2+1) / (n2+1)
        if random.choice([True, False]):
            answer += random.randint(-3, 3)
        integrand = sp.diff(answer, x)
        question = f"\\int \\left({sp.latex(integrand)}\\right) \\, dx"
        elo = 1250
    
    answer_str = sp.latex(answer) + " + C"
    
    return {
        "question_latex": question,
        "answer_key": answer_str,
        "difficulty": elo,
        "main_topic": "calculus",
        "subtopic": "integration",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))