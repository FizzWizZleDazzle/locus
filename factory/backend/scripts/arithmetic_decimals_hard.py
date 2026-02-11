"""
arithmetic - decimals (hard)
Generated: 2026-02-11T21:25:07.295900
"""

import random
import json
from sympy import sympify, nsimplify, Float, Rational

def generate_problem():
    problem_type = random.choice([
        'multi_step_operations',
        'complex_word_problem',
        'mixed_operations_with_conversion',
        'decimal_equation_solving',
        'percent_decimal_compound'
    ])
    
    if problem_type == 'multi_step_operations':
        # Multi-step decimal operations (1600-1700)
        target = round(random.uniform(5, 50), 2)
        a = round(random.uniform(1, 10), 2)
        b = round(random.uniform(1, 10), 2)
        c = round(random.uniform(1, 5), 2)
        
        result = round((target * c + b) / a, 2)
        
        question = f"Solve for $x$: ${a}x - {b} = {target} \\times {c}$"
        answer = str(nsimplify(result, rational=False))
        difficulty = 1650
        
    elif problem_type == 'complex_word_problem':
        # Word problem with decimals requiring multiple steps (1700-1800)
        price_per_lb = round(random.uniform(2.5, 8.5), 2)
        quantity1 = round(random.uniform(1.5, 5.5), 1)
        quantity2 = round(random.uniform(1.5, 5.5), 1)
        discount_rate = random.choice([0.15, 0.20, 0.25])
        
        subtotal = (quantity1 + quantity2) * price_per_lb
        discount = subtotal * discount_rate
        total = round(subtotal - discount, 2)
        
        discount_percent = int(discount_rate * 100)
        question = f"A store sells apples at $\\${price_per_lb}$ per pound. Sarah buys ${quantity1}$ pounds and then ${quantity2}$ more pounds. If she receives a ${discount_percent}\\%$ discount on the total, how much does she pay?"
        answer = str(nsimplify(total, rational=False))
        difficulty = 1750
        
    elif problem_type == 'mixed_operations_with_conversion':
        # Complex mixed operations (1650-1750)
        a = round(random.uniform(2, 9), 1)
        b = round(random.uniform(2, 9), 1)
        c = round(random.uniform(1, 5), 2)
        d = round(random.uniform(1, 5), 2)
        
        result = round((a * b - c) / d, 2)
        
        question = f"Calculate: $\\frac{{{a} \\times {b} - {c}}}{{{d}}}$"
        answer = str(nsimplify(result, rational=False))
        difficulty = 1680
        
    elif problem_type == 'decimal_equation_solving':
        # Equation with decimals on both sides (1700-1800)
        x_val = round(random.uniform(3, 15), 1)
        a = round(random.uniform(1.5, 5.5), 1)
        b = round(random.uniform(2, 10), 2)
        c = round(random.uniform(1.5, 5.5), 1)
        
        d = round(a * x_val + b - c * x_val, 2)
        
        question = f"Solve for $x$: ${a}x + {b} = {c}x + {d}$"
        answer = str(nsimplify(x_val, rational=False))
        difficulty = 1720
        
    else:  # percent_decimal_compound
        # Compound percentage/decimal problem (1750-1850)
        original = round(random.uniform(100, 500), 2)
        increase_rate = random.choice([0.12, 0.15, 0.18, 0.20])
        decrease_rate = random.choice([0.08, 0.10, 0.12])
        
        after_increase = original * (1 + increase_rate)
        final = round(after_increase * (1 - decrease_rate), 2)
        
        inc_pct = int(increase_rate * 100)
        dec_pct = int(decrease_rate * 100)
        
        question = f"A price of $\\${original}$ is increased by ${inc_pct}\\%$, then decreased by ${dec_pct}\\%$. What is the final price?"
        answer = str(nsimplify(final, rational=False))
        difficulty = 1780
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "arithmetic",
        "subtopic": "decimals",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))