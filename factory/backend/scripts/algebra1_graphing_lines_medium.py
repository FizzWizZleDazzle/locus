"""
algebra1 - graphing_lines (medium)
Generated: 2026-02-11T21:29:09.019595
"""

import random
import json
from sympy import symbols, Eq, simplify, latex, Rational

def generate_graphing_lines_problem():
    x, y = symbols('x y')
    problem_type = random.choice([
        'slope_intercept_from_points',
        'point_slope_form',
        'standard_to_slope_intercept',
        'find_equation_given_slope_point',
        'parallel_line',
        'perpendicular_line'
    ])
    
    if problem_type == 'slope_intercept_from_points':
        x1 = random.randint(-5, 5)
        y1 = random.randint(-5, 5)
        m = random.choice([Rational(i, j) for i in range(-4, 5) if i != 0 for j in [1, 2, 3, 4] if i % j != 0] + list(range(-3, 4)))
        if m == 0:
            m = random.choice([-2, -1, 1, 2])
        y2 = y1 + m * random.randint(1, 4)
        x2 = x1 + random.randint(1, 4)
        actual_m = (y2 - y1) / (x2 - x1)
        b = y1 - actual_m * x1
        
        question = f"Find the equation of the line passing through the points $({x1}, {y1})$ and $({x2}, {y2})$. Write your answer in slope-intercept form $y = mx + b$."
        answer = f"y = {latex(simplify(actual_m))}*x + {latex(simplify(b))}"
        difficulty = 1400
        
    elif problem_type == 'point_slope_form':
        x1 = random.randint(-5, 5)
        y1 = random.randint(-5, 5)
        m = random.choice([Rational(i, j) for i in range(-3, 4) if i != 0 for j in [1, 2]] + list(range(-3, 4)))
        if m == 0:
            m = random.choice([-2, -1, 1, 2])
        b = y1 - m * x1
        
        question = f"Write the equation of the line with slope $m = {latex(m)}$ passing through the point $({x1}, {y1})$ in slope-intercept form."
        answer = f"y = {latex(m)}*x + {latex(simplify(b))}"
        difficulty = 1300
        
    elif problem_type == 'standard_to_slope_intercept':
        m = random.choice([Rational(i, j) for i in range(-4, 5) if i != 0 for j in [1, 2, 3]] + list(range(-3, 4)))
        if m == 0:
            m = random.choice([-2, -1, 1, 2])
        b = random.randint(-6, 6)
        A = random.randint(1, 5)
        B = random.randint(1, 5)
        C = A * b + B * m * (-1)
        
        question = f"Convert the equation ${A}x + {B}y = {latex(simplify(C))}$ to slope-intercept form."
        slope = Rational(-A, B)
        intercept = Rational(C, B)
        answer = f"y = {latex(simplify(slope))}*x + {latex(simplify(intercept))}"
        difficulty = 1350
        
    elif problem_type == 'find_equation_given_slope_point':
        m = random.choice([Rational(i, j) for i in range(-3, 4) if i != 0 for j in [2, 3]] + list(range(-2, 3)))
        if m == 0:
            m = 1
        x1 = random.randint(-4, 4)
        y1 = random.randint(-4, 4)
        b = y1 - m * x1
        
        question = f"Find the equation of the line with slope ${latex(m)}$ that passes through $({x1}, {y1})$. Express in slope-intercept form."
        answer = f"y = {latex(m)}*x + {latex(simplify(b))}"
        difficulty = 1300
        
    elif problem_type == 'parallel_line':
        m = random.choice([Rational(i, j) for i in range(-3, 4) if i != 0 for j in [2, 3]] + list(range(-2, 3)))
        if m == 0:
            m = 2
        b1 = random.randint(-5, 5)
        x1 = random.randint(-4, 4)
        y1 = random.randint(-4, 4)
        b2 = y1 - m * x1
        
        question = f"Find the equation of the line parallel to $y = {latex(m)}x + {latex(b1)}$ that passes through the point $({x1}, {y1})$."
        answer = f"y = {latex(m)}*x + {latex(simplify(b2))}"
        difficulty = 1500
        
    elif problem_type == 'perpendicular_line':
        m1 = random.choice([Rational(i, 1) for i in range(-3, 4) if i != 0])
        if m1 == 0:
            m1 = 2
        m2 = Rational(-1, m1)
        b1 = random.randint(-5, 5)
        x1 = random.randint(-4, 4)
        y1 = random.randint(-4, 4)
        b2 = y1 - m2 * x1
        
        question = f"Find the equation of the line perpendicular to $y = {latex(m1)}x + {latex(b1)}$ that passes through the point $({x1}, {y1})$."
        answer = f"y = {latex(m2)}*x + {latex(simplify(b2))}"
        difficulty = 1550
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "graphing_lines",
        "grading_mode": "equivalent"
    }

problem = generate_graphing_lines_problem()
print(json.dumps(problem))