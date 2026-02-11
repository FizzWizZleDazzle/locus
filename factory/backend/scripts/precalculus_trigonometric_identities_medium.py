"""
precalculus - trigonometric_identities (medium)
Generated: 2026-02-11T21:53:55.752778
"""

import sympy as sp
import random
import json

def generate_trig_identity_problem():
    x = sp.Symbol('x')
    
    problem_types = [
        'simplify_basic',
        'simplify_pythagorean',
        'simplify_double_angle',
        'verify_identity',
        'simplify_compound'
    ]
    
    problem_type = random.choice(problem_types)
    
    if problem_type == 'simplify_basic':
        # 1200-1400: Simple one or two identity applications
        options = [
            (sp.sin(x)**2 + sp.cos(x)**2, sp.Integer(1), 1250),
            (sp.tan(x) * sp.cos(x), sp.sin(x), 1300),
            (sp.sin(x) / sp.cos(x), sp.tan(x), 1300),
            (sp.cos(x) / sp.sin(x), sp.cot(x), 1300),
            (sp.sec(x) * sp.cos(x), sp.Integer(1), 1250),
            (sp.csc(x) * sp.sin(x), sp.Integer(1), 1250),
        ]
        expr, answer, elo = random.choice(options)
        question = f"\\text{{Simplify: }} {sp.latex(expr)}"
        
    elif problem_type == 'simplify_pythagorean':
        # 1300-1500: Pythagorean identity variations
        options = [
            (1 - sp.cos(x)**2, sp.sin(x)**2, 1350),
            (1 - sp.sin(x)**2, sp.cos(x)**2, 1350),
            (sp.tan(x)**2 + 1, sp.sec(x)**2, 1400),
            (sp.sec(x)**2 - 1, sp.tan(x)**2, 1400),
            (sp.cot(x)**2 + 1, sp.csc(x)**2, 1400),
            (sp.csc(x)**2 - 1, sp.cot(x)**2, 1400),
            (sp.sin(x)**2 / (1 - sp.cos(x)**2), sp.Integer(1), 1450),
            ((1 - sp.sin(x)**2) / sp.cos(x)**2, sp.Integer(1), 1450),
        ]
        expr, answer, elo = random.choice(options)
        question = f"\\text{{Simplify: }} {sp.latex(expr)}"
        
    elif problem_type == 'simplify_double_angle':
        # 1400-1600: Double angle formulas
        options = [
            (2 * sp.sin(x) * sp.cos(x), sp.sin(2*x), 1450),
            (sp.cos(x)**2 - sp.sin(x)**2, sp.cos(2*x), 1450),
            (2 * sp.cos(x)**2 - 1, sp.cos(2*x), 1500),
            (1 - 2 * sp.sin(x)**2, sp.cos(2*x), 1500),
            (sp.sin(2*x) / (2 * sp.cos(x)), sp.sin(x), 1550),
        ]
        expr, answer, elo = random.choice(options)
        question = f"\\text{{Simplify: }} {sp.latex(expr)}"
        
    elif problem_type == 'verify_identity':
        # 1400-1600: Multi-step simplifications
        options = [
            (sp.tan(x) * sp.csc(x), sp.sec(x), 1450),
            (sp.sin(x) * sp.sec(x), sp.tan(x), 1450),
            ((sp.sin(x)**2 - sp.cos(x)**2) / (sp.sin(x) - sp.cos(x)), sp.sin(x) + sp.cos(x), 1550),
            (sp.tan(x) / sp.sec(x), sp.sin(x), 1400),
            (sp.cot(x) / sp.csc(x), sp.cos(x), 1400),
            ((1 + sp.tan(x)**2) / sp.tan(x)**2, sp.csc(x)**2, 1500),
        ]
        expr, answer, elo = random.choice(options)
        question = f"\\text{{Simplify: }} {sp.latex(expr)}"
        
    else:  # simplify_compound
        # 1450-1600: More complex expressions
        options = [
            (sp.sin(x)**2 * sp.tan(x)**2 + sp.sin(x)**2, sp.tan(x)**2, 1500),
            ((sp.sec(x)**2 - 1) / sp.sec(x)**2, sp.sin(x)**2, 1500),
            (sp.cos(x)**2 * (sp.tan(x)**2 + 1), sp.Integer(1), 1550),
            (sp.sin(x)**2 * (sp.cot(x)**2 + 1), sp.Integer(1), 1550),
            ((sp.tan(x) + sp.cot(x)) * sp.sin(x) * sp.cos(x), sp.Integer(1), 1580),
        ]
        expr, answer, elo = random.choice(options)
        question = f"\\text{{Simplify: }} {sp.latex(expr)}"
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": elo,
        "main_topic": "precalculus",
        "subtopic": "trigonometric_identities",
        "grading_mode": "equivalent"
    }

problem = generate_trig_identity_problem()
print(json.dumps(problem))