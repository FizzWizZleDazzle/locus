"""
precalculus - trigonometric_identities (easy)
Generated: 2026-02-11T21:53:36.171475
"""

import sympy as sp
import random
import json

def generate_trig_identity_problem():
    x = sp.Symbol('x')
    
    problem_type = random.choice([
        'basic_identity',
        'simple_simplify',
        'pythagorean_basic',
        'angle_sum_simple',
        'double_angle_direct'
    ])
    
    if problem_type == 'basic_identity':
        # 1000-1100: Direct recall of basic identities
        identities = [
            (sp.sin(x)**2 + sp.cos(x)**2, sp.Integer(1), "\\sin^2(x) + \\cos^2(x)", 1050),
            (sp.tan(x), sp.sin(x)/sp.cos(x), "\\tan(x)", 1080),
            (sp.sec(x), 1/sp.cos(x), "\\sec(x)", 1100),
            (sp.csc(x), 1/sp.sin(x), "\\csc(x)", 1100),
            (sp.cot(x), sp.cos(x)/sp.sin(x), "\\cot(x)", 1080),
        ]
        answer, expression, question_latex, elo = random.choice(identities)
        question = f"Simplify: ${question_latex}$"
        
    elif problem_type == 'simple_simplify':
        # 1100-1200: One-step simplification
        choices = [
            (sp.Integer(1) - sp.cos(x)**2, sp.sin(x)**2, "1 - \\cos^2(x)", 1150),
            (sp.Integer(1) - sp.sin(x)**2, sp.cos(x)**2, "1 - \\sin^2(x)", 1150),
            (sp.sin(x)/sp.cos(x), sp.tan(x), "\\frac{\\sin(x)}{\\cos(x)}", 1120),
            (sp.cos(x)/sp.sin(x), sp.cot(x), "\\frac{\\cos(x)}{\\sin(x)}", 1120),
            (1/sp.cos(x), sp.sec(x), "\\frac{1}{\\cos(x)}", 1100),
            (1/sp.sin(x), sp.csc(x), "\\frac{1}{\\sin(x)}", 1100),
        ]
        expression, answer, question_latex, elo = random.choice(choices)
        question = f"Simplify: ${question_latex}$"
        
    elif problem_type == 'pythagorean_basic':
        # 1150-1250: Basic Pythagorean identity manipulation
        choices = [
            (sp.tan(x)**2 + 1, sp.sec(x)**2, "\\tan^2(x) + 1", 1200),
            (sp.Integer(1) + sp.cot(x)**2, sp.csc(x)**2, "1 + \\cot^2(x)", 1200),
            (sp.sec(x)**2 - 1, sp.tan(x)**2, "\\sec^2(x) - 1", 1220),
            (sp.csc(x)**2 - 1, sp.cot(x)**2, "\\csc^2(x) - 1", 1220),
        ]
        expression, answer, question_latex, elo = random.choice(choices)
        question = f"Simplify: ${question_latex}$"
        
    elif problem_type == 'angle_sum_simple':
        # 1200-1300: Direct application of sum formulas with specific angles
        angle1 = random.choice([30, 45, 60])
        angle2 = random.choice([30, 45, 60])
        
        a1_rad = sp.pi * angle1 / 180
        a2_rad = sp.pi * angle2 / 180
        
        func = random.choice(['sin', 'cos'])
        
        if func == 'sin':
            answer = sp.sin(a1_rad + a2_rad)
            question = f"Simplify using sum formulas: $\\sin({angle1}^\\circ + {angle2}^\\circ)$"
            elo = 1250
        else:
            answer = sp.cos(a1_rad + a2_rad)
            question = f"Simplify using sum formulas: $\\cos({angle1}^\\circ + {angle2}^\\circ)$"
            elo = 1250
            
    else:  # double_angle_direct
        # 1200-1300: Direct double angle evaluation
        angle = random.choice([15, 30, 45, 60])
        a_rad = sp.pi * angle / 180
        
        func = random.choice(['sin', 'cos'])
        
        if func == 'sin':
            answer = sp.sin(2 * a_rad)
            question = f"Simplify using double angle formulas: $\\sin(2 \\cdot {angle}^\\circ)$"
            elo = 1280
        else:
            answer = sp.cos(2 * a_rad)
            question = f"Simplify using double angle formulas: $\\cos(2 \\cdot {angle}^\\circ)$"
            elo = 1280
    
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