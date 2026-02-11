"""
algebra1 - polynomials (hard)
Generated: 2026-02-11T21:37:21.415685
"""

import sympy as sp
import random
import json

def generate_hard_polynomial_problem():
    x = sp.Symbol('x')
    
    problem_type = random.choice([
        'factor_complex',
        'polynomial_division_remainder',
        'factor_by_grouping',
        'complete_square_complex',
        'system_polynomial',
        'factor_sum_difference_cubes'
    ])
    
    if problem_type == 'factor_complex':
        a = random.choice([1, 2, 3])
        b = random.choice([1, 2, 3, 4, 5])
        c = random.choice([1, 2, 3, 4])
        d = random.choice([1, 2, 3, 4, 5])
        
        factor1 = a*x + b
        factor2 = c*x - d
        
        expanded = sp.expand(factor1 * factor2)
        
        question = f"Factor completely: ${sp.latex(expanded)}$"
        answer = f"({sp.latex(factor1)})({sp.latex(factor2)})"
        difficulty = 1650
        
    elif problem_type == 'polynomial_division_remainder':
        q_a = random.randint(1, 3)
        q_b = random.randint(-5, 5)
        quotient = q_a * x + q_b
        
        d_a = random.randint(1, 2)
        d_b = random.randint(-4, 4)
        if d_b == 0:
            d_b = 1
        divisor = d_a * x + d_b
        
        remainder = random.randint(-10, 10)
        
        dividend = sp.expand(quotient * divisor + remainder)
        
        question = f"Find the remainder when ${sp.latex(dividend)}$ is divided by ${sp.latex(divisor)}$"
        answer = str(remainder)
        difficulty = 1700
        
    elif problem_type == 'factor_by_grouping':
        a = random.randint(1, 3)
        b = random.randint(1, 4)
        c = random.randint(1, 3)
        d = random.randint(1, 4)
        
        poly = a*c*x**3 + a*d*x**2 + b*c*x + b*d
        
        question = f"Factor by grouping: ${sp.latex(poly)}$"
        answer = f"({a}x + {b})({c}x^2 + {d})" if a > 0 and b > 0 else sp.latex(sp.factor(poly))
        difficulty = 1680
        
    elif problem_type == 'complete_square_complex':
        a = random.choice([2, 3, 4])
        h = random.randint(-5, 5)
        k = random.randint(-10, 10)
        
        vertex_form = a * (x - h)**2 + k
        expanded = sp.expand(vertex_form)
        
        question = f"Rewrite ${sp.latex(expanded)}$ in vertex form $a(x-h)^2 + k$"
        answer = sp.latex(vertex_form)
        difficulty = 1720
        
    elif problem_type == 'system_polynomial':
        a = random.randint(2, 5)
        b = random.randint(1, 4)
        c = random.randint(1, 5)
        
        poly1 = x**2 + a*x + b
        poly2 = x + c
        
        result = sp.expand(poly1 * poly2)
        
        coeff_x2 = result.as_coefficients_dict()[x**2]
        
        question = f"If $(x^2 + {a}x + {b})(x + {c}) = x^3 + bx^2 + cx + d$, find the value of $b$"
        answer = str(coeff_x2)
        difficulty = 1650
        
    elif problem_type == 'factor_sum_difference_cubes':
        if random.choice([True, False]):
            a = random.randint(1, 4)
            b = random.randint(1, 4)
            poly = a**3 * x**3 + b**3
            question = f"Factor completely: ${sp.latex(poly)}$"
            answer = sp.latex(sp.factor(poly))
            difficulty = 1750
        else:
            a = random.randint(1, 4)
            b = random.randint(1, 4)
            poly = a**3 * x**3 - b**3
            question = f"Factor completely: ${sp.latex(poly)}$"
            answer = sp.latex(sp.factor(poly))
            difficulty = 1750
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "polynomials",
        "grading_mode": "equivalent"
    }

problem = generate_hard_polynomial_problem()
print(json.dumps(problem))