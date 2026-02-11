"""
arithmetic - addition_subtraction (hard)
Generated: 2026-02-11T21:22:35.454041
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'mixed_operations_large',
        'strategic_grouping',
        'pattern_completion',
        'multi_level_nested',
        'algebraic_thinking'
    ])
    
    if problem_type == 'mixed_operations_large':
        # Large numbers with strategic addition/subtraction requiring mental regrouping
        answer = random.randint(500, 2000)
        num_terms = random.randint(5, 8)
        
        # Build expression backward from answer
        current = answer
        terms = []
        for i in range(num_terms - 1):
            if random.choice([True, False]):
                delta = random.randint(50, 500)
                terms.append(f"+{delta}")
                current -= delta
            else:
                delta = random.randint(50, 500)
                terms.append(f"-{delta}")
                current += delta
        
        question = f"{current}" + "".join(terms)
        difficulty = 1650 + random.randint(0, 100)
        
    elif problem_type == 'strategic_grouping':
        # Problems requiring strategic regrouping to simplify
        answer = random.randint(100, 500)
        complement = random.randint(10, 50)
        base = random.randint(50, 200)
        
        # Create expression like: a + (100 - b) + b + c that simplifies nicely
        part1 = answer - 100 - base
        part2 = 100 - complement
        part3 = complement
        part4 = base
        
        terms = [part1, part2, part3, part4]
        random.shuffle(terms)
        
        question = str(terms[0])
        for t in terms[1:]:
            if t >= 0:
                question += f"+{t}"
            else:
                question += f"{t}"
        
        difficulty = 1700 + random.randint(0, 150)
        
    elif problem_type == 'pattern_completion':
        # Arithmetic sequences with missing values requiring addition/subtraction
        start = random.randint(100, 500)
        diff = random.choice([12, 15, 18, 20, 25, 30])
        length = random.randint(6, 9)
        
        sequence = [start + i * diff for i in range(length)]
        missing_indices = random.sample(range(1, length-1), 2)
        
        # Answer is sum of missing values
        answer = sum(sequence[i] for i in missing_indices)
        
        # Create question showing pattern with blanks
        display = []
        for i, val in enumerate(sequence):
            if i in missing_indices:
                display.append("\\Box")
            else:
                display.append(str(val))
        
        question = f"\\text{{Find the sum of missing values: }}" + ", ".join(display)
        difficulty = 1750 + random.randint(0, 100)
        
    elif problem_type == 'multi_level_nested':
        # Nested operations requiring careful order tracking
        answer = random.randint(200, 800)
        
        inner1 = random.randint(50, 150)
        inner2 = random.randint(30, 100)
        inner_sum = inner1 + inner2
        
        outer_add = random.randint(100, 300)
        remaining = answer - outer_add - inner_sum
        
        question = f"{remaining} + ({inner1} + {inner2}) + {outer_add}"
        difficulty = 1600 + random.randint(0, 100)
        
    else:  # algebraic_thinking
        # Problems like: "What must be added to A to get B?" but with complex expressions
        target = random.randint(1000, 3000)
        num_known_terms = random.randint(4, 6)
        
        known_sum = 0
        terms = []
        for _ in range(num_known_terms):
            if random.choice([True, False]):
                val = random.randint(50, 400)
                terms.append(f"+{val}")
                known_sum += val
            else:
                val = random.randint(50, 400)
                terms.append(f"-{val}")
                known_sum -= val
        
        answer = target - known_sum
        
        expression = "".join(terms)
        if expression[0] == '+':
            expression = expression[1:]
        
        question = f"\\text{{Find }} x \\text{{ if }} x{expression} = {target}"
        difficulty = 1800 + random.randint(0, 100)
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "arithmetic",
        "subtopic": "addition_subtraction",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_problem()
print(json.dumps(problem))