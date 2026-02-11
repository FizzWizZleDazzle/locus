"""
arithmetic - fractions (hard)
Generated: 2026-02-11T21:24:25.274062
"""

import random
import json
from sympy import symbols, Rational, simplify, latex, gcd, lcm, sqrt, Integer, nsimplify
from sympy.abc import x

def generate_hard_fraction_problem():
    problem_type = random.choice([
        'complex_mixed_operations',
        'nested_fractions',
        'fraction_equations',
        'multi_step_word_problem',
        'complex_simplification'
    ])
    
    if problem_type == 'complex_mixed_operations':
        # Multiple operations with 3-4 fractions
        answer = Rational(random.randint(1, 10), random.randint(1, 10))
        
        a = Rational(random.randint(1, 12), random.randint(2, 15))
        b = Rational(random.randint(1, 12), random.randint(2, 15))
        c = Rational(random.randint(1, 12), random.randint(2, 15))
        
        # Construct: answer = a + b * c - d or similar complex expression
        temp = a + b * c
        d = simplify(temp - answer)
        
        question = f"\\text{{Compute: }} {latex(a)} + {latex(b)} \\times {latex(c)} - \\left({latex(d)}\\right)"
        difficulty = 1650
        
    elif problem_type == 'nested_fractions':
        # Complex fractions with fractions in numerator/denominator
        answer = Rational(random.randint(1, 8), random.randint(1, 8))
        
        a = Rational(random.randint(1, 10), random.randint(2, 12))
        b = Rational(random.randint(1, 10), random.randint(2, 12))
        c = Rational(random.randint(1, 10), random.randint(2, 12))
        
        # Create nested: (a + b) / (c + d) = answer
        d = simplify((a + b) / answer - c)
        
        question = f"\\text{{Simplify: }} \\frac{{{latex(a)} + {latex(b)}}}{{{latex(c)} + {latex(d)}}}"
        difficulty = 1700
        
    elif problem_type == 'fraction_equations':
        # Solve equation with multiple fractions
        answer = Rational(random.randint(1, 15), random.randint(1, 8))
        
        a = Rational(random.randint(1, 9), random.randint(2, 10))
        b = Rational(random.randint(1, 9), random.randint(2, 10))
        c = Rational(random.randint(1, 9), random.randint(2, 10))
        
        # Create equation: a*x + b = c*x + d, solve for x = answer
        d = simplify(a * answer + b - c * answer)
        
        question = f"\\text{{Solve for }} x\\text{{: }} {latex(a)}x + {latex(b)} = {latex(c)}x + {latex(d)}"
        difficulty = 1750
        
    elif problem_type == 'multi_step_word_problem':
        # Word problem requiring multiple fraction operations
        answer = Rational(random.randint(2, 20), 1)
        
        frac1 = Rational(random.randint(1, 3), random.randint(4, 6))
        frac2 = Rational(random.randint(1, 2), random.randint(3, 5))
        
        # Total quantity where frac1 * total is used, then frac2 * remaining = final
        # answer * frac1 used, remaining = answer * (1 - frac1)
        # answer * (1 - frac1) * frac2 left
        left_over = simplify(answer * (1 - frac1) * (1 - frac2))
        
        question = f"\\text{{A baker made some cookies. He sold }} {latex(frac1)} \\text{{ of them in the morning}}"
        question += f"\\text{{ and }} {latex(frac2)} \\text{{ of the remainder in the afternoon. }}"
        question += f"\\text{{If }} {latex(left_over)} \\text{{ cookies remained, how many cookies did he make originally?}}"
        difficulty = 1800
        
    else:  # complex_simplification
        # Simplify expression with powers and multiple fractions
        answer = Rational(random.randint(1, 6), random.randint(1, 6))
        
        a = Rational(random.randint(2, 8), random.randint(3, 9))
        b = Rational(random.randint(2, 8), random.randint(3, 9))
        c = Rational(random.randint(2, 8), random.randint(3, 9))
        
        # Create: (a * b + c) / d = answer
        d = simplify((a * b + c) / answer)
        
        question = f"\\text{{Simplify: }} \\frac{{{latex(a)} \\times {latex(b)} + {latex(c)}}}{{{latex(d)}}}"
        difficulty = 1680
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "arithmetic",
        "subtopic": "fractions",
        "grading_mode": "equivalent"
    }

problem = generate_hard_fraction_problem()
print(json.dumps(problem))