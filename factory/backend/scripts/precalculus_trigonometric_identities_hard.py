"""
precalculus - trigonometric_identities (hard)
Generated: 2026-02-11T21:54:19.948766
"""

import sympy as sp
import random
import json

def generate_problem():
    x = sp.Symbol('x')
    
    problem_type = random.randint(1, 8)
    
    if problem_type == 1:
        # Simplify complex expression with multiple angle formulas
        # ELO: 1650-1750
        angle_mult = random.choice([2, 3])
        func_choice = random.choice(['sin', 'cos'])
        
        if func_choice == 'sin':
            # sin(2x) = 2sin(x)cos(x)
            if angle_mult == 2:
                answer = 2 * sp.sin(x) * sp.cos(x)
                question = r"\sin(2x)"
                elo = 1650
            else:
                # sin(3x) = 3sin(x) - 4sin^3(x)
                answer = 3*sp.sin(x) - 4*sp.sin(x)**3
                question = r"\sin(3x)"
                elo = 1750
        else:
            # cos(2x) = cos^2(x) - sin^2(x)
            if angle_mult == 2:
                answer = sp.cos(x)**2 - sp.sin(x)**2
                question = r"\cos(2x)"
                elo = 1650
            else:
                # cos(3x) = 4cos^3(x) - 3cos(x)
                answer = 4*sp.cos(x)**3 - 3*sp.cos(x)
                question = r"\cos(3x)"
                elo = 1750
        
        question_latex = f"Simplify {question} in terms of \\sin(x) and \\cos(x)"
        
    elif problem_type == 2:
        # Prove/simplify identity with multiple steps
        # ELO: 1700-1800
        identity_type = random.randint(1, 3)
        
        if identity_type == 1:
            # (1 - cos(2x))/(sin(2x)) = tan(x)
            question_latex = r"\frac{1 - \cos(2x)}{\sin(2x)}"
            answer = sp.tan(x)
            elo = 1750
        elif identity_type == 2:
            # sin(x)/(1+cos(x)) = (1-cos(x))/sin(x)
            question_latex = r"\frac{\sin(x)}{1 + \cos(x)}"
            answer = (1 - sp.cos(x))/sp.sin(x)
            elo = 1700
        else:
            # (sin(x) + cos(x))^2 = 1 + sin(2x)
            question_latex = r"(\sin(x) + \cos(x))^2"
            answer = 1 + sp.sin(2*x)
            elo = 1680
        
        question_latex = f"Simplify: {question_latex}"
        
    elif problem_type == 3:
        # Product-to-sum or sum-to-product formulas
        # ELO: 1750-1850
        a = random.choice([2, 3, 4, 5])
        b = random.choice([1, 2, 3]) if a > 2 else random.choice([2, 3, 4])
        
        formula_type = random.choice(['product', 'sum'])
        
        if formula_type == 'product':
            # sin(ax)cos(bx) = (1/2)[sin((a+b)x) + sin((a-b)x)]
            answer = sp.Rational(1, 2) * (sp.sin((a+b)*x) + sp.sin((a-b)*x))
            question_latex = f"Express \\sin({a}x)\\cos({b}x) as a sum"
            elo = 1780
        else:
            # sin(ax) + sin(bx) = 2sin((a+b)x/2)cos((a-b)x/2)
            answer = 2 * sp.sin((a+b)*x/2) * sp.cos((a-b)*x/2)
            question_latex = f"Express \\sin({a}x) + \\sin({b}x) as a product"
            elo = 1800
        
    elif problem_type == 4:
        # Simplify with half-angle formulas
        # ELO: 1700-1800
        func = random.choice(['sin', 'cos', 'tan'])
        
        if func == 'sin':
            # sin^2(x/2) = (1-cos(x))/2
            answer = (1 - sp.cos(x))/2
            question_latex = r"\sin^2\left(\frac{x}{2}\right)"
            elo = 1720
        elif func == 'cos':
            # cos^2(x/2) = (1+cos(x))/2
            answer = (1 + sp.cos(x))/2
            question_latex = r"\cos^2\left(\frac{x}{2}\right)"
            elo = 1720
        else:
            # tan(x/2) = sin(x)/(1+cos(x))
            answer = sp.sin(x)/(1 + sp.cos(x))
            question_latex = r"\tan\left(\frac{x}{2}\right)"
            elo = 1750
        
        question_latex = f"Express {question_latex} in terms of \\sin(x) and/or \\cos(x)"
        
    elif problem_type == 5:
        # Complex fraction simplification
        # ELO: 1750-1850
        version = random.randint(1, 3)
        
        if version == 1:
            # (1 + tan^2(x)) = sec^2(x)
            question_latex = r"1 + \tan^2(x)"
            answer = sp.sec(x)**2
            elo = 1650
        elif version == 2:
            # (sec(x) - tan(x))(sec(x) + tan(x)) = 1
            question_latex = r"(\sec(x) - \tan(x))(\sec(x) + \tan(x))"
            answer = sp.Integer(1)
            elo = 1700
        else:
            # (1 - sin(x))(1 + sin(x)) = cos^2(x)
            question_latex = r"(1 - \sin(x))(1 + \sin(x))"
            answer = sp.cos(x)**2
            elo = 1670
        
        question_latex = f"Simplify: {question_latex}"
        
    elif problem_type == 6:
        # Verify complex identity
        # ELO: 1800-1900
        a = random.choice([2, 3])
        
        identity_choice = random.randint(1, 3)
        
        if identity_choice == 1:
            # tan(x) + cot(x) = 2csc(2x)
            question_latex = r"\tan(x) + \cot(x)"
            answer = 2*sp.csc(2*x)
            elo = 1820
        elif identity_choice == 2:
            # (cos(x) - sin(x))^2 = 1 - sin(2x)
            question_latex = r"(\cos(x) - \sin(x))^2"
            answer = 1 - sp.sin(2*x)
            elo = 1680
        else:
            # sin^4(x) - cos^4(x) = -cos(2x)
            question_latex = r"\sin^4(x) - \cos^4(x)"
            answer = -sp