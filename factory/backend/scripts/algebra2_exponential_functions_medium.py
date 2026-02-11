"""
algebra2 - exponential_functions (medium)
Generated: 2026-02-11T21:48:22.070402
"""

import random
import json
from sympy import symbols, exp, log, simplify, latex, sympify, Rational

def generate_exponential_problem():
    x = symbols('x')
    problem_type = random.choice([
        'solve_exponential_basic',
        'solve_exponential_same_base',
        'solve_exponential_logarithm',
        'evaluate_exponential',
        'growth_decay_word',
        'exponential_equation_quadratic_form'
    ])
    
    if problem_type == 'solve_exponential_basic':
        base = random.choice([2, 3, 4, 5])
        answer = random.randint(2, 5)
        result = base ** answer
        question = f"Solve for $x$: ${base}^x = {result}$"
        answer_key = str(answer)
        difficulty = 1300
        
    elif problem_type == 'solve_exponential_same_base':
        base = random.choice([2, 3, 5])
        answer = random.randint(2, 6)
        coef1 = random.randint(1, 3)
        const1 = random.randint(1, 4)
        rhs_exp = coef1 * answer + const1
        question = f"Solve for $x$: ${base}^{{{coef1}x + {const1}}} = {base}^{{{rhs_exp}}}$"
        answer_key = str(answer)
        difficulty = 1350
        
    elif problem_type == 'solve_exponential_logarithm':
        base = random.choice([2, 3, 5, 7])
        result = random.randint(20, 100)
        question = f"Solve for $x$ (express in logarithmic form): ${base}^x = {result}$"
        answer_key = f"log({result})/log({base})"
        difficulty = 1400
        
    elif problem_type == 'evaluate_exponential':
        base = random.choice([2, 3, 4, 5])
        exponent = Rational(random.choice([-2, -1, 1, 2, 3]), random.choice([1, 2, 3]))
        if exponent.q == 1:
            exponent = int(exponent)
        answer = base ** float(exponent)
        if exponent.q == 1:
            question = f"Evaluate: ${base}^{{{exponent}}}$"
        else:
            question = f"Evaluate: ${base}^{{{latex(exponent)}}}$"
        
        if answer == int(answer):
            answer_key = str(int(answer))
        else:
            answer_key = f"{base}**({exponent})"
        difficulty = 1320
        
    elif problem_type == 'growth_decay_word':
        initial = random.choice([100, 200, 500, 1000, 5000])
        rate_percent = random.choice([5, 10, 15, 20, 25])
        time = random.randint(2, 5)
        is_growth = random.choice([True, False])
        
        if is_growth:
            rate = 1 + rate_percent / 100
            answer = int(initial * (rate ** time))
            question = f"A population starts at {initial} and grows at {rate_percent}% per year. What is the population after {time} years? (Round to nearest whole number)"
        else:
            rate = 1 - rate_percent / 100
            answer = int(initial * (rate ** time))
            question = f"A car valued at $\\${initial}$ depreciates at {rate_percent}% per year. What is its value after {time} years? (Round to nearest whole number)"
        
        answer_key = str(answer)
        difficulty = 1450
        
    elif problem_type == 'exponential_equation_quadratic_form':
        base = random.choice([2, 3])
        u_val = random.randint(2, 4)
        answer = random.randint(1, 3)
        
        a = 1
        b = random.choice([-5, -4, -3, 3, 4, 5])
        c = -(u_val * (base**answer + b))
        
        question = f"Solve for $x$: ${base}^{{2x}} + {b} \\cdot {base}^x + {c} = 0$"
        answer_key = str(answer)
        difficulty = 1550
    
    return {
        "question_latex": question,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "exponential_functions",
        "grading_mode": "equivalent"
    }

problem = generate_exponential_problem()
print(json.dumps(problem))