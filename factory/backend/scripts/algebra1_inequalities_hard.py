"""
algebra1 - inequalities (hard)
Generated: 2026-02-11T21:28:32.050601
"""

import sympy as sp
import random
import json

def generate_hard_inequality():
    x = sp.Symbol('x')
    
    problem_type = random.choice([
        'compound_with_distribution',
        'absolute_value_compound',
        'rational_inequality',
        'quadratic_inequality',
        'nested_absolute_value',
        'multi_step_compound'
    ])
    
    if problem_type == 'compound_with_distribution':
        # Pattern: a(bx + c) + d < e AND f(gx + h) > i
        a = random.randint(2, 4)
        b = random.randint(2, 5)
        c = random.randint(-5, 5)
        d = random.randint(-10, 10)
        
        # Pick clean solution range, e.g., -2 < x < 3
        sol_left = random.randint(-5, 0)
        sol_right = sol_left + random.randint(3, 6)
        
        # Build first inequality: a(bx + c) + d < e
        # Solve for e: when x = sol_right, we want equality
        e = a * (b * sol_right + c) + d
        
        # Build second inequality: f(gx + h) > i
        f = random.randint(2, 4)
        g = random.randint(2, 5)
        h = random.randint(-5, 5)
        # When x = sol_left, we want equality
        i = f * (g * sol_left + h)
        
        question = f"{a}({b}x + {c}) + {d} < {e} \\text{{ and }} {f}({g}x + {h}) > {i}"
        answer = f"{sol_left} < x < {sol_right}"
        difficulty = random.randint(1650, 1750)
        
    elif problem_type == 'absolute_value_compound':
        # Pattern: a < |bx + c| < d
        # Pick clean solution
        sol_left1 = random.randint(-6, -2)
        sol_right1 = random.randint(-1, 1)
        sol_left2 = random.randint(2, 4)
        sol_right2 = random.randint(5, 8)
        
        b = random.randint(2, 4)
        c = random.randint(-5, 5)
        
        # The inner values at boundaries
        inner_at_left = b * sol_left1 + c
        inner_at_right = b * sol_right2 + c
        
        a = random.randint(1, abs(inner_at_left) - 1) if inner_at_left != 0 else 1
        d = abs(inner_at_right) + random.randint(0, 2)
        
        question = f"{a} < |{b}x + {c}| < {d}"
        answer = f"(x < {sol_right1} \\text{{ or }} x > {sol_left2}) \\text{{ and }} ({sol_left1} < x < {sol_right2})"
        difficulty = random.randint(1750, 1850)
        
    elif problem_type == 'rational_inequality':
        # Pattern: (ax + b)/(cx + d) > e
        # Pick excluded point and solution boundary
        excluded = random.randint(-5, 5)
        if excluded == 0:
            excluded = 1
        
        boundary = excluded + random.randint(2, 5)
        
        c = random.randint(1, 3)
        d = -c * excluded
        
        a = random.randint(2, 4)
        b = random.randint(-10, 10)
        e = random.randint(-3, 3)
        
        # Adjust to get clean boundary
        # At boundary: (a*boundary + b)/(c*boundary + d) = e
        # So a*boundary + b = e*(c*boundary + d)
        b = e * (c * boundary + d) - a * boundary
        
        if random.choice([True, False]):
            question = f"\\frac{{{a}x + {b}}}{{{c}x + {d}}} > {e}"
            if boundary > excluded:
                answer = f"x < {excluded} \\text{{ or }} x > {boundary}"
            else:
                answer = f"{boundary} < x < {excluded}"
        else:
            question = f"\\frac{{{a}x + {b}}}{{{c}x + {d}}} \\leq {e}"
            if boundary > excluded:
                answer = f"{excluded} < x \\leq {boundary}"
            else:
                answer = f"x \\leq {boundary} \\text{{ or }} x > {excluded}"
        
        difficulty = random.randint(1700, 1800)
        
    elif problem_type == 'quadratic_inequality':
        # Pattern: ax^2 + bx + c < 0
        # Pick two roots
        r1 = random.randint(-5, 0)
        r2 = random.randint(1, 6)
        
        a = random.choice([1, 2, -1, -2])
        
        # Build quadratic from roots
        expr = a * (x - r1) * (x - r2)
        expanded = sp.expand(expr)
        
        comp = random.choice(['<', '>', '\\leq', '\\geq'])
        question = f"{sp.latex(expanded)} {comp} 0"
        
        if (a > 0 and comp in ['<', '\\leq']) or (a < 0 and comp in ['>', '\\geq']):
            if comp in ['<', '>']:
                answer = f"{r1} < x < {r2}"
            else:
                answer = f"{r1} \\leq x \\leq {r2}"
        else:
            if comp in ['<', '>']:
                answer = f"x < {r1} \\text{{ or }} x > {r2}"
            else:
                answer = f"x \\leq {r1} \\text{{ or }} x \\geq {r2}"
        
        difficulty = random.randint(1650, 1750)
        
    elif problem_type == 'nested_absolute_value':
        # Pattern: |ax + b| + c > d|ex + f|
        sol_point = random.randint(-3, 3)
        
        a = random.randint(2, 4)
        b = random.randint(-10, 10)
        c = random.randint(1, 5)
        d = random.randint(2, 3)
        e = random.randint(2, 4)
        f = random.randint(-10, 10)
        
        question = f"|{a}x + {b}| + {c} > {d}|{e}x + {f}|"
        answer = f"x > {sol_point}"
        difficulty = random.randint(1800, 1900)
        
    else:  # multi_step_compound
        # Pattern: a|bx + c| - d < e AND fx + g > h
        sol_left = random.randint(-4, 0)
        sol_right = random.randint(1, 5)
        
        a = random.randint(2, 4)
        b = random.randint(2, 4)
        c = random.randint(-8, 8)
        d = random.randint(5, 15)
        
        # At x = sol_right and x = sol_left, determine e
        val_at_right = abs(b * sol_right + c)
        e = a * val_at_right - d + random.randint(0, 3)
        
        f = random.randint(2