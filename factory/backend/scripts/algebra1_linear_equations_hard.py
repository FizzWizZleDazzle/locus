"""
algebra1 - linear_equations (hard)
Generated: 2026-02-11T21:27:31.148865
"""

import random
import json
from sympy import symbols, expand, simplify, latex, Rational, factorint

def generate_hard_linear_equation():
    x = symbols('x')
    
    # Pick a clean answer first
    answer = random.randint(-10, 10)
    
    # Choose problem type for 1600-1900 ELO range
    problem_type = random.choice([
        'nested_distribution',
        'multi_fraction',
        'complex_distribution',
        'double_nested',
        'fraction_both_sides'
    ])
    
    if problem_type == 'nested_distribution':
        # a(b(x - c) + d) + e(f(x - g) + h) = value
        # ELO: ~1650
        a = random.randint(2, 5)
        b = random.randint(2, 4)
        c = random.randint(1, 5)
        d = random.randint(1, 6)
        e = random.randint(2, 5)
        f = random.randint(2, 4)
        g = random.randint(1, 5)
        h = random.randint(1, 6)
        
        # Calculate what RHS should be
        lhs_expanded = a * b * answer - a * b * c + a * d + e * f * answer - e * f * g + e * h
        
        question = f"{a}({b}(x - {c}) + {d}) + {e}({f}(x - {g}) + {h}) = {lhs_expanded}"
        difficulty = 1650
        
    elif problem_type == 'multi_fraction':
        # (ax + b)/c + (dx + e)/f = value
        # ELO: ~1700
        a = random.randint(2, 6)
        b = random.randint(-8, 8)
        c = random.randint(2, 5)
        d = random.randint(2, 6)
        e = random.randint(-8, 8)
        f = random.randint(2, 5)
        
        # Make sure c and f are coprime for cleaner problem
        while factorint(c) == factorint(f) and c != f:
            f = random.randint(2, 5)
        
        rhs_value = Rational(a * answer + b, c) + Rational(d * answer + e, f)
        
        question = f"\\frac{{{a}x + {b}}}{{{c}}} + \\frac{{{d}x + {e}}}{{{f}}} = {latex(rhs_value)}"
        difficulty = 1700
        
    elif problem_type == 'complex_distribution':
        # a(bx + c) - d(ex - f) = g(hx + i) + value
        # ELO: ~1680
        a = random.randint(2, 5)
        b = random.randint(2, 5)
        c = random.randint(1, 6)
        d = random.randint(2, 5)
        e = random.randint(2, 5)
        f = random.randint(1, 6)
        g = random.randint(2, 4)
        h = random.randint(2, 5)
        i = random.randint(1, 6)
        
        lhs_coeff = a * b + d * e
        lhs_const = a * c + d * f
        rhs_coeff = g * h
        rhs_const = g * i
        
        # Solve: (lhs_coeff)x + lhs_const = (rhs_coeff)x + rhs_const + k
        # x = answer, so k = (lhs_coeff - rhs_coeff)*answer + lhs_const - rhs_const
        k = (lhs_coeff - rhs_coeff) * answer + lhs_const - rhs_const
        
        question = f"{a}({b}x + {c}) - {d}({e}x - {f}) = {g}({h}x + {i}) + {k}"
        difficulty = 1680
        
    elif problem_type == 'double_nested':
        # a(b(cx + d) + e) = f(g(hx + i) + j) + value
        # ELO: ~1750
        a = random.randint(2, 4)
        b = random.randint(2, 3)
        c = random.randint(2, 4)
        d = random.randint(1, 5)
        e = random.randint(1, 5)
        f = random.randint(2, 4)
        g = random.randint(2, 3)
        h = random.randint(2, 4)
        i = random.randint(1, 5)
        j = random.randint(1, 5)
        
        lhs_coeff = a * b * c
        lhs_const = a * b * d + a * e
        rhs_coeff = f * g * h
        rhs_const = f * g * i + f * j
        
        k = (lhs_coeff - rhs_coeff) * answer + lhs_const - rhs_const
        
        question = f"{a}({b}({c}x + {d}) + {e}) = {f}({g}({h}x + {i}) + {j}) + {k}"
        difficulty = 1750
        
    else:  # fraction_both_sides
        # (ax + b)/c - (dx + e)/f = (gx + h)/i
        # ELO: ~1800
        a = random.randint(2, 5)
        b = random.randint(-6, 6)
        c = random.randint(2, 4)
        d = random.randint(2, 5)
        e = random.randint(-6, 6)
        f = random.randint(2, 4)
        g = random.randint(2, 5)
        h = random.randint(-6, 6)
        i = random.randint(2, 4)
        
        rhs_value = Rational(g * answer + h, i)
        lhs_value = Rational(a * answer + b, c) - Rational(d * answer + e, f)
        
        # Calculate h needed
        h_needed = i * lhs_value - g * answer
        
        question = f"\\frac{{{a}x + {b}}}{{{c}}} - \\frac{{{d}x + {e}}}{{{f}}} = \\frac{{{g}x + {latex(h_needed)}}}{{{i}}}"
        difficulty = 1800
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "linear_equations",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_hard_linear_equation()
print(json.dumps(problem))