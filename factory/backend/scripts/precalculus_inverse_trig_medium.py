"""
precalculus - inverse_trig (medium)
Generated: 2026-02-11T21:55:01.856807
"""

import sympy as sp
import random
import json

x = sp.Symbol('x')

problem_type = random.choice([
    'basic_eval',
    'basic_eval_fraction',
    'composition',
    'solve_equation',
    'domain_restriction',
    'exact_value_composition'
])

if problem_type == 'basic_eval':
    func_choice = random.choice(['arcsin', 'arccos', 'arctan'])
    
    if func_choice == 'arcsin':
        answer = random.choice([sp.pi/6, sp.pi/4, sp.pi/3, -sp.pi/6, -sp.pi/4, -sp.pi/3, 0])
        input_val = sp.sin(answer)
        question = f"\\arcsin\\left({sp.latex(input_val)}\\right)"
    elif func_choice == 'arccos':
        answer = random.choice([sp.pi/6, sp.pi/4, sp.pi/3, sp.pi/2, 2*sp.pi/3, 3*sp.pi/4, 5*sp.pi/6])
        input_val = sp.cos(answer)
        question = f"\\arccos\\left({sp.latex(input_val)}\\right)"
    else:
        answer = random.choice([sp.pi/6, sp.pi/4, sp.pi/3, -sp.pi/6, -sp.pi/4, -sp.pi/3, 0])
        input_val = sp.tan(answer)
        question = f"\\arctan\\left({sp.latex(input_val)}\\right)"
    
    difficulty = 1250

elif problem_type == 'basic_eval_fraction':
    func_choice = random.choice(['arcsin', 'arccos'])
    
    if func_choice == 'arcsin':
        answer = random.choice([sp.pi/6, -sp.pi/6, sp.pi/4, -sp.pi/4])
        input_val = sp.sin(answer)
        question = f"\\sin^{{-1}}\\left({sp.latex(input_val)}\\right)"
    else:
        answer = random.choice([sp.pi/6, sp.pi/3, sp.pi/4, 5*sp.pi/6])
        input_val = sp.cos(answer)
        question = f"\\cos^{{-1}}\\left({sp.latex(input_val)}\\right)"
    
    difficulty = 1300

elif problem_type == 'composition':
    outer = random.choice(['sin', 'cos', 'tan'])
    inner = random.choice(['arcsin', 'arccos', 'arctan'])
    
    if outer == 'sin' and inner == 'arcsin':
        a = random.randint(2, 9)
        b = random.randint(a+1, 12)
        answer = sp.Rational(a, b)
        question = f"\\sin\\left(\\arcsin\\left(\\frac{{{a}}}{{{b}}}\\right)\\right)"
        difficulty = 1350
    elif outer == 'cos' and inner == 'arccos':
        a = random.randint(1, 7)
        b = random.randint(a+1, 10)
        answer = sp.Rational(a, b)
        question = f"\\cos\\left(\\arccos\\left(\\frac{{{a}}}{{{b}}}\\right)\\right)"
        difficulty = 1350
    elif outer == 'sin' and inner == 'arccos':
        a = random.randint(2, 7)
        b = random.randint(a+1, 10)
        answer = sp.sqrt(b**2 - a**2) / b
        question = f"\\sin\\left(\\arccos\\left(\\frac{{{a}}}{{{b}}}\\right)\\right)"
        difficulty = 1450
    elif outer == 'cos' and inner == 'arcsin':
        a = random.randint(2, 7)
        b = random.randint(a+1, 10)
        answer = sp.sqrt(b**2 - a**2) / b
        question = f"\\cos\\left(\\arcsin\\left(\\frac{{{a}}}{{{b}}}\\right)\\right)"
        difficulty = 1450
    else:
        a = random.randint(1, 5)
        b = random.randint(a+1, 8)
        answer = sp.Rational(a, sp.sqrt(a**2 + b**2))
        question = f"\\sin\\left(\\arctan\\left(\\frac{{{a}}}{{{b}}}\\right)\\right)"
        difficulty = 1500

elif problem_type == 'solve_equation':
    coeff = random.choice([1, 2, 3])
    const = random.choice([sp.pi/6, sp.pi/4, sp.pi/3])
    
    answer = sp.sin(const) / coeff
    question = f"\\text{{Solve for }} x: \\arcsin({coeff}x) = {sp.latex(const)}"
    difficulty = 1400

elif problem_type == 'domain_restriction':
    a = random.randint(2, 6)
    b = random.randint(1, 5)
    
    answer = (1 - b) / a
    question = f"\\text{{Find the value of }} x \\text{{ such that }} \\arcsin({a}x + {b}) = -\\frac{{\\pi}}{{2}}"
    difficulty = 1500

else:
    inner_angle = random.choice([sp.pi/6, sp.pi/4, sp.pi/3])
    outer_func = random.choice(['arcsin', 'arccos'])
    
    if outer_func == 'arcsin':
        inner_val = sp.cos(inner_angle)
        answer = sp.asin(inner_val)
        question = f"\\arcsin\\left(\\cos\\left({sp.latex(inner_angle)}\\right)\\right)"
    else:
        inner_val = sp.sin(inner_angle)
        answer = sp.acos(inner_val)
        question = f"\\arccos\\left(\\sin\\left({sp.latex(inner_angle)}\\right)\\right)"
    
    difficulty = 1550

output = {
    "question_latex": question,
    "answer_key": str(answer),
    "difficulty": difficulty,
    "main_topic": "precalculus",
    "subtopic": "inverse_trig",
    "grading_mode": "equivalent"
}

print(json.dumps(output))