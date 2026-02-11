"""
arithmetic - order_of_operations (easy)
Generated: 2026-02-11T21:26:05.883407
"""

import sympy as sp
import random
import json

def generate_order_of_operations_problem():
    elo_target = random.randint(1000, 1300)
    
    if elo_target <= 1150:
        # Very simple: single operation or two numbers with one operation beyond addition
        choice = random.choice(['add_sub', 'mult_div', 'simple_parens'])
        
        if choice == 'add_sub':
            a = random.randint(1, 20)
            b = random.randint(1, 20)
            c = random.randint(1, 15)
            answer = a + b - c
            if answer > 0:
                question = f"{a} + {b} - {c}"
                elo = 1050
            else:
                answer = a + b + c
                question = f"{a} + {b} + {c}"
                elo = 1000
        
        elif choice == 'mult_div':
            a = random.randint(2, 12)
            b = random.randint(2, 9)
            answer = a * b
            question = f"{a} \\times {b}"
            elo = 1100
        
        else:  # simple_parens
            a = random.randint(2, 10)
            b = random.randint(1, 10)
            c = random.randint(2, 5)
            answer = (a + b) * c
            question = f"({a} + {b}) \\times {c}"
            elo = 1150
    
    elif elo_target <= 1250:
        # Two operations: multiplication/division with addition/subtraction
        choice = random.choice(['mult_add', 'mult_sub', 'div_add'])
        
        if choice == 'mult_add':
            a = random.randint(2, 9)
            b = random.randint(2, 9)
            c = random.randint(1, 15)
            answer = a * b + c
            question = f"{a} \\times {b} + {c}"
            elo = 1200
        
        elif choice == 'mult_sub':
            a = random.randint(2, 9)
            b = random.randint(2, 9)
            c = random.randint(1, min(15, a * b - 1))
            answer = a * b - c
            question = f"{a} \\times {b} - {c}"
            elo = 1200
        
        else:  # div_add
            b = random.randint(2, 6)
            quotient = random.randint(2, 8)
            a = b * quotient
            c = random.randint(1, 12)
            answer = quotient + c
            question = f"{a} \\div {b} + {c}"
            elo = 1250
    
    else:
        # Three operations or nested parentheses
        choice = random.choice(['three_ops', 'nested_parens', 'mixed'])
        
        if choice == 'three_ops':
            a = random.randint(2, 8)
            b = random.randint(2, 6)
            c = random.randint(1, 10)
            d = random.randint(1, 10)
            answer = a * b + c - d
            if answer > 0:
                question = f"{a} \\times {b} + {c} - {d}"
            else:
                answer = a * b + c + d
                question = f"{a} \\times {b} + {c} + {d}"
            elo = 1280
        
        elif choice == 'nested_parens':
            a = random.randint(2, 8)
            b = random.randint(1, 8)
            c = random.randint(2, 5)
            d = random.randint(1, 6)
            answer = (a + b) * c - d
            if answer > 0:
                question = f"({a} + {b}) \\times {c} - {d}"
            else:
                answer = (a + b) * c + d
                question = f"({a} + {b}) \\times {c} + {d}"
            elo = 1300
        
        else:  # mixed
            a = random.randint(2, 6)
            b = random.randint(2, 6)
            c = random.randint(1, 8)
            d = random.randint(2, 4)
            answer = a * (b + c) - d
            if answer > 0:
                question = f"{a} \\times ({b} + {c}) - {d}"
            else:
                answer = a * (b + c) + d
                question = f"{a} \\times ({b} + {c}) + {d}"
            elo = 1290
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "arithmetic",
        "subtopic": "order_of_operations",
        "grading_mode": "equivalent"
    }
    
    print(json.dumps(result))

generate_order_of_operations_problem()