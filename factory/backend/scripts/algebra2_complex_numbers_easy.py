"""
algebra2 - complex_numbers (easy)
Generated: 2026-02-11T21:45:31.583859
"""

import sympy as sp
import random
import json

def generate_complex_number_problem():
    problem_type = random.choice([
        'addition', 'subtraction', 'multiplication', 
        'simplify_i_power', 'conjugate', 'real_part', 'imaginary_part'
    ])
    
    if problem_type == 'addition':
        a1, b1 = random.randint(-10, 10), random.randint(-10, 10)
        a2, b2 = random.randint(-10, 10), random.randint(-10, 10)
        if b1 == 0 or b2 == 0:
            b1, b2 = random.randint(1, 10), random.randint(1, 10)
        
        ans_real = a1 + a2
        ans_imag = b1 + b2
        answer = sp.sympify(f"{ans_real} + {ans_imag}*I")
        
        z1_str = f"{a1} + {b1}i" if b1 >= 0 else f"{a1} - {-b1}i"
        z2_str = f"{a2} + {b2}i" if b2 >= 0 else f"{a2} - {-b2}i"
        question = f"({z1_str}) + ({z2_str})"
        difficulty = 1050
        
    elif problem_type == 'subtraction':
        a1, b1 = random.randint(-10, 10), random.randint(-10, 10)
        a2, b2 = random.randint(-10, 10), random.randint(-10, 10)
        if b1 == 0 or b2 == 0:
            b1, b2 = random.randint(1, 10), random.randint(1, 10)
        
        ans_real = a1 - a2
        ans_imag = b1 - b2
        answer = sp.sympify(f"{ans_real} + {ans_imag}*I")
        
        z1_str = f"{a1} + {b1}i" if b1 >= 0 else f"{a1} - {-b1}i"
        z2_str = f"{a2} + {b2}i" if b2 >= 0 else f"{a2} - {-b2}i"
        question = f"({z1_str}) - ({z2_str})"
        difficulty = 1100
        
    elif problem_type == 'multiplication':
        a1, b1 = random.randint(-5, 5), random.randint(-5, 5)
        a2, b2 = random.randint(-5, 5), random.randint(-5, 5)
        if a1 == 0: a1 = random.randint(1, 5)
        if b1 == 0: b1 = random.randint(1, 5)
        if a2 == 0: a2 = random.randint(1, 5)
        if b2 == 0: b2 = random.randint(1, 5)
        
        ans_real = a1 * a2 - b1 * b2
        ans_imag = a1 * b2 + a2 * b1
        answer = sp.sympify(f"{ans_real} + {ans_imag}*I")
        
        z1_str = f"{a1} + {b1}i" if b1 >= 0 else f"{a1} - {-b1}i"
        z2_str = f"{a2} + {b2}i" if b2 >= 0 else f"{a2} - {-b2}i"
        question = f"({z1_str})({z2_str})"
        difficulty = 1250
        
    elif problem_type == 'simplify_i_power':
        powers = [4, 5, 6, 7, 8, 9, 10, 12, 15, 16, 20]
        n = random.choice(powers)
        
        remainder = n % 4
        power_map = {0: 1, 1: sp.I, 2: -1, 3: -sp.I}
        answer = power_map[remainder]
        
        question = f"i^{{{n}}}"
        difficulty = 1150
        
    elif problem_type == 'conjugate':
        a, b = random.randint(-10, 10), random.randint(1, 10)
        if random.choice([True, False]):
            b = -b
        
        answer = sp.sympify(f"{a} - {b}*I")
        
        z_str = f"{a} + {b}i" if b >= 0 else f"{a} - {-b}i"
        question = f"\\text{{Find the conjugate of }} {z_str}"
        difficulty = 1050
        
    elif problem_type == 'real_part':
        a, b = random.randint(-10, 10), random.randint(-10, 10)
        if b == 0: b = random.randint(1, 10)
        
        answer = sp.sympify(str(a))
        
        z_str = f"{a} + {b}i" if b >= 0 else f"{a} - {-b}i"
        question = f"\\text{{Find the real part of }} {z_str}"
        difficulty = 1000
        
    elif problem_type == 'imaginary_part':
        a, b = random.randint(-10, 10), random.randint(-10, 10)
        if b == 0: b = random.randint(1, 10)
        
        answer = sp.sympify(str(b))
        
        z_str = f"{a} + {b}i" if b >= 0 else f"{a} - {-b}i"
        question = f"\\text{{Find the imaginary part of }} {z_str}"
        difficulty = 1000
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "complex_numbers",
        "grading_mode": "equivalent"
    }

problem = generate_complex_number_problem()
print(json.dumps(problem))