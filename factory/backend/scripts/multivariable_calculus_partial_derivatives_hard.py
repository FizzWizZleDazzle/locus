"""
multivariable_calculus - partial_derivatives (hard)
Generated: 2026-02-11T22:03:47.728996
"""

import sympy as sp
import random
import json

def generate_problem():
    x, y, z, t = sp.symbols('x y z t', real=True)
    
    problem_type = random.choice([
        'chain_rule_implicit',
        'mixed_partial_equality',
        'directional_derivative',
        'gradient_level_curve',
        'second_order_mixed',
        'implicit_differentiation'
    ])
    
    if problem_type == 'chain_rule_implicit':
        # Harder: Chain rule with substitution
        a = random.choice([2, 3, 4])
        b = random.choice([2, 3])
        c = random.choice([1, 2, 3])
        
        # Pick clean answer first
        if random.choice([True, False]):
            # w = x^a * y^b where x = r*cos(t), y = r*sin(t)
            answer = a * sp.cos(t)**a * sp.sin(t)**b * sp.cos(t) + b * sp.cos(t)**a * sp.sin(t)**(b-1) * sp.sin(t)
            answer = sp.simplify(answer)
            
            question = f"Let $w = x^{{{a}}} y^{{{b}}}$ where $x = r\\cos(t)$ and $y = r\\sin(t)$. Find $\\frac{{\\partial w}}{{\\partial t}}$ when $r = 1$."
            
            r_sym = sp.Symbol('r', real=True, positive=True)
            x_expr = r_sym * sp.cos(t)
            y_expr = r_sym * sp.sin(t)
            w = x_expr**a * y_expr**b
            dwdt = sp.diff(w, t)
            answer = dwdt.subs(r_sym, 1)
            answer = sp.simplify(answer)
            
        else:
            # w = x^2 + y^2 where x = r*cos(t), y = r*sin(t), find dw/dr
            answer = sp.Integer(2)
            question = f"Let $w = x^2 + y^2$ where $x = r\\cos(t)$ and $y = r\\sin(t)$. Find $\\frac{{\\partial w}}{{\\partial r}}$."
            
            r_sym = sp.Symbol('r', real=True, positive=True)
            x_expr = r_sym * sp.cos(t)
            y_expr = r_sym * sp.sin(t)
            w = x_expr**2 + y_expr**2
            answer = sp.diff(w, r_sym)
            answer = sp.simplify(answer)
        
        difficulty = random.randint(1650, 1750)
        
    elif problem_type == 'mixed_partial_equality':
        # Verify or find condition for f_xy = f_yx
        a = random.choice([2, 3, 4])
        b = random.choice([2, 3, 4])
        k = random.randint(2, 5)
        
        # Build function where we know mixed partials
        f = x**a * y**b + k * sp.sin(x*y)
        f_xy = sp.diff(sp.diff(f, x), y)
        answer = sp.simplify(f_xy)
        
        question = f"For $f(x,y) = x^{{{a}}} y^{{{b}}} + {k}\\sin(xy)$, compute $\\frac{{\\partial^2 f}}{{\\partial x \\partial y}}$."
        difficulty = random.randint(1600, 1700)
        
    elif problem_type == 'directional_derivative':
        # Directional derivative in direction of vector
        a = random.choice([2, 3])
        b = random.choice([2, 3])
        
        # Function and point
        f = x**a + y**b
        x0 = random.choice([1, 2])
        y0 = random.choice([1, 2])
        
        # Direction vector
        v1 = random.choice([1, 2, 3])
        v2 = random.choice([1, 2, 3])
        
        # Compute gradient at point
        grad_x = sp.diff(f, x).subs([(x, x0), (y, y0)])
        grad_y = sp.diff(f, y).subs([(x, x0), (y, y0)])
        
        # Unit vector
        magnitude = sp.sqrt(v1**2 + v2**2)
        u1 = v1 / magnitude
        u2 = v2 / magnitude
        
        # Directional derivative
        answer = grad_x * u1 + grad_y * u2
        answer = sp.simplify(answer)
        
        question = f"Find the directional derivative of $f(x,y) = x^{{{a}}} + y^{{{b}}}$ at the point $({x0}, {y0})$ in the direction of the vector $\\langle {v1}, {v2} \\rangle$."
        difficulty = random.randint(1700, 1800)
        
    elif problem_type == 'gradient_level_curve':
        # Find gradient perpendicular to level curve
        a = random.choice([2, 3])
        b = random.choice([2, 3])
        
        f = x**a + y**b
        x0 = random.choice([1, 2])
        y0 = random.choice([1, 2])
        
        grad_x = sp.diff(f, x).subs([(x, x0), (y, y0)])
        grad_y = sp.diff(f, y).subs([(x, x0), (y, y0)])
        
        answer = sp.Matrix([grad_x, grad_y])
        answer_str = f"[{grad_x}, {grad_y}]"
        
        question = f"Find the gradient vector $\\nabla f$ of $f(x,y) = x^{{{a}}} + y^{{{b}}}$ at the point $({x0}, {y0})$."
        difficulty = random.randint(1650, 1750)
        
        return {
            "question_latex": question,
            "answer_key": answer_str,
            "difficulty": difficulty,
            "main_topic": "multivariable_calculus",
            "subtopic": "partial_derivatives",
            "grading_mode": "equivalent"
        }
        
    elif problem_type == 'second_order_mixed':
        # Second order partial derivatives
        a = random.choice([3, 4])
        b = random.choice([2, 3])
        
        f = x**a * y**b
        f_xx = sp.diff(f, x, 2)
        answer = sp.simplify(f_xx)
        
        question = f"For $f(x,y) = x^{{{a}}} y^{{{b}}}$, compute $\\frac{{\\partial^2 f}}{{\\partial x^2}}$."
        difficulty = random.randint(1600, 1700)
        
    else:  # implicit_differentiation
        # Implicit differentiation with partial derivatives
        a = random.choice([2, 3])
        b = random.choice([2, 3])
        c = random.randint(1, 5)
        
        # x^a + y^b + z^b = c, find dz/dx
        # Using implicit: a*x^(a-1) + b*z^(b-1)*dz/dx = 0
        answer = -a * x**(a-1) / (b * z**(b-1))
        answer = sp.simplify(answer)
        
        question = f"Given the implicit