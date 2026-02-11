"""
algebra1 - factoring (easy)
Generated: 2026-02-11T21:37:34.225095
"""

import random
import json
from sympy import symbols, expand, factor, latex

def generate_factoring_problem():
    x = symbols('x')
    
    problem_type = random.randint(1, 4)
    
    if problem_type == 1:
        # Simple GCF factoring: ax + ab -> a(x + b)
        # ELO: 1000-1100 (one-step, identify common factor)
        a = random.randint(2, 10)
        b = random.randint(2, 12)
        
        # Answer: a(x + b)
        answer = f"{a}*(x + {b})"
        
        # Problem: ax + ab
        expanded = a * x + a * b
        question = f"\\text{{Factor: }} {latex(expanded)}"
        
        elo = random.randint(1000, 1100)
        
    elif problem_type == 2:
        # Difference of squares: x^2 - a^2 -> (x-a)(x+a)
        # ELO: 1100-1200 (recognize pattern, one-step)
        a = random.randint(2, 10)
        
        # Answer: (x - a)(x + a)
        answer = f"(x - {a})*(x + {a})"
        
        # Problem: x^2 - a^2
        expanded = x**2 - a**2
        question = f"\\text{{Factor: }} {latex(expanded)}"
        
        elo = random.randint(1100, 1200)
        
    elif problem_type == 3:
        # Simple trinomial: x^2 + bx + c where c = mn, b = m+n
        # ELO: 1200-1300 (two factors, find pair that adds to b)
        m = random.randint(1, 8)
        n = random.randint(1, 8)
        
        b = m + n
        c = m * n
        
        # Answer: (x + m)(x + n)
        answer = f"(x + {m})*(x + {n})"
        
        # Problem: x^2 + bx + c
        expanded = x**2 + b*x + c
        question = f"\\text{{Factor: }} {latex(expanded)}"
        
        elo = random.randint(1200, 1300)
        
    else:
        # GCF with two terms, slightly larger coefficients
        # ELO: 1050-1150 (one-step but requires identifying GCF)
        gcd_val = random.randint(3, 8)
        coef1 = random.randint(2, 6)
        coef2 = random.randint(2, 6)
        
        # Answer: gcd_val(coef1*x + coef2)
        answer = f"{gcd_val}*({coef1}*x + {coef2})"
        
        # Problem: gcd_val*coef1*x + gcd_val*coef2
        expanded = gcd_val * coef1 * x + gcd_val * coef2
        question = f"\\text{{Factor: }} {latex(expanded)}"
        
        elo = random.randint(1050, 1150)
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": elo,
        "main_topic": "algebra1",
        "subtopic": "factoring",
        "grading_mode": "equivalent"
    }

problem = generate_factoring_problem()
print(json.dumps(problem))