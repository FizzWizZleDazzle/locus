"""
calculus - applications_of_integration (hard)
Generated: 2026-02-11T22:02:51.123268
"""

import sympy as sp
import random
import json

def generate_problem():
    x, t = sp.symbols('x t')
    problem_type = random.choice([
        'volume_washer',
        'volume_shell',
        'arc_length',
        'work_problem',
        'area_between_curves_complex'
    ])
    
    if problem_type == 'volume_washer':
        # Pick clean answer first
        volume_answer = random.choice([sp.pi * sp.Rational(n, d) for n in [8, 16, 32, 64, 128] for d in [1, 3, 5, 15]])
        
        # Work backward: V = pi * int[(R^2 - r^2)dx]
        # Use functions that integrate cleanly
        a = random.randint(0, 2)
        b = random.randint(a + 2, a + 4)
        
        # Choose outer and inner functions
        choices = [
            (lambda: sp.sqrt(x), lambda: 0),
            (lambda: x, lambda: x**2 / random.choice([2, 4])),
            (lambda: random.choice([2, 3]) - x**2, lambda: 0),
        ]
        
        outer_func_gen, inner_func_gen = random.choice(choices)
        outer = outer_func_gen()
        inner = inner_func_gen()
        
        # Adjust bounds to get clean answer
        if outer == sp.sqrt(x):
            a, b = 0, random.choice([1, 4, 9])
            outer = sp.sqrt(x)
            inner = 0
        elif outer == x and inner == x**2 / 2:
            a, b = 0, 2
            outer = x
            inner = x**2 / 2
        else:
            a, b = 0, 1
            outer = 2 - x**2
            inner = 1
        
        question = f"Find the volume of the solid obtained by rotating the region bounded by $y = {sp.latex(outer)}$, $y = {sp.latex(inner)}$, $x = {a}$, and $x = {b}$ about the $x$-axis."
        
        integrand = sp.pi * (outer**2 - inner**2)
        answer = sp.integrate(integrand, (x, a, b))
        difficulty = 1650
        
    elif problem_type == 'volume_shell':
        # Cylindrical shells: V = 2*pi*int[x*f(x)dx]
        a = 0
        b = random.choice([1, 2])
        
        func_choices = [x - x**2, x**2, sp.sqrt(x)]
        func = random.choice(func_choices)
        
        if func == x - x**2:
            a, b = 0, 1
        elif func == x**2:
            a, b = 0, random.choice([1, 2])
        else:
            a, b = 0, random.choice([1, 4])
        
        question = f"Use the method of cylindrical shells to find the volume of the solid obtained by rotating the region bounded by $y = {sp.latex(func)}$, $y = 0$, $x = {a}$, and $x = {b}$ about the $y$-axis."
        
        integrand = 2 * sp.pi * x * func
        answer = sp.integrate(integrand, (x, a, b))
        difficulty = 1700
        
    elif problem_type == 'arc_length':
        # Arc length with clean answer
        func_choices = [
            (x**sp.Rational(3,2) / 3, 0, 4),
            ((x**3 / 6 + 1/(2*x)), 1, 2),
            (sp.Rational(2,3) * (x**2 + 1)**sp.Rational(3,2), 0, 1)
        ]
        
        func, a, b = random.choice(func_choices)
        
        question = f"Find the arc length of the curve $y = {sp.latex(func)}$ from $x = {a}$ to $x = {b}$."
        
        dy_dx = sp.diff(func, x)
        integrand = sp.sqrt(1 + dy_dx**2)
        answer = sp.integrate(integrand, (x, a, b))
        difficulty = 1750
        
    elif problem_type == 'work_problem':
        # Work problem with varying force
        length = random.choice([4, 6, 8, 10])
        k = random.choice([2, 3, 4, 5])
        natural_length = random.choice([2, 3])
        
        question = f"A spring has a natural length of ${natural_length}$ m. If a force of ${k * 2}$ N is required to stretch the spring to ${natural_length + 2}$ m, how much work is done in stretching the spring from ${natural_length}$ m to ${natural_length + 3}$ m?"
        
        # F = kx, work = int[kx dx] from 0 to stretch_distance
        x_var = sp.Symbol('x_var')
        work_integral = k * x_var
        answer = sp.integrate(work_integral, (x_var, 0, 3)) - sp.integrate(work_integral, (x_var, 0, 0))
        answer = sp.Rational(9 * k, 2)
        difficulty = 1650
        
    else:  # area_between_curves_complex
        # Complex area between curves requiring multiple integrals
        func1_choices = [x**2, x**3, sp.sin(x)]
        func2_choices = [2*x, x, sp.cos(x)]
        
        choice = random.randint(0, 1)
        if choice == 0:
            func1 = x**2
            func2 = 2*x
            a, b = 0, 2
            question = f"Find the area of the region bounded by $y = {sp.latex(func1)}$ and $y = {sp.latex(func2)}$."
        else:
            func1 = x**3
            func2 = x
            a, b = 0, 1
            question = f"Find the area of the region enclosed by $y = {sp.latex(func1)}$ and $y = {sp.latex(func2)}$ between their intersection points."
        
        # Find intersection points and integrate
        integrand = sp.Abs(func2 - func1)
        answer = sp.integrate(func2 - func1, (x, a, b))
        difficulty = 1600
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "calculus",
        "subtopic": "applications_of_integration",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))