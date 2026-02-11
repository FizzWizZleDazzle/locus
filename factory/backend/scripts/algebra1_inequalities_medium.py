"""
algebra1 - inequalities (medium)
Generated: 2026-02-11T21:28:04.075991
"""

import random
import json
from sympy import symbols, latex, simplify, sympify

def generate_inequality_problem():
    x = symbols('x')
    
    problem_types = [
        'two_step',
        'distributive',
        'variables_both_sides',
        'compound'
    ]
    
    problem_type = random.choice(problem_types)
    
    if problem_type == 'two_step':
        # ELO: 1300-1400 (two-step: ax + b < c)
        a = random.choice([2, 3, 4, 5, -2, -3, -4])
        x_val = random.randint(-5, 5)
        b = random.randint(-10, 10)
        c = a * x_val + b + random.choice([-2, -1, 1, 2])
        
        if random.choice([True, False]):
            op = '<'
            if a > 0:
                answer = f'x < {(c - b) / a}'
            else:
                answer = f'x > {(c - b) / a}'
        else:
            op = '>'
            if a > 0:
                answer = f'x > {(c - b) / a}'
            else:
                answer = f'x < {(c - b) / a}'
        
        if a == 1:
            question = f"x + {b} {op} {c}" if b >= 0 else f"x - {-b} {op} {c}"
        elif a == -1:
            question = f"-x + {b} {op} {c}" if b >= 0 else f"-x - {-b} {op} {c}"
        else:
            if b >= 0:
                question = f"{a}x + {b} {op} {c}"
            else:
                question = f"{a}x - {-b} {op} {c}"
        
        difficulty = 1350
        
    elif problem_type == 'distributive':
        # ELO: 1400-1500 (requires distribution: a(bx + c) + d < e)
        a = random.choice([2, 3, -2, -3])
        b = random.choice([1, 2, -1])
        c = random.randint(-5, 5)
        d = random.randint(-8, 8)
        x_val = random.randint(-3, 3)
        e = a * (b * x_val + c) + d + random.choice([1, 2, 3])
        
        op = random.choice(['<', '>'])
        
        expanded_a = a * b
        expanded_b = a * c + d
        
        if op == '<':
            if expanded_a > 0:
                answer = f'x < {(e - expanded_b) / expanded_a}'
            else:
                answer = f'x > {(e - expanded_b) / expanded_a}'
        else:
            if expanded_a > 0:
                answer = f'x > {(e - expanded_b) / expanded_a}'
            else:
                answer = f'x < {(e - expanded_b) / expanded_a}'
        
        if c >= 0:
            inner = f"x + {c}" if b == 1 else f"{b}x + {c}" if b != -1 else f"-x + {c}"
        else:
            inner = f"x - {-c}" if b == 1 else f"{b}x - {-c}" if b != -1 else f"-x - {-c}"
        
        if a == -1:
            left_part = f"-({inner})"
        else:
            left_part = f"{a}({inner})"
        
        if d >= 0:
            question = f"{left_part} + {d} {op} {e}"
        else:
            question = f"{left_part} - {-d} {op} {e}"
        
        difficulty = 1450
        
    elif problem_type == 'variables_both_sides':
        # ELO: 1500-1550 (variables on both sides: ax + b < cx + d)
        a = random.choice([2, 3, 4, 5])
        c = random.choice([1, 2, 3])
        b = random.randint(-10, 10)
        d = random.randint(-10, 10)
        
        if a == c:
            c = a + 1
        
        op = random.choice(['<', '>'])
        
        coeff_diff = a - c
        const_diff = d - b
        
        if op == '<':
            if coeff_diff > 0:
                answer = f'x < {const_diff / coeff_diff}'
            else:
                answer = f'x > {const_diff / coeff_diff}'
        else:
            if coeff_diff > 0:
                answer = f'x > {const_diff / coeff_diff}'
            else:
                answer = f'x < {const_diff / coeff_diff}'
        
        left = f"{a}x + {b}" if b >= 0 else f"{a}x - {-b}"
        if c == 1:
            right = f"x + {d}" if d >= 0 else f"x - {-d}"
        else:
            right = f"{c}x + {d}" if d >= 0 else f"{c}x - {-d}"
        
        question = f"{left} {op} {right}"
        difficulty = 1520
        
    else:  # compound
        # ELO: 1550-1600 (compound: a < bx + c < d)
        b = random.choice([2, 3, -2, -3])
        c = random.randint(-5, 5)
        x_lower = random.randint(-3, 2)
        x_upper = x_lower + random.randint(2, 5)
        
        a = b * x_lower + c + random.choice([1, 2])
        d = b * x_upper + c - random.choice([1, 2])
        
        if b > 0:
            answer = f'{(a - c) / b} < x < {(d - c) / b}'
        else:
            answer = f'{(d - c) / b} < x < {(a - c) / b}'
        
        middle = f"{b}x + {c}" if c >= 0 else f"{b}x - {-c}"
        question = f"{a} < {middle} < {d}"
        difficulty = 1575
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "inequalities",
        "grading_mode": "equivalent"
    }

problem = generate_inequality_problem()
print(json.dumps(problem))