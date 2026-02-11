"""
calculus - applications_of_derivatives (easy)
Generated: 2026-02-11T22:01:01.485032
"""

import sympy as sp
import random
import json

x = sp.Symbol('x')

problem_type = random.choice([
    'critical_points_polynomial',
    'increasing_decreasing_simple',
    'local_max_min_simple',
    'rate_of_change_simple'
])

if problem_type == 'critical_points_polynomial':
    a = random.choice([1, 2, 3])
    b = random.choice([2, 4, 6, 8])
    
    critical_point = random.choice([1, 2, -1, -2])
    
    derivative = a * (x - critical_point)
    f = sp.integrate(derivative, x) + random.randint(-5, 5)
    
    question = f"Find the critical point(s) of \\( f(x) = {sp.latex(f)} \\)."
    answer = str(critical_point)
    difficulty = 1100

elif problem_type == 'increasing_decreasing_simple':
    a = random.choice([1, 2, 3, -1, -2, -3])
    b = random.randint(-10, 10)
    
    f = a * x + b
    derivative = sp.diff(f, x)
    
    if a > 0:
        answer_text = "increasing"
    else:
        answer_text = "decreasing"
    
    question = f"Is the function \\( f(x) = {sp.latex(f)} \\) increasing or decreasing?"
    answer = answer_text
    difficulty = 1050

elif problem_type == 'local_max_min_simple':
    a = random.choice([1, 2, -1, -2])
    critical_x = random.choice([0, 1, 2, -1])
    
    f = a * (x - critical_x)**2 + random.randint(-5, 5)
    
    if a > 0:
        answer_type = "minimum"
    else:
        answer_type = "maximum"
    
    question = f"Does \\( f(x) = {sp.latex(f)} \\) have a local maximum or minimum at \\( x = {critical_x} \\)?"
    answer = answer_type
    difficulty = 1150

else:  # rate_of_change_simple
    a = random.choice([2, 3, 4, 5])
    b = random.randint(-10, 10)
    x0 = random.choice([1, 2, 3, -1])
    
    f = a * x + b
    derivative = sp.diff(f, x)
    rate = derivative.subs(x, x0)
    
    question = f"Find the rate of change of \\( f(x) = {sp.latex(f)} \\) at \\( x = {x0} \\)."
    answer = str(rate)
    difficulty = 1100

output = {
    "question_latex": question,
    "answer_key": answer,
    "difficulty": difficulty,
    "main_topic": "calculus",
    "subtopic": "applications_of_derivatives",
    "grading_mode": "equivalent"
}

print(json.dumps(output))