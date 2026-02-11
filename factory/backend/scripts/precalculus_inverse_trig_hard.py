"""
precalculus - inverse_trig (hard)
Generated: 2026-02-11T21:55:28.532926
"""

import sympy as sp
import random
import json

def generate_inverse_trig_problem():
    x, y, t = sp.symbols('x y t', real=True)
    
    problem_type = random.choice([
        'composition_with_trig',
        'composition_nested',
        'equation_solving',
        'simplify_complex',
        'derivative_inverse',
        'algebraic_manipulation'
    ])
    
    if problem_type == 'composition_with_trig':
        # sin(arccos(x)) type problems - medium-hard
        func_pairs = [
            ('sin', 'arccos', sp.sqrt(1 - x**2)),
            ('cos', 'arcsin', sp.sqrt(1 - x**2)),
            ('tan', 'arcsin', x / sp.sqrt(1 - x**2)),
            ('tan', 'arccos', sp.sqrt(1 - x**2) / x),
            ('sin', 'arctan', x / sp.sqrt(1 + x**2)),
            ('cos', 'arctan', 1 / sp.sqrt(1 + x**2)),
        ]
        outer, inner, answer = random.choice(func_pairs)
        
        # Add complexity with coefficients
        coef = random.choice([2, 3, sp.Rational(1, 2)])
        arg = coef * x
        
        if inner == 'arcsin':
            question = f"\\sin\\left(\\arcsin\\left({sp.latex(arg)}\\right)\\right)"
            answer_expr = arg
            elo = 1620
        elif inner == 'arccos' and outer == 'sin':
            question = f"\\sin\\left(\\arccos\\left({sp.latex(arg)}\\right)\\right)"
            answer_expr = sp.sqrt(1 - arg**2)
            elo = 1680
        elif inner == 'arcsin' and outer == 'cos':
            question = f"\\cos\\left(\\arcsin\\left({sp.latex(arg)}\\right)\\right)"
            answer_expr = sp.sqrt(1 - arg**2)
            elo = 1680
        elif inner == 'arctan' and outer == 'sin':
            question = f"\\sin\\left(\\arctan\\left({sp.latex(arg)}\\right)\\right)"
            answer_expr = arg / sp.sqrt(1 + arg**2)
            elo = 1750
        elif inner == 'arctan' and outer == 'cos':
            question = f"\\cos\\left(\\arctan\\left({sp.latex(arg)}\\right)\\right)"
            answer_expr = 1 / sp.sqrt(1 + arg**2)
            elo = 1750
        else:
            question = f"\\tan\\left(\\arcsin\\left({sp.latex(arg)}\\right)\\right)"
            answer_expr = arg / sp.sqrt(1 - arg**2)
            elo = 1780
            
    elif problem_type == 'composition_nested':
        # arcsin(sin(x)) with restrictions
        funcs = ['arcsin', 'arccos', 'arctan']
        func = random.choice(funcs)
        
        if func == 'arcsin':
            angle = random.choice([sp.pi/3, sp.pi/4, sp.pi/6, 2*sp.pi/3, 3*sp.pi/4])
            question = f"\\arcsin\\left(\\sin\\left({sp.latex(angle)}\\right)\\right)"
            if angle <= sp.pi/2:
                answer_expr = angle
            else:
                answer_expr = sp.pi - angle
            elo = 1720
        elif func == 'arccos':
            angle = random.choice([sp.pi/3, sp.pi/4, sp.pi/6, 2*sp.pi/3, 5*sp.pi/6])
            question = f"\\arccos\\left(\\cos\\left({sp.latex(angle)}\\right)\\right)"
            answer_expr = angle
            elo = 1720
        else:
            angle = random.choice([sp.pi/3, sp.pi/4, -sp.pi/4, -sp.pi/3])
            question = f"\\arctan\\left(\\tan\\left({sp.latex(angle)}\\right)\\right)"
            answer_expr = angle
            elo = 1700
            
    elif problem_type == 'equation_solving':
        # Solve equations with inverse trig
        eq_type = random.choice(['linear', 'quadratic'])
        
        if eq_type == 'linear':
            a = random.randint(2, 4)
            b = random.randint(1, 3)
            c = random.choice([sp.pi/6, sp.pi/4, sp.pi/3])
            # arcsin(ax + b) = c
            # Solution: x = (sin(c) - b)/a
            answer_expr = (sp.sin(c) - b) / a
            question = f"\\arcsin\\left({a}x + {b}\\right) = {sp.latex(c)}"
            elo = 1650
        else:
            # arctan(x) + arctan(2x) = pi/4
            question = "\\arctan(x) + \\arctan(2x) = \\frac{\\pi}{4}"
            # tan(arctan(x) + arctan(2x)) = 1
            # (x + 2x)/(1 - 2x^2) = 1
            # 3x = 1 - 2x^2, so 2x^2 + 3x - 1 = 0
            answer_expr = (-3 + sp.sqrt(17)) / 4
            elo = 1850
            
    elif problem_type == 'simplify_complex':
        # More complex expressions
        expr_choice = random.randint(1, 3)
        
        if expr_choice == 1:
            # arctan(x) + arctan(1/x) for x > 0
            question = "\\arctan(x) + \\arctan\\left(\\frac{1}{x}\\right) \\text{ for } x > 0"
            answer_expr = sp.pi / 2
            elo = 1780
        elif expr_choice == 2:
            # 2*arcsin(x)
            question = "\\sin(2\\arcsin(x))"
            answer_expr = 2 * x * sp.sqrt(1 - x**2)
            elo = 1820
        else:
            # arccos(x) + arcsin(x)
            question = "\\arccos(x) + \\arcsin(x)"
            answer_expr = sp.pi / 2
            elo = 1700
            
    elif problem_type == 'derivative_inverse':
        # Derivative of inverse trig functions with chain rule
        inner_choices = [2*x, x**2, sp.sqrt(x), 3*x - 1]
        inner = random.choice(inner_choices)
        func = random.choice(['arcsin', 'arctan', 'arccos'])
        
        if func == 'arcsin':
            f = sp.asin(inner)
            question = f"\\frac{{d}}{{dx}}\\left[\\arcsin\\left({sp.latex(inner)}\\right)\\right]"
            elo = 1760
        elif func == 'arctan':
            f = sp.atan(inner)
            question = f"\\frac{{d}}{{dx}}\\left[\\arctan\\left({sp.latex(inner)}\\right)\\right]"
            elo = 1740
        else:
            f = sp.acos(inner)
            question = f"\\frac{{d}}{{dx}}\\left[\\arccos\\left({sp.latex(inner)}\\right)\\right]