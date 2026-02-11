"""
algebra1 - graphing_lines (easy)
Generated: 2026-02-11T21:28:47.052674
"""

import random
import json
from sympy import symbols, latex, simplify, Rational

def generate_problem():
    x, y = symbols('x y')
    problem_type = random.choice([
        'find_slope_two_points',
        'identify_slope_from_equation',
        'identify_y_intercept',
        'point_on_line',
        'slope_from_graph_description'
    ])
    
    if problem_type == 'find_slope_two_points':
        x1 = random.randint(-5, 5)
        y1 = random.randint(-5, 5)
        
        slope_num = random.choice([-3, -2, -1, 1, 2, 3])
        slope_den = random.choice([1, 2])
        slope = Rational(slope_num, slope_den)
        
        dx = random.choice([1, 2, 3, 4])
        x2 = x1 + dx
        y2 = y1 + slope * dx
        
        question = f"Find the slope of the line passing through the points $({x1}, {y1})$ and $({x2}, {y2})$."
        answer = slope
        difficulty = 1150
        
    elif problem_type == 'identify_slope_from_equation':
        m = random.choice([-4, -3, -2, -1, 1, 2, 3, 4])
        b = random.randint(-6, 6)
        
        question = f"What is the slope of the line $y = {latex(m*x + b)}$?"
        answer = m
        difficulty = 1050
        
    elif problem_type == 'identify_y_intercept':
        m = random.choice([-3, -2, -1, 1, 2, 3])
        b = random.randint(-8, 8)
        
        question = f"What is the y-intercept of the line $y = {latex(m*x + b)}$?"
        answer = b
        difficulty = 1000
        
    elif problem_type == 'point_on_line':
        m = random.choice([-2, -1, 1, 2, 3])
        b = random.randint(-5, 5)
        
        x_val = random.randint(-3, 3)
        y_val = m * x_val + b
        
        question = f"Does the point $({x_val}, {y_val})$ lie on the line $y = {latex(m*x + b)}$? Answer 'yes' or 'no'."
        answer = "yes"
        difficulty = 1200
        
    else:  # slope_from_graph_description
        m = random.choice([-2, -1, 1, 2])
        b = random.randint(-4, 4)
        
        rise = m * 2
        run = 2
        
        question = f"A line has a slope of ${latex(m)}$ and passes through the point $(0, {b})$. Write the equation of the line in slope-intercept form."
        answer = simplify(m*x + b)
        difficulty = 1250
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "graphing_lines",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))