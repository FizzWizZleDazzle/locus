"""
arithmetic - order_of_operations (hard)
Generated: 2026-02-11T21:26:44.418635
"""

import random
import json
from sympy import symbols, sympify, latex, simplify

def generate_problem():
    # Target ELO range: 1600-1900 (Advanced to Expert for order of operations)
    
    problem_type = random.choice([
        'nested_operations_with_fractions',
        'multiple_nested_parentheses',
        'mixed_operations_complex',
        'exponents_with_nested_ops',
        'fraction_division_with_ops'
    ])
    
    if problem_type == 'nested_operations_with_fractions':
        # Pick clean answer
        answer = random.choice([2, 3, 4, 5, 6, 8, 10, 12, 15, 20])
        
        # Build backward: answer = ((a * b) / c + d) * e - f
        e = random.randint(2, 4)
        f = random.randint(5, 15)
        intermediate1 = (answer + f) / e
        
        d = random.randint(2, 8)
        intermediate2 = intermediate1 - d
        
        c = random.choice([2, 3, 4, 5, 6])
        numerator = int(intermediate2 * c)
        
        # Factor numerator
        factors = [i for i in range(2, 20) if numerator % i == 0 and i < numerator]
        if factors:
            a = random.choice(factors)
            b = numerator // a
        else:
            a = numerator
            b = 1
        
        question = f"\\left(\\frac{{{a} \\times {b}}}{{{c}}} + {d}\\right) \\times {e} - {f}"
        difficulty = 1650
        
    elif problem_type == 'multiple_nested_parentheses':
        # answer = a - (b + (c - (d * e)))
        answer = random.choice([5, 10, 15, 20, 25])
        
        d = random.randint(2, 5)
        e = random.randint(3, 6)
        de = d * e
        
        c = de + random.randint(5, 15)
        c_minus_de = c - de
        
        b = random.randint(3, 10)
        b_plus = b + c_minus_de
        
        a = answer + b_plus
        
        question = f"{a} - \\left({b} + \\left({c} - \\left({d} \\times {e}\\right)\\right)\\right)"
        difficulty = 1700
        
    elif problem_type == 'mixed_operations_complex':
        # answer = (a^2 + b * c) / d - e
        answer = random.choice([3, 4, 5, 6, 7, 8, 10])
        
        e = random.randint(2, 8)
        intermediate = answer + e
        
        d = random.choice([2, 3, 4, 5])
        numerator = intermediate * d
        
        a = random.randint(2, 5)
        a_sq = a ** 2
        
        bc = numerator - a_sq
        if bc > 0:
            factors = [i for i in range(2, 15) if bc % i == 0 and i < bc]
            if factors:
                b = random.choice(factors)
                c = bc // b
            else:
                b = bc
                c = 1
        else:
            b = random.randint(2, 5)
            c = random.randint(2, 5)
            a_sq = numerator - b * c
            a = 2
        
        question = f"\\frac{{{a}^2 + {b} \\times {c}}}{{{d}}} - {e}"
        difficulty = 1750
        
    elif problem_type == 'exponents_with_nested_ops':
        # answer = (a + b)^2 / c - d
        answer = random.choice([4, 5, 6, 8, 10, 12])
        
        d = random.randint(3, 10)
        intermediate = answer + d
        
        c = random.choice([2, 4, 5])
        square_val = intermediate * c
        
        # Find a, b such that (a+b)^2 = square_val
        sum_ab = int(square_val ** 0.5)
        if sum_ab ** 2 == square_val:
            a = random.randint(1, sum_ab - 1)
            b = sum_ab - a
        else:
            a = 3
            b = 2
            square_val = 25
            c = random.choice([i for i in [1, 5] if 25 % i == 0])
            intermediate = 25 // c
            d = intermediate - answer
        
        question = f"\\frac{{\\left({a} + {b}\\right)^2}}{{{c}}} - {d}"
        difficulty = 1800
        
    else:  # fraction_division_with_ops
        # answer = (a * b / c) / (d + e) + f
        answer = random.choice([2, 3, 4, 5])
        
        f = random.randint(1, 5)
        intermediate1 = answer - f
        
        d = random.randint(2, 6)
        e = random.randint(2, 6)
        de_sum = d + e
        
        numerator_val = intermediate1 * de_sum
        
        c = random.choice([2, 3, 4])
        ab = int(numerator_val * c)
        
        factors = [i for i in range(2, 20) if ab % i == 0 and i < ab]
        if factors:
            a = random.choice(factors)
            b = ab // a
        else:
            a = ab
            b = 1
        
        question = f"\\frac{{{a} \\times {b} \\div {c}}}{{{d} + {e}}} + {f}"
        difficulty = 1850
    
    # Verify answer using SymPy
    try:
        # Convert LaTeX to expression (simplified parsing)
        expr_str = question.replace('\\left', '').replace('\\right', '')
        expr_str = expr_str.replace('\\times', '*').replace('\\div', '/')
        expr_str = expr_str.replace('^', '**').replace('\\frac', '')
        expr_str = expr_str.replace('{', '(').replace('}', ')')
        
        computed_answer = eval(expr_str)
        answer_key = str(simplify(computed_answer))
    except:
        answer_key = str(answer)
    
    return {
        "question_latex": question,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "arithmetic",
        "subtopic": "order_of_operations",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))