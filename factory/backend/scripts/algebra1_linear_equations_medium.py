"""
algebra1 - linear_equations (medium)
Generated: 2026-02-11T21:27:08.450031
"""

import random
import json
from sympy import symbols, simplify, latex, Eq

def generate_linear_equation():
    x = symbols('x')
    
    problem_type = random.choice(['two_step', 'distribution', 'variables_both_sides', 'multi_step'])
    
    if problem_type == 'two_step':
        answer = random.randint(-10, 10)
        a = random.choice([2, 3, 4, 5, -2, -3, -4, -5])
        b = random.randint(-20, 20)
        lhs = a * x + b
        rhs = a * answer + b
        question = f"{latex(lhs)} = {rhs}"
        difficulty = random.randint(1250, 1350)
        
    elif problem_type == 'distribution':
        answer = random.randint(-8, 8)
        outer = random.choice([2, 3, 4, -2, -3])
        inner_coef = random.choice([1, 2, -1, -2])
        inner_const = random.randint(-5, 5)
        add_const = random.randint(-10, 10)
        
        lhs_expr = outer * (inner_coef * x + inner_const) + add_const
        rhs_value = outer * (inner_coef * answer + inner_const) + add_const
        
        question = f"{outer}({latex(inner_coef*x + inner_const)}) + {add_const} = {rhs_value}"
        if add_const < 0:
            question = f"{outer}({latex(inner_coef*x + inner_const)}) - {abs(add_const)} = {rhs_value}"
        
        difficulty = random.randint(1450, 1550)
        
    elif problem_type == 'variables_both_sides':
        answer = random.randint(-10, 10)
        a1 = random.choice([2, 3, 4, 5])
        b1 = random.randint(-15, 15)
        a2 = random.choice([1, 2, 3])
        if a1 == a2:
            a2 += 1
        
        lhs = a1 * x + b1
        rhs = a2 * x + (a2 * answer + (a1 * answer + b1) - a2 * answer)
        
        question = f"{latex(lhs)} = {latex(rhs)}"
        difficulty = random.randint(1400, 1500)
        
    else:  # multi_step
        answer = random.randint(-8, 8)
        a = random.choice([2, 3, -2, -3])
        b = random.randint(-10, 10)
        c = random.randint(-8, 8)
        d = random.choice([2, 3, 4])
        
        lhs = a * x + b
        rhs_part1 = c
        rhs_part2 = d * x
        
        rhs_value = c + d * answer
        lhs_value = a * answer + b
        
        diff = lhs_value - rhs_value
        rhs_final = c + d * x - diff
        
        lhs_expr = a * x + b
        rhs_expr = simplify(rhs_final)
        
        question = f"{latex(lhs_expr)} = {latex(rhs_expr)}"
        difficulty = random.randint(1500, 1600)
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "linear_equations",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_linear_equation()
print(json.dumps(problem))