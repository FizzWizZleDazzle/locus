"""
calculus - integration (hard)
Generated: 2026-02-11T22:00:50.463516
"""

import sympy as sp
import random
import json

x = sp.Symbol('x')

def generate_problem():
    problem_type = random.choice([
        'substitution',
        'by_parts',
        'trig_substitution',
        'partial_fractions',
        'complex_chain'
    ])
    
    if problem_type == 'substitution':
        # ELO 1600-1700: u-substitution with composition
        inner_choices = [
            (x**2 + random.randint(1, 5), 2*x),
            (random.randint(2, 5)*x + random.randint(1, 10), random.randint(2, 5)),
            (x**3 + random.randint(1, 5), 3*x**2)
        ]
        u, du_dx = random.choice(inner_choices)
        
        power = random.randint(2, 4)
        outer_coeff = random.randint(1, 3)
        
        integrand = outer_coeff * (u ** power) * du_dx
        integrand = sp.expand(integrand)
        
        answer = outer_coeff * (u ** (power + 1)) / (power + 1)
        answer = sp.expand(answer)
        
        question = sp.latex(integrand)
        difficulty = 1650
        
    elif problem_type == 'by_parts':
        # ELO 1700-1800: Integration by parts
        poly_deg = random.randint(1, 2)
        if poly_deg == 1:
            a = random.randint(1, 5)
            b = random.randint(1, 10)
            u_part = a*x + b
        else:
            a = random.randint(1, 3)
            b = random.randint(1, 5)
            u_part = a*x**2 + b*x
        
        exp_type = random.choice(['exp', 'trig'])
        if exp_type == 'exp':
            k = random.randint(1, 3)
            v_prime = sp.exp(k*x)
            integrand = u_part * v_prime
            answer = sp.integrate(integrand, x)
            difficulty = 1750
        else:
            k = random.randint(1, 3)
            trig_choice = random.choice([sp.sin, sp.cos])
            v_prime = trig_choice(k*x)
            integrand = u_part * v_prime
            answer = sp.integrate(integrand, x)
            difficulty = 1750
        
        question = sp.latex(integrand)
        
    elif problem_type == 'trig_substitution':
        # ELO 1800-1850: Trigonometric substitution patterns
        a = random.randint(2, 5)
        pattern_type = random.choice(['sqrt_a2_minus_x2', 'sqrt_x2_plus_a2'])
        
        if pattern_type == 'sqrt_a2_minus_x2':
            integrand = 1 / sp.sqrt(a**2 - x**2)
            answer = sp.asin(x/a)
            difficulty = 1800
        else:
            integrand = 1 / sp.sqrt(x**2 + a**2)
            answer = sp.asinh(x/a)
            difficulty = 1800
        
        question = sp.latex(integrand)
        
    elif problem_type == 'partial_fractions':
        # ELO 1750-1850: Partial fractions
        a = random.randint(1, 5)
        b = random.randint(1, 5)
        while b == a:
            b = random.randint(1, 5)
        
        A = random.randint(1, 4)
        B = random.randint(1, 4)
        
        numerator = A*(x - b) + B*(x - a)
        numerator = sp.expand(numerator)
        denominator = (x - a)*(x - b)
        
        integrand = numerator / denominator
        answer = A*sp.log(sp.Abs(x - a)) + B*sp.log(sp.Abs(x - b))
        
        question = sp.latex(integrand)
        difficulty = 1800
        
    else:  # complex_chain
        # ELO 1650-1750: Complex chain rule compositions
        inner_type = random.choice(['poly', 'trig'])
        if inner_type == 'poly':
            a = random.randint(2, 4)
            b = random.randint(1, 5)
            inner = a*x**2 + b
            inner_deriv = 2*a*x
        else:
            k = random.randint(1, 3)
            inner = sp.sin(k*x)
            inner_deriv = k*sp.cos(k*x)
        
        outer_type = random.choice(['exp', 'power'])
        if outer_type == 'exp':
            integrand = sp.exp(inner) * inner_deriv
            answer = sp.exp(inner)
            difficulty = 1700
        else:
            n = random.randint(3, 5)
            integrand = (inner ** n) * inner_deriv
            answer = (inner ** (n + 1)) / (n + 1)
            difficulty = 1680
        
        question = sp.latex(integrand)
    
    answer_str = sp.latex(answer)
    
    return {
        "question_latex": f"\\int {question} \\, dx",
        "answer_key": answer_str,
        "difficulty": difficulty,
        "main_topic": "calculus",
        "subtopic": "integration",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))