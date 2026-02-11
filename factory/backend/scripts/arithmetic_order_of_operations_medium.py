"""
arithmetic - order_of_operations (medium)
Generated: 2026-02-11T21:26:17.946395
"""

import sympy as sp
import random
import json

def generate_order_of_operations_problem():
    target_elo = random.randint(1300, 1600)
    
    if target_elo < 1400:
        answer = random.randint(5, 30)
        
        a = random.randint(2, 9)
        b = random.randint(2, 9)
        c = answer - a * b
        
        if random.choice([True, False]):
            question = f"{a} \\times {b} + {c}"
            actual_answer = a * b + c
        else:
            c = answer + a * b
            question = f"{c} - {a} \\times {b}"
            actual_answer = c - a * b
        
        elo = random.randint(1300, 1390)
    
    elif target_elo < 1500:
        answer = random.randint(10, 50)
        
        a = random.randint(2, 6)
        b = random.randint(2, 6)
        c = random.randint(2, 8)
        
        if random.choice([True, False]):
            d = answer - (a * b + c)
            question = f"{a} \\times {b} + {c} + {d}"
            actual_answer = a * b + c + d
        else:
            intermediate = a * b
            d = random.randint(2, 6)
            e = answer - intermediate - d
            question = f"{a} \\times {b} + {d} \\times {e}"
            actual_answer = a * b + d * e
        
        elo = random.randint(1400, 1490)
    
    else:
        answer = random.randint(5, 40)
        
        a = random.randint(2, 6)
        b = random.randint(2, 6)
        c = random.randint(1, 5)
        d = random.randint(2, 6)
        
        choice = random.randint(1, 3)
        
        if choice == 1:
            inner = a + b
            e = answer - inner * c
            question = f"({a} + {b}) \\times {c} + {e}"
            actual_answer = (a + b) * c + e
        elif choice == 2:
            inner = a * b
            outer = c + d
            f = answer - inner - outer
            question = f"{a} \\times {b} + ({c} + {d}) + {f}"
            actual_answer = a * b + (c + d) + f
        else:
            inner1 = a + b
            inner2 = c * d
            e = answer - inner1 - inner2
            question = f"({a} + {b}) + {c} \\times {d} + {e}"
            actual_answer = (a + b) + c * d + e
        
        elo = random.randint(1500, 1600)
    
    result = {
        "question_latex": question,
        "answer_key": str(actual_answer),
        "difficulty": elo,
        "main_topic": "arithmetic",
        "subtopic": "order_of_operations",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_order_of_operations_problem()
print(json.dumps(problem))