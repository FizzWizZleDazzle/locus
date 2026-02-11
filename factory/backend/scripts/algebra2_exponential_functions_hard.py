"""
algebra2 - exponential_functions (hard)
Generated: 2026-02-11T21:48:41.244453
"""

import sympy as sp
import random
import json

def generate_exponential_problem():
    x = sp.Symbol('x')
    
    problem_type = random.choice([
        'solve_exponential_equation',
        'compound_growth_decay',
        'exponential_with_logs',
        'system_exponential',
        'complex_exponential_equation'
    ])
    
    if problem_type == 'solve_exponential_equation':
        # Harder exponential equations requiring logarithms
        # Pick answer first
        solution = random.randint(2, 8)
        
        base = random.choice([2, 3, 5, 7])
        coefficient = random.randint(2, 9)
        constant = random.randint(10, 50)
        
        # Form: coefficient * base^x = result
        result = coefficient * (base ** solution) + constant
        
        question = f"Solve for $x$: ${coefficient} \\cdot {base}^x + {constant} = {result}$"
        answer = sp.sympify(solution)
        difficulty = random.randint(1600, 1700)
        
    elif problem_type == 'compound_growth_decay':
        # Word problem with exponential growth/decay
        initial = random.choice([1000, 5000, 10000, 20000])
        rate = random.choice([0.05, 0.08, 0.10, 0.12, 0.15])
        time = random.randint(5, 15)
        
        # Pick whether growth or decay
        if random.choice([True, False]):
            # Growth
            final = initial * ((1 + rate) ** time)
            question = f"An investment of $\\${initial}$ grows at ${int(rate*100)}\\%$ per year compounded annually. After how many years will it reach $\\${int(final)}$?"
        else:
            # Decay
            final = initial * ((1 - rate) ** time)
            question = f"A substance with initial mass ${initial}$ grams decays at ${int(rate*100)}\\%$ per year. After how many years will ${int(final)}$ grams remain?"
        
        answer = sp.sympify(time)
        difficulty = random.randint(1650, 1750)
        
    elif problem_type == 'exponential_with_logs':
        # Solve using logarithms
        solution = random.randint(3, 10)
        base = random.choice([2, 3, 4, 5])
        multiplier = random.randint(2, 5)
        
        # Form: base^(mx) = value
        value = base ** (multiplier * solution)
        
        question = f"Solve for $x$: ${base}^{{{multiplier}x}} = {value}$"
        answer = sp.sympify(solution)
        difficulty = random.randint(1600, 1680)
        
    elif problem_type == 'system_exponential':
        # Two exponential equations
        sol_x = random.randint(2, 5)
        sol_y = random.randint(2, 5)
        
        base1 = random.choice([2, 3])
        base2 = random.choice([2, 3, 5])
        
        # Form: base1^x * base2^y = result
        result1 = (base1 ** sol_x) * (base2 ** sol_y)
        result2 = (base1 ** (sol_x + 1)) * (base2 ** (sol_y - 1))
        
        question = f"Solve the system: ${base1}^x \\cdot {base2}^y = {result1}$ and ${base1}^{{x+1}} \\cdot {base2}^{{y-1}} = {result2}$. Find $x$."
        answer = sp.sympify(sol_x)
        difficulty = random.randint(1750, 1850)
        
    else:  # complex_exponential_equation
        # Exponential equation requiring substitution
        solution = random.randint(2, 6)
        base = random.choice([2, 3])
        
        # Form: base^(2x) + a*base^x - b = 0
        # Let u = base^x, then u^2 + a*u - b = 0
        u_val = base ** solution
        a = random.randint(2, 6)
        b = u_val * u_val + a * u_val
        
        question = f"Solve for $x$: ${base}^{{2x}} + {a} \\cdot {base}^x = {b}$"
        answer = sp.sympify(solution)
        difficulty = random.randint(1700, 1900)
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "exponential_functions",
        "grading_mode": "equivalent"
    }

problem = generate_exponential_problem()
print(json.dumps(problem))