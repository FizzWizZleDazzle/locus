"""
algebra1 - graphing_lines (hard)
Generated: 2026-02-11T21:29:35.711477
"""

import sympy as sp
import random
import json

def generate_problem():
    x, y = sp.symbols('x y')
    
    problem_type = random.choice([
        'perpendicular_through_point',
        'parallel_through_point',
        'intersection_of_lines',
        'line_through_two_points_complex',
        'distance_and_line',
        'reflection_across_line'
    ])
    
    if problem_type == 'perpendicular_through_point':
        # Given line and point, find perpendicular line
        m1 = random.choice([2, 3, -2, -3, sp.Rational(2,3), sp.Rational(3,4), sp.Rational(-3,2)])
        b1 = random.randint(-5, 5)
        
        px = random.randint(-4, 4)
        py = random.randint(-4, 4)
        
        m2 = sp.Rational(-1, m1)
        b2 = py - m2 * px
        
        answer = sp.simplify(m2 * x + b2)
        
        question = f"Find the equation of the line perpendicular to $y = {sp.latex(m1 * x + b1)}$ that passes through the point $({px}, {py})$. Express your answer in the form $y = mx + b$."
        
        return question, str(answer), 1650
    
    elif problem_type == 'parallel_through_point':
        # Given line in standard form, find parallel line through point
        a = random.randint(2, 5)
        b = random.randint(2, 5)
        c = random.randint(-8, 8)
        
        px = random.randint(-4, 4)
        py = random.randint(-4, 4)
        
        # Parallel line: same a, b coefficients
        c_new = a * px + b * py
        
        answer_expr = sp.Eq(a*x + b*y, c_new)
        
        question = f"Find the equation of the line parallel to ${a}x + {b}y = {c}$ that passes through $({px}, {py})$. Express your answer in standard form $Ax + By = C$ where $A$, $B$, and $C$ are integers."
        
        return question, f"Eq({a}*x + {b}*y, {c_new})", 1620
    
    elif problem_type == 'intersection_of_lines':
        # Find intersection, then find line through intersection and another point
        m1 = random.randint(1, 4)
        b1 = random.randint(-3, 3)
        
        m2 = random.randint(-4, -1)
        b2 = random.randint(-3, 3)
        
        # Intersection point
        x_int = sp.Rational(b2 - b1, m1 - m2)
        y_int = m1 * x_int + b1
        
        # Another point
        px = random.randint(-5, 5)
        py = random.randint(-5, 5)
        
        if px != x_int:
            m_new = sp.simplify((py - y_int) / (px - x_int))
            b_new = sp.simplify(y_int - m_new * x_int)
            
            answer = sp.simplify(m_new * x + b_new)
            
            question = f"Lines $y = {m1}x + {b1}$ and $y = {m2}x + {b2}$ intersect at point $P$. Find the equation of the line passing through $P$ and the point $({px}, {py})$ in slope-intercept form."
            
            return question, str(answer), 1720
    
    elif problem_type == 'line_through_two_points_complex':
        # Two points with fractions/complex coordinates
        x1 = sp.Rational(random.randint(-10, 10), random.choice([2, 3, 4]))
        y1 = sp.Rational(random.randint(-10, 10), random.choice([2, 3, 4]))
        x2 = sp.Rational(random.randint(-10, 10), random.choice([2, 3, 4]))
        y2 = sp.Rational(random.randint(-10, 10), random.choice([2, 3, 4]))
        
        if x1 != x2:
            m = sp.simplify((y2 - y1) / (x2 - x1))
            b = sp.simplify(y1 - m * x1)
            
            answer = sp.simplify(m * x + b)
            
            question = f"Find the equation of the line passing through $\\left({sp.latex(x1)}, {sp.latex(y1)}\\right)$ and $\\left({sp.latex(x2)}, {sp.latex(y2)}\\right)$ in slope-intercept form."
            
            return question, str(answer), 1680
    
    elif problem_type == 'distance_and_line':
        # Find line at specific distance from origin with given slope
        m = random.choice([sp.Rational(3,4), sp.Rational(4,3), sp.Rational(-3,4), sp.Rational(5,12)])
        d = random.randint(2, 5)
        
        # Distance from origin to line y = mx + b is |b|/sqrt(1+m^2)
        b_val = d * sp.sqrt(1 + m**2)
        b_pos = sp.simplify(b_val)
        
        answer = sp.simplify(m * x + b_pos)
        
        question = f"Find the equation of the line with slope ${sp.latex(m)}$ that is at a distance of ${d}$ units from the origin (choose the line with positive $y$-intercept). Express in slope-intercept form."
        
        return question, str(answer), 1780
    
    else:  # reflection_across_line
        # Find reflection of a point across y=x or y=-x, then line through it and another point
        px = random.randint(1, 5)
        py = random.randint(-5, -1)
        
        # Reflect across y = x
        rx, ry = py, px
        
        qx = random.randint(-3, 3)
        qy = random.randint(-3, 3)
        
        if qx != rx:
            m = sp.Rational(qy - ry, qx - rx)
            b = ry - m * rx
            
            answer = sp.simplify(m * x + b)
            
            question = f"Point $P = ({px}, {py})$ is reflected across the line $y = x$ to obtain point $P'$. Find the equation of the line through $P'$ and $Q = ({qx}, {qy})$ in slope-intercept form."
            
            return question, str(answer), 1750
    
    # Fallback
    return generate_problem()

question, answer, difficulty = generate_problem()

output = {
    "question_latex": question,
    "answer_key": answer,
    "difficulty": difficulty,
    "main_topic": "algebra1",
    "subtopic": "graphing_lines",
    "grading_mode": "equivalent"
}

print(json.dumps(output))