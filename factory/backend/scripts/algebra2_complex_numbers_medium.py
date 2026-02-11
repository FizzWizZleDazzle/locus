"""
algebra2 - complex_numbers (medium)
Generated: 2026-02-11T21:45:55.772399
"""

import sympy as sp
import random
import json

def generate_complex_number_problem():
    problem_type = random.choice([
        'addition_subtraction',
        'multiplication',
        'division',
        'conjugate_product',
        'modulus',
        'powers'
    ])
    
    if problem_type == 'addition_subtraction':
        # ELO: 1300-1400 (two-step: add real parts, add imaginary parts)
        a1, b1 = random.randint(-10, 10), random.randint(-10, 10)
        a2, b2 = random.randint(-10, 10), random.randint(-10, 10)
        
        if b1 == 0 or b2 == 0:
            b1, b2 = random.randint(1, 10), random.randint(1, 10)
        
        z1 = a1 + b1 * sp.I
        z2 = a2 + b2 * sp.I
        
        operation = random.choice(['+', '-'])
        if operation == '+':
            answer = z1 + z2
            question = f"Simplify: ({a1} + {b1}i) + ({a2} + {b2}i)"
            elo = 1300
        else:
            answer = z1 - z2
            question = f"Simplify: ({a1} + {b1}i) - ({a2} + {b2}i)"
            elo = 1350
        
        return {
            "question_latex": question.replace('+-', '-').replace('+ -', '- '),
            "answer_key": str(answer),
            "difficulty": elo,
            "main_topic": "algebra2",
            "subtopic": "complex_numbers",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'multiplication':
        # ELO: 1400-1500 (FOIL method, combine terms)
        a1, b1 = random.randint(-8, 8), random.randint(-8, 8)
        a2, b2 = random.randint(-8, 8), random.randint(-8, 8)
        
        if b1 == 0:
            b1 = random.randint(1, 8)
        if b2 == 0:
            b2 = random.randint(1, 8)
        
        z1 = a1 + b1 * sp.I
        z2 = a2 + b2 * sp.I
        answer = sp.expand(z1 * z2)
        
        z1_str = f"{a1} + {b1}i" if b1 > 0 else f"{a1} - {abs(b1)}i"
        z2_str = f"{a2} + {b2}i" if b2 > 0 else f"{a2} - {abs(b2)}i"
        question = f"Multiply: ({z1_str})({z2_str})"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1450,
            "main_topic": "algebra2",
            "subtopic": "complex_numbers",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'division':
        # ELO: 1500-1600 (multiply by conjugate, simplify)
        a2, b2 = random.randint(-6, 6), random.randint(1, 6)
        if a2 == 0:
            a2 = random.randint(1, 6)
        
        # Pick clean answer first
        ans_real = random.randint(-5, 5)
        ans_imag = random.randint(-5, 5)
        answer = ans_real + ans_imag * sp.I
        
        # Work backward: z1 = answer * z2
        z2 = a2 + b2 * sp.I
        z1 = sp.expand(answer * z2)
        
        z1_str = str(z1).replace('I', 'i').replace('*', '')
        z2_str = f"{a2} + {b2}i" if b2 > 0 else f"{a2} - {abs(b2)}i"
        question = f"\\frac{{{z1_str}}}{{{z2_str}}}"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1550,
            "main_topic": "algebra2",
            "subtopic": "complex_numbers",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'conjugate_product':
        # ELO: 1400-1450 (multiply conjugates, difference of squares)
        a, b = random.randint(-8, 8), random.randint(1, 8)
        if a == 0:
            a = random.randint(1, 8)
        
        z = a + b * sp.I
        answer = sp.expand(z * sp.conjugate(z))
        
        z_str = f"{a} + {b}i"
        z_conj_str = f"{a} - {b}i"
        question = f"Multiply: ({z_str})({z_conj_str})"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1400,
            "main_topic": "algebra2",
            "subtopic": "complex_numbers",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'modulus':
        # ELO: 1350-1400 (apply modulus formula)
        a, b = random.randint(-10, 10), random.randint(-10, 10)
        if a == 0 and b == 0:
            a, b = random.randint(1, 10), random.randint(1, 10)
        
        z = a + b * sp.I
        answer = sp.sqrt(a**2 + b**2)
        
        z_str = f"{a} + {b}i" if b > 0 else f"{a} - {abs(b)}i"
        question = f"Find the modulus of {z_str}"
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1350,
            "main_topic": "algebra2",
            "subtopic": "complex_numbers",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'powers':
        # ELO: 1450-1550 (powers of i or small powers of complex numbers)
        subtype = random.choice(['power_of_i', 'complex_squared'])
        
        if subtype == 'power_of_i':
            # ELO: 1350 (pattern recognition)
            power = random.randint(3, 20)
            answer = sp.I ** power
            question = f"Simplify: i^{{{power}}}"
            
            return {
                "question_latex": question,
                "answer_key": str(answer),
                "difficulty": 1350,
                "main_topic": "algebra2",
                "subtopic": "complex_numbers",
                "grading_mode": "equivalent"
            }
        else:
            # ELO: 1500 (squaring or cubing)
            a, b = random.randint(-6, 6), random.randint(1, 6)
            if a == 0:
                a = random.randint(1, 6)
            
            z = a + b * sp.I