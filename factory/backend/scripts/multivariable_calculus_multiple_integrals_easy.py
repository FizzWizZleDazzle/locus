"""
multivariable_calculus - multiple_integrals (easy)
Generated: 2026-02-11T22:04:03.180163
"""

import sympy as sp
import random
import json

x, y = sp.symbols('x y')

def generate_problem():
    problem_type = random.choice([
        'rectangular_simple',
        'rectangular_polynomial',
        'iterated_constant',
        'iterated_linear'
    ])
    
    if problem_type == 'rectangular_simple':
        # 1000-1100: Integrate a constant over a rectangle
        c = random.randint(1, 5)
        x_lower = random.randint(0, 2)
        x_upper = x_lower + random.randint(1, 3)
        y_lower = random.randint(0, 2)
        y_upper = y_lower + random.randint(1, 3)
        
        integrand = c
        answer = c * (x_upper - x_lower) * (y_upper - y_lower)
        
        question = f"\\int_{{{x_lower}}}^{{{x_upper}}} \\int_{{{y_lower}}}^{{{y_upper}}} {c} \\, dy \\, dx"
        difficulty = random.randint(1000, 1100)
        
    elif problem_type == 'rectangular_polynomial':
        # 1100-1200: Integrate a simple monomial over a rectangle
        coeff = random.randint(1, 4)
        power_choice = random.choice(['x', 'y', 'xy'])
        
        x_lower = 0
        x_upper = random.randint(1, 3)
        y_lower = 0
        y_upper = random.randint(1, 3)
        
        if power_choice == 'x':
            integrand = coeff * x
            answer = coeff * (x_upper**2 / 2 - x_lower**2 / 2) * (y_upper - y_lower)
            integrand_latex = f"{coeff}x"
        elif power_choice == 'y':
            integrand = coeff * y
            answer = coeff * (x_upper - x_lower) * (y_upper**2 / 2 - y_lower**2 / 2)
            integrand_latex = f"{coeff}y"
        else:  # xy
            integrand = coeff * x * y
            answer = coeff * (x_upper**2 / 2 - x_lower**2 / 2) * (y_upper**2 / 2 - y_lower**2 / 2)
            integrand_latex = f"{coeff}xy"
        
        question = f"\\int_{{{x_lower}}}^{{{x_upper}}} \\int_{{{y_lower}}}^{{{y_upper}}} {integrand_latex} \\, dy \\, dx"
        difficulty = random.randint(1100, 1200)
        
    elif problem_type == 'iterated_constant':
        # 1150-1250: Double integral with constant, clear order
        c = random.randint(2, 6)
        x_lower = 0
        x_upper = random.randint(1, 4)
        y_lower = 0
        y_upper = random.randint(1, 4)
        
        integrand = c
        answer = c * (x_upper - x_lower) * (y_upper - y_lower)
        
        question = f"\\iint_R {c} \\, dA \\text{{ where }} R = [{x_lower}, {x_upper}] \\times [{y_lower}, {y_upper}]"
        difficulty = random.randint(1150, 1250)
        
    else:  # iterated_linear
        # 1200-1300: Polynomial integrand over rectangle
        coeff1 = random.randint(1, 3)
        coeff2 = random.randint(1, 3)
        
        x_lower = 0
        x_upper = random.randint(1, 2)
        y_lower = 0
        y_upper = random.randint(1, 2)
        
        integrand = coeff1 * x + coeff2 * y
        
        int_x = coeff1 * (x_upper**2 / 2 - x_lower**2 / 2)
        int_y = coeff2 * (y_upper**2 / 2 - y_lower**2 / 2)
        answer = int_x * (y_upper - y_lower) + (x_upper - x_lower) * int_y
        
        integrand_latex = f"{coeff1}x + {coeff2}y"
        question = f"\\int_{{{x_lower}}}^{{{x_upper}}} \\int_{{{y_lower}}}^{{{y_upper}}} ({integrand_latex}) \\, dy \\, dx"
        difficulty = random.randint(1200, 1300)
    
    answer_simplified = sp.simplify(sp.Rational(answer))
    
    return {
        "question_latex": question,
        "answer_key": str(answer_simplified),
        "difficulty": difficulty,
        "main_topic": "multivariable_calculus",
        "subtopic": "multiple_integrals",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))