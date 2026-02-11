"""
algebra2 - logarithms (hard)
Generated: 2026-02-11T21:49:36.773114
"""

import sympy as sp
import random
import json

def generate_logarithm_problem():
    x = sp.Symbol('x')
    problem_type = random.choice([
        'multi_log_equation',
        'log_inequality_system',
        'exponential_log_combo',
        'change_of_base_equation',
        'log_quadratic',
        'nested_log'
    ])
    
    if problem_type == 'multi_log_equation':
        # Problems like log_a(x) + log_b(x) = c
        # Pick answer first
        ans = random.choice([2, 3, 4, 5, 8, 9, 16, 25])
        base1 = random.choice([2, 3, 4, 5])
        base2 = random.choice([2, 3, 5, 7])
        while base2 == base1:
            base2 = random.choice([2, 3, 5, 7])
        
        val1 = sp.log(ans, base1)
        val2 = sp.log(ans, base2)
        total = val1 + val2
        
        question = f"\\log_{{{base1}}}(x) + \\log_{{{base2}}}(x) = {sp.latex(total)}"
        answer = str(ans)
        difficulty = 1650
        
    elif problem_type == 'log_inequality_system':
        # log_a(x-b) - log_a(x-c) > d
        ans = random.randint(5, 12)
        base = random.choice([2, 3, 5])
        b = random.randint(1, ans - 3)
        c = random.randint(0, b - 1)
        
        # Use quotient rule: log(x-b) - log(x-c) = log((x-b)/(x-c))
        # We want (x-b)/(x-c) > base^d
        d = 1
        ratio_at_ans = (ans - b) / (ans - c)
        
        question = f"\\log_{{{base}}}(x - {b}) - \\log_{{{base}}}(x - {c}) > {d}"
        answer = f"({ans}, oo)"
        difficulty = 1700
        
    elif problem_type == 'exponential_log_combo':
        # 2^(log_2(x)) + 3^(log_3(x)) = k
        ans = random.randint(3, 10)
        k = 2 * ans
        
        question = f"2^{{\\log_2(x)}} + 3^{{\\log_3(x)}} = {k}"
        answer = str(ans)
        difficulty = 1620
        
    elif problem_type == 'change_of_base_equation':
        # log_a(x) * log_x(b) = c
        ans = random.choice([4, 8, 9, 16, 25, 27])
        base_a = random.choice([2, 3, 5])
        base_b = random.choice([2, 3, 4, 5, 6])
        
        # log_a(ans) * log_ans(base_b) = log_a(base_b)
        result = sp.log(base_b, base_a)
        
        question = f"\\log_{{{base_a}}}(x) \\cdot \\log_x({base_b}) = {sp.latex(result)}"
        answer = str(ans)
        difficulty = 1750
        
    elif problem_type == 'log_quadratic':
        # log_a(x^2 + bx + c) = d
        # Pick factored form (x-r1)(x-r2)
        r1 = random.randint(2, 5)
        r2 = random.randint(6, 10)
        ans_values = [r1, r2]
        ans = random.choice(ans_values)
        
        b_coef = -(r1 + r2)
        c_coef = r1 * r2
        
        base = random.choice([2, 3, 5, 10])
        d = random.randint(1, 3)
        rhs = base ** d
        
        # x^2 + bx + c = rhs
        new_c = c_coef - rhs
        
        # Solve x^2 + b*x + new_c = 0
        discriminant = b_coef**2 - 4*new_c
        if discriminant > 0:
            sol1 = (-b_coef + discriminant**0.5) / 2
            sol2 = (-b_coef - discriminant**0.5) / 2
            # Pick positive solution
            answer = str(max(sol1, sol2) if sol1 > 0 or sol2 > 0 else sol1)
        else:
            answer = str(r1)
        
        if b_coef >= 0:
            question = f"\\log_{{{base}}}(x^2 + {b_coef}x + {c_coef}) = {d}"
        else:
            question = f"\\log_{{{base}}}(x^2 {b_coef}x + {c_coef}) = {d}"
        
        difficulty = 1680
        
    else:  # nested_log
        # log_a(log_b(x)) = c
        ans = random.choice([8, 16, 27, 32, 64, 81, 125, 243])
        base_b = random.choice([2, 3, 5])
        base_a = random.choice([2, 3])
        c = random.randint(1, 2)
        
        # log_b(ans) = base_a^c
        inner = base_a ** c
        # So ans = base_b^inner
        ans = base_b ** inner
        
        question = f"\\log_{{{base_a}}}(\\log_{{{base_b}}}(x)) = {c}"
        answer = str(ans)
        difficulty = 1800
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "logarithms",
        "grading_mode": "equivalent"
    }

problem = generate_logarithm_problem()
print(json.dumps(problem))