"""
algebra2 - rational_expressions (medium)
Generated: 2026-02-11T21:46:49.600216
"""

import sympy as sp
import random
import json

def generate_rational_expression_problem():
    x = sp.Symbol('x')
    
    problem_type = random.choice([
        'simplify_basic',
        'simplify_factor',
        'add_subtract',
        'multiply_divide',
        'complex_fraction'
    ])
    
    if problem_type == 'simplify_basic':
        # ELO: 1300-1400 - Factor and cancel common terms
        factor = random.randint(2, 5)
        a = random.randint(1, 4) * factor
        b = random.randint(1, 4) * factor
        c = random.randint(2, 6)
        
        numerator = a * x + b
        denominator = factor * c
        
        question = f"\\frac{{{sp.latex(numerator)}}}{{{denominator}}}"
        answer = sp.simplify(numerator / denominator)
        difficulty = 1350
        
    elif problem_type == 'simplify_factor':
        # ELO: 1400-1500 - Factor numerator and denominator, cancel
        common_factor = random.choice([x + random.randint(1, 5), x - random.randint(1, 5)])
        num_factor = random.choice([x + random.randint(1, 5), x - random.randint(1, 5)])
        denom_factor = random.choice([x + random.randint(1, 5), x - random.randint(1, 5)])
        
        numerator = sp.expand(common_factor * num_factor)
        denominator = sp.expand(common_factor * denom_factor)
        
        question = f"\\frac{{{sp.latex(numerator)}}}{{{sp.latex(denominator)}}}"
        answer = sp.simplify(numerator / denominator)
        difficulty = 1450
        
    elif problem_type == 'add_subtract':
        # ELO: 1400-1550 - Add/subtract with different denominators
        a = random.randint(1, 5)
        b = random.randint(1, 5)
        c = random.randint(2, 6)
        d = random.randint(2, 6)
        
        if random.choice([True, False]):
            # Same denominator (easier)
            expr1 = a / (x + c)
            expr2 = b / (x + c)
            op = random.choice(['+', '-'])
            if op == '+':
                question = f"\\frac{{{a}}}{{{sp.latex(x + c)}}} + \\frac{{{b}}}{{{sp.latex(x + c)}}}"
                answer = sp.simplify(expr1 + expr2)
            else:
                question = f"\\frac{{{a}}}{{{sp.latex(x + c)}}} - \\frac{{{b}}}{{{sp.latex(x + c)}}}"
                answer = sp.simplify(expr1 - expr2)
            difficulty = 1380
        else:
            # Different denominators (harder)
            expr1 = a / (x + c)
            expr2 = b / (x + d)
            op = random.choice(['+', '-'])
            if op == '+':
                question = f"\\frac{{{a}}}{{{sp.latex(x + c)}}} + \\frac{{{b}}}{{{sp.latex(x + d)}}}"
                answer = sp.simplify(expr1 + expr2)
            else:
                question = f"\\frac{{{a}}}{{{sp.latex(x + c)}}} - \\frac{{{b}}}{{{sp.latex(x + d)}}}"
                answer = sp.simplify(expr1 - expr2)
            difficulty = 1500
            
    elif problem_type == 'multiply_divide':
        # ELO: 1400-1500 - Multiply or divide rational expressions
        a = random.randint(1, 4)
        b = random.randint(1, 5)
        c = random.randint(1, 4)
        d = random.randint(1, 5)
        
        expr1_num = a * x
        expr1_den = x + b
        expr2_num = x + d
        expr2_den = c * x
        
        if random.choice([True, False]):
            # Multiplication
            question = f"\\frac{{{sp.latex(expr1_num)}}}{{{sp.latex(expr1_den)}}} \\cdot \\frac{{{sp.latex(expr2_num)}}}{{{sp.latex(expr2_den)}}}"
            answer = sp.simplify((expr1_num / expr1_den) * (expr2_num / expr2_den))
            difficulty = 1420
        else:
            # Division
            question = f"\\frac{{{sp.latex(expr1_num)}}}{{{sp.latex(expr1_den)}}} \\div \\frac{{{sp.latex(expr2_num)}}}{{{sp.latex(expr2_den)}}}"
            answer = sp.simplify((expr1_num / expr1_den) / (expr2_num / expr2_den))
            difficulty = 1480
            
    else:  # complex_fraction
        # ELO: 1500-1600 - Complex fraction (fraction divided by fraction)
        a = random.randint(1, 4)
        b = random.randint(1, 5)
        c = random.randint(1, 4)
        d = random.randint(1, 5)
        
        top_expr = a / x
        bottom_expr = c / (x + b)
        
        question = f"\\frac{{\\frac{{{a}}}{{{sp.latex(x)}}}}}{{\\frac{{{c}}}{{{sp.latex(x + b)}}}}}"
        answer = sp.simplify(top_expr / bottom_expr)
        difficulty = 1550
    
    result = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "rational_expressions",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_rational_expression_problem()
print(json.dumps(problem))