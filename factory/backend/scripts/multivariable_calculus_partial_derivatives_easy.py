"""
multivariable_calculus - partial_derivatives (easy)
Generated: 2026-02-11T22:03:06.799974
"""

import sympy as sp
import random
import json

x, y, z = sp.symbols('x y z')

def generate_problem():
    problem_type = random.choice([
        'simple_polynomial',
        'power_function',
        'simple_product',
        'simple_exponential',
        'simple_trig'
    ])
    
    if problem_type == 'simple_polynomial':
        # Type: ∂/∂x of simple polynomial like 3x^2 + 2y
        power = random.randint(2, 4)
        coeff = random.randint(2, 8)
        const_term = random.randint(1, 9)
        
        func = coeff * x**power + const_term * y
        partial_deriv = sp.diff(func, x)
        
        question = f"\\frac{{\\partial}}{{\\partial x}}\\left({sp.latex(func)}\\right)"
        answer = str(partial_deriv)
        elo = 1050
        
    elif problem_type == 'power_function':
        # Type: ∂/∂y of x^a * y^b (simple power)
        a = random.randint(1, 3)
        b = random.randint(2, 4)
        coeff = random.randint(1, 5)
        
        func = coeff * x**a * y**b
        partial_deriv = sp.diff(func, y)
        
        question = f"\\frac{{\\partial}}{{\\partial y}}\\left({sp.latex(func)}\\right)"
        answer = str(partial_deriv)
        elo = 1100
        
    elif problem_type == 'simple_product':
        # Type: ∂/∂x of xy or x^2*y
        power_x = random.randint(1, 3)
        power_y = random.randint(1, 2)
        coeff = random.randint(2, 6)
        
        func = coeff * x**power_x * y**power_y
        var_choice = random.choice([x, y])
        partial_deriv = sp.diff(func, var_choice)
        
        var_str = 'x' if var_choice == x else 'y'
        question = f"\\frac{{\\partial}}{{\\partial {var_str}}}\\left({sp.latex(func)}\\right)"
        answer = str(partial_deriv)
        elo = 1150
        
    elif problem_type == 'simple_exponential':
        # Type: ∂/∂x of e^(ax) * y or e^x + y^2
        a = random.randint(1, 3)
        b = random.randint(2, 3)
        
        if random.choice([True, False]):
            func = sp.exp(a*x) + y**b
            partial_deriv = sp.diff(func, x)
            var_str = 'x'
        else:
            func = x**b + sp.exp(a*y)
            partial_deriv = sp.diff(func, y)
            var_str = 'y'
        
        question = f"\\frac{{\\partial}}{{\\partial {var_str}}}\\left({sp.latex(func)}\\right)"
        answer = str(partial_deriv)
        elo = 1250
        
    else:  # simple_trig
        # Type: ∂/∂x of sin(x) + y^2 or cos(y) + x^2
        power = random.randint(2, 3)
        coeff = random.randint(1, 4)
        
        if random.choice([True, False]):
            trig_func = random.choice([sp.sin, sp.cos])
            func = trig_func(x) + coeff * y**power
            partial_deriv = sp.diff(func, x)
            var_str = 'x'
        else:
            trig_func = random.choice([sp.sin, sp.cos])
            func = coeff * x**power + trig_func(y)
            partial_deriv = sp.diff(func, y)
            var_str = 'y'
        
        question = f"\\frac{{\\partial}}{{\\partial {var_str}}}\\left({sp.latex(func)}\\right)"
        answer = str(partial_deriv)
        elo = 1200
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": elo,
        "main_topic": "multivariable_calculus",
        "subtopic": "partial_derivatives",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))