"""
algebra2 - rational_expressions (hard)
Generated: 2026-02-11T21:47:11.403953
"""

import sympy as sp
import random
import json

x = sp.Symbol('x')

def generate_rational_expression_problem():
    problem_type = random.choice([
        'simplify_complex',
        'solve_equation',
        'add_subtract_complex',
        'compound_fraction'
    ])
    
    if problem_type == 'simplify_complex':
        # Complex rational expression simplification
        # Start with simplified form, work backward
        a = random.randint(1, 4)
        b = random.randint(1, 5)
        c = random.randint(1, 4)
        
        # Answer: (ax + b)/(cx + d)
        d = random.randint(1, 5)
        while d == b * c // a if a != 0 else False:
            d = random.randint(1, 5)
        
        answer = (a*x + b) / (c*x + d)
        
        # Create complex numerator and denominator by multiplying by (ex + f)
        e = random.randint(1, 3)
        f = random.randint(1, 4)
        
        numerator = sp.expand((a*x + b) * (e*x + f))
        denominator = sp.expand((c*x + d) * (e*x + f))
        
        question = f"\\text{{Simplify: }} \\frac{{{sp.latex(numerator)}}}{{{sp.latex(denominator)}}}"
        answer_key = str(answer)
        difficulty = 1650
        
    elif problem_type == 'solve_equation':
        # Solve rational equation with multiple terms
        # Pick answer first
        sol = random.randint(-5, 5)
        while sol == 0:
            sol = random.randint(-5, 5)
        
        a = random.randint(1, 4)
        b = random.randint(1, 6)
        c = random.randint(1, 4)
        d = random.randint(1, 6)
        
        # Create equation: a/(x-p) + b/(x-q) = c
        p = random.randint(-3, 3)
        q = random.randint(-3, 3)
        while q == p or sol == p or sol == q:
            q = random.randint(-3, 3)
        
        # Calculate what c should be for our solution
        lhs = a/(sol - p) + b/(sol - q)
        c_val = float(lhs)
        
        if abs(c_val) < 10 and c_val != 0:
            question = f"\\text{{Solve for }} x: \\frac{{{a}}}{{{sp.latex(x - p)}}} + \\frac{{{b}}}{{{sp.latex(x - q)}}} = {sp.latex(sp.nsimplify(c_val))}"
            answer_key = str(sol)
            difficulty = 1750
        else:
            # Fallback to simpler equation
            m = random.randint(2, 5)
            n = random.randint(1, 8)
            sol = random.randint(1, 6)
            p = random.randint(-4, 4)
            while sol == p:
                p = random.randint(-4, 4)
            
            rhs = m * sol + n
            question = f"\\text{{Solve for }} x: \\frac{{{m}x + {n}}}{{{sp.latex(x - p)}}} = {rhs}"
            answer_key = str(sol)
            difficulty = 1700
            
    elif problem_type == 'add_subtract_complex':
        # Add/subtract with different denominators requiring factoring
        a = random.randint(1, 3)
        b = random.randint(1, 4)
        c = random.randint(1, 3)
        
        # Denominators that share a common factor
        p = random.randint(1, 4)
        q = random.randint(1, 4)
        r = random.randint(1, 3)
        
        # First fraction: a/(x+p), second: b/((x+p)(x+q))
        expr1 = a / (x + p)
        expr2 = b / ((x + p) * (x + q))
        
        combined = expr1 + expr2
        answer = sp.simplify(combined)
        
        denom2_expanded = sp.expand((x + p) * (x + q))
        
        question = f"\\text{{Simplify: }} \\frac{{{a}}}{{{sp.latex(x + p)}}} + \\frac{{{b}}}{{{sp.latex(denom2_expanded)}}}"
        answer_key = str(answer)
        difficulty = 1680
        
    else:  # compound_fraction
        # Complex fraction (fraction within fraction)
        a = random.randint(1, 4)
        b = random.randint(1, 5)
        c = random.randint(1, 3)
        d = random.randint(1, 4)
        e = random.randint(1, 3)
        
        # Answer form: (a*x + b)/(c*x + d)
        answer = (a*x + b) / (c*x + d)
        
        # Create compound: (a + b/x) / (c + d/x)
        numerator_part = a + b/x
        denominator_part = c + d/x
        
        question = f"\\text{{Simplify: }} \\frac{{{sp.latex(numerator_part)}}}{{{sp.latex(denominator_part)}}}"
        answer_key = str(answer)
        difficulty = 1720
    
    result = {
        "question_latex": question,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "rational_expressions",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_rational_expression_problem()
print(json.dumps(problem))