"""
arithmetic - multiplication_division (medium)
Generated: 2026-02-11T21:23:08.041536
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'two_step_mixed',
        'division_with_remainder_check',
        'multi_step_chain',
        'word_problem_basic',
        'combined_operations'
    ])
    
    if problem_type == 'two_step_mixed':
        # 1300-1400: Simple two-step: multiply then divide or vice versa
        answer = random.randint(3, 15)
        multiplier = random.randint(2, 9)
        divisor = random.randint(2, 8)
        
        intermediate = answer * divisor
        start_value = intermediate // multiplier
        
        if random.choice([True, False]):
            question = f"{start_value} \\times {multiplier} \\div {divisor}"
        else:
            question = f"{intermediate} \\div {divisor} \\times {answer // start_value if start_value != 0 else 1}"
            answer = intermediate // divisor * (answer // start_value if start_value != 0 else 1)
        
        difficulty = random.randint(1300, 1400)
        
    elif problem_type == 'division_with_remainder_check':
        # 1400-1500: Division resulting in whole number, verify by multiplication
        answer = random.randint(8, 25)
        divisor = random.randint(3, 12)
        dividend = answer * divisor
        
        question = f"\\text{{Calculate: }} {dividend} \\div {divisor}"
        difficulty = random.randint(1350, 1450)
        
    elif problem_type == 'multi_step_chain':
        # 1450-1550: Three operations in sequence
        answer = random.randint(5, 20)
        
        a = random.randint(2, 8)
        b = random.randint(2, 7)
        c = random.randint(2, 6)
        
        # Work backward: answer <- * c <- / b <- * a
        step3 = answer
        step2 = step3 * b
        step1 = step2 // c
        start = step1 // a
        
        # Verify forward
        check = start * a * c // b
        if check == answer:
            question = f"{start} \\times {a} \\times {c} \\div {b}"
        else:
            # Fallback simpler
            answer = random.randint(6, 18)
            x = random.randint(2, 6)
            y = random.randint(2, 5)
            start = answer * y // x
            question = f"{start} \\times {x} \\div {y}"
            answer = start * x // y
            
        difficulty = random.randint(1450, 1550)
        
    elif problem_type == 'word_problem_basic':
        # 1500-1600: Simple word problem requiring setup
        answer = random.randint(4, 15)
        
        scenarios = [
            (lambda a, m: (f"\\text{{If }} {m} \\text{{ boxes each contain }} {a} \\text{{ items, how many items in one box?}}", m * a, m, 'div'),
             lambda a, m: a),
            (lambda a, p: (f"\\text{{A group of }} {p} \\text{{ people share }} {a * p} \\text{{ items equally. How many does each person get?}}", a * p, p, 'div'),
             lambda a, p: a),
            (lambda a, r: (f"\\text{{Calculate: }} {r} \\text{{ groups of }} {a}", r, a, 'mult'),
             lambda a, r: a * r)
        ]
        
        scenario_func, answer_func = random.choice(scenarios)
        param = random.randint(3, 8)
        question_text, num1, num2, op = scenario_func(answer, param)
        
        if op == 'div':
            question = question_text
            answer = answer_func(answer, param)
        else:
            question = question_text
            answer = answer_func(answer, param)
            
        difficulty = random.randint(1500, 1600)
        
    else:  # combined_operations
        # 1400-1500: Parentheses with mixed operations
        answer = random.randint(6, 20)
        
        a = random.randint(2, 6)
        b = random.randint(2, 5)
        
        inner = answer * b
        outer = inner // a
        
        question = f"({outer} \\times {a}) \\div {b}"
        difficulty = random.randint(1400, 1500)
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "arithmetic",
        "subtopic": "multiplication_division",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_problem()
print(json.dumps(problem))