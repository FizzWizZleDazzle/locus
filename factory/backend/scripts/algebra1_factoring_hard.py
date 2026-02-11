"""
algebra1 - factoring (hard)
Generated: 2026-02-11T21:38:02.330386
"""

import random
import json
from sympy import symbols, expand, factor, latex, sympify

def generate_hard_factoring_problem():
    x = symbols('x')
    
    problem_type = random.choice([
        'difference_of_squares_complex',
        'quadratic_leading_coef',
        'cubic_factoring',
        'quadratic_complex_grouping',
        'sum_difference_cubes',
        'four_term_grouping'
    ])
    
    if problem_type == 'difference_of_squares_complex':
        a = random.randint(2, 5)
        b = random.randint(2, 6)
        c = random.randint(2, 5)
        d = random.randint(2, 6)
        
        answer = f"({a}*x + {b})*({a}*x - {b})"
        expr = expand(sympify(answer))
        
        gcd_factor = random.randint(1, 3)
        expr = gcd_factor * expr
        
        question = latex(expr)
        difficulty = random.randint(1620, 1680)
        
    elif problem_type == 'quadratic_leading_coef':
        a = random.randint(3, 8)
        b = random.randint(2, 7)
        c = random.randint(2, 7)
        d = random.randint(2, 7)
        
        answer = f"({a}*x + {b})*({c}*x + {d})"
        expr = expand(sympify(answer))
        
        question = latex(expr)
        difficulty = random.randint(1650, 1720)
        
    elif problem_type == 'cubic_factoring':
        a = random.randint(1, 4)
        b = random.randint(2, 5)
        c = random.randint(1, 4)
        d = random.randint(2, 5)
        
        answer = f"(x + {a})*(x + {b})*(x - {c})"
        expr = expand(sympify(answer))
        
        question = latex(expr)
        difficulty = random.randint(1700, 1780)
        
    elif problem_type == 'quadratic_complex_grouping':
        a = random.randint(2, 5)
        b = random.randint(2, 6)
        c = random.randint(2, 5)
        d = random.randint(2, 6)
        
        answer = f"({a}*x + {b})*({c}*x - {d})"
        expr = expand(sympify(answer))
        
        question = latex(expr)
        difficulty = random.randint(1680, 1750)
        
    elif problem_type == 'sum_difference_cubes':
        if random.choice([True, False]):
            a = random.randint(2, 5)
            b = random.randint(2, 5)
            answer = f"(x + {b})*(x**2 - {b}*x + {b**2})"
            expr = expand(sympify(answer))
        else:
            a = random.randint(2, 5)
            b = random.randint(2, 5)
            answer = f"(x - {b})*(x**2 + {b}*x + {b**2})"
            expr = expand(sympify(answer))
        
        question = latex(expr)
        difficulty = random.randint(1750, 1850)
        
    elif problem_type == 'four_term_grouping':
        a = random.randint(2, 5)
        b = random.randint(2, 5)
        c = random.randint(2, 5)
        d = random.randint(2, 5)
        
        answer = f"(x + {a})*({b}*x + {c})"
        expr = expand(sympify(answer))
        
        question = latex(expr)
        difficulty = random.randint(1640, 1720)
    
    answer_factored = str(factor(expr))
    
    return {
        "question_latex": f"\\text{{Factor completely: }} {question}",
        "answer_key": answer_factored,
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "factoring",
        "grading_mode": "equivalent"
    }

problem = generate_hard_factoring_problem()
print(json.dumps(problem))