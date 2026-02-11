"""
algebra2 - radical_expressions (hard)
Generated: 2026-02-11T21:47:52.302691
"""

import random
import json
from sympy import *

def generate_problem():
    x = symbols('x')
    
    problem_type = random.choice([
        'rationalize_complex_denominator',
        'solve_radical_equation_multi',
        'simplify_nested_radicals',
        'radical_equation_extraneous',
        'combine_complex_radicals'
    ])
    
    if problem_type == 'rationalize_complex_denominator':
        a = random.randint(2, 5)
        b = random.randint(2, 5)
        c = random.randint(2, 4)
        d = random.randint(2, 4)
        
        numerator = a
        denom_rational = b
        denom_radical_coef = c
        denom_radical_inside = d**2 * random.randint(2, 3)
        
        question = f"\\frac{{{a}}}{{{b} + {c}\\sqrt{{{denom_radical_inside}}}}}"
        
        conjugate_denom = b**2 - c**2 * denom_radical_inside
        answer_num = a * b - a * c * sqrt(denom_radical_inside)
        answer = simplify(answer_num / conjugate_denom)
        
        return {
            "question_latex": f"Rationalize the denominator: ${question}$",
            "answer_key": str(answer),
            "difficulty": random.randint(1650, 1750),
            "main_topic": "algebra2",
            "subtopic": "radical_expressions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'solve_radical_equation_multi':
        sol = random.randint(3, 12)
        
        k = random.randint(2, 4)
        m = random.randint(1, 3)
        
        inside1 = k * sol + m
        inside2 = sol - m
        
        question = f"\\sqrt{{{k}x + {m}}} + \\sqrt{{x - {m}}} = {sqrt(inside1) + sqrt(inside2)}"
        
        return {
            "question_latex": f"Solve for $x$: ${question}$",
            "answer_key": str(sol),
            "difficulty": random.randint(1700, 1800),
            "main_topic": "algebra2",
            "subtopic": "radical_expressions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'simplify_nested_radicals':
        a = random.randint(2, 5)
        b = random.randint(2, 5)
        
        inside = a**2 + b**2 + 2*a*b
        
        question = f"\\sqrt{{{a**2} + {b**2} + 2\\sqrt{{{a**2 * b**2}}}}}"
        answer = a + b
        
        return {
            "question_latex": f"Simplify: ${question}$",
            "answer_key": str(answer),
            "difficulty": random.randint(1750, 1850),
            "main_topic": "algebra2",
            "subtopic": "radical_expressions",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'radical_equation_extraneous':
        real_sol = random.randint(5, 15)
        
        a = random.randint(2, 4)
        b = real_sol - a
        
        inside_expr = f"{a}x - {a*b}"
        rhs = real_sol
        
        question = f"\\sqrt{{{inside_expr}}} = x - {b}"
        
        return {
            "question_latex": f"Solve for $x$ (exclude extraneous solutions): ${question}$",
            "answer_key": str(real_sol),
            "difficulty": random.randint(1650, 1750),
            "main_topic": "algebra2",
            "subtopic": "radical_expressions",
            "grading_mode": "equivalent"
        }
    
    else:  # combine_complex_radicals
        a = random.randint(2, 4)
        b = random.randint(2, 4)
        base = random.randint(2, 5)
        
        coef1 = a
        coef2 = b
        inside1 = base * random.choice([4, 9, 16])
        inside2 = base
        
        question = f"{coef1}\\sqrt{{{inside1}}} - {coef2}\\sqrt{{{inside2}}}"
        
        factor1 = int(sqrt(inside1 // base))
        combined_coef = coef1 * factor1 - coef2
        answer = combined_coef * sqrt(base)
        
        return {
            "question_latex": f"Simplify: ${question}$",
            "answer_key": str(answer),
            "difficulty": random.randint(1600, 1700),
            "main_topic": "algebra2",
            "subtopic": "radical_expressions",
            "grading_mode": "equivalent"
        }

problem = generate_problem()
print(json.dumps(problem))