"""
arithmetic - multiplication_division (hard)
Generated: 2026-02-11T21:23:27.342195
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'multi_step_with_fractions',
        'nested_operations',
        'word_problem_complex',
        'mixed_operations_large',
        'division_with_remainders_complex'
    ])
    
    if problem_type == 'multi_step_with_fractions':
        # Pick clean answer
        answer = random.randint(5, 20)
        
        # Work backwards: answer * divisor / multiplier = original
        divisor = random.choice([3, 4, 5, 6, 8, 12])
        multiplier = random.choice([2, 3, 4, 5])
        
        # Create: (a * multiplier) / divisor = answer
        a = (answer * divisor) // multiplier
        if (answer * divisor) % multiplier != 0:
            a = (answer * divisor + multiplier) // multiplier
            answer = (a * multiplier) // divisor
        
        question = f"\\text{{Calculate: }} \\frac{{{a} \\times {multiplier}}}{{{divisor}}}"
        difficulty = 1650
        
    elif problem_type == 'nested_operations':
        # Pick clean final answer
        answer = random.randint(10, 50)
        
        # Build: ((a * b) / c + d) * e = answer
        e = random.choice([2, 3, 4])
        intermediate = answer // e
        if answer % e != 0:
            intermediate = answer // e + 1
            answer = intermediate * e
        
        d = random.randint(2, 8)
        quotient = intermediate - d
        
        c = random.choice([2, 3, 4, 5])
        b = random.choice([3, 4, 5, 6])
        a = (quotient * c) // b
        if (quotient * c) % b != 0:
            a = (quotient * c + b) // b
        
        question = f"\\text{{Calculate: }} \\left(\\frac{{{a} \\times {b}}}{{{c}}} + {d}\\right) \\times {e}"
        difficulty = 1750
        
    elif problem_type == 'word_problem_complex':
        # Pick clean answer (number of items per box)
        items_per_box = random.randint(8, 25)
        
        num_boxes = random.randint(12, 30)
        total_items = items_per_box * num_boxes
        
        containers = random.randint(3, 6)
        items_per_container = total_items // containers
        
        question = f"\\text{{A warehouse has {num_boxes} boxes, each containing {items_per_box} items.}}\\\\ \\text{{If these items are redistributed equally into {containers} containers, how many items per container?}}"
        answer = items_per_container
        difficulty = 1700
        
    elif problem_type == 'mixed_operations_large':
        # Pick clean answer
        answer = random.randint(15, 60)
        
        # Build: (a * b - c * d) / e = answer
        e = random.choice([3, 4, 5, 6])
        numerator = answer * e
        
        d = random.randint(5, 12)
        c = random.randint(4, 9)
        cd = c * d
        
        ab = numerator + cd
        b = random.choice([6, 7, 8, 9, 11])
        a = ab // b
        if ab % b != 0:
            a = ab // b + 1
            ab = a * b
            numerator = ab - cd
            answer = numerator // e
        
        question = f"\\text{{Calculate: }} \\frac{{{a} \\times {b} - {c} \\times {d}}}{{{e}}}"
        difficulty = 1680
        
    else:  # division_with_remainders_complex
        # Complex division chain
        start = random.randint(500, 999)
        d1 = random.choice([7, 9, 11, 13])
        
        q1 = start // d1
        r1 = start % d1
        
        d2 = random.choice([4, 5, 6])
        q2 = q1 // d2
        
        question = f"\\text{{Find the quotient when }} {start} \\text{{ is divided by }} {d1}, \\text{{ then that quotient is divided by }} {d2}."
        answer = q2
        difficulty = 1650
    
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