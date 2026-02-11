"""
algebra2 - conic_sections (easy)
Generated: 2026-02-11T21:50:34.758154
"""

import sympy as sp
import random
import json

x, y = sp.symbols('x y')

problem_type = random.choice(['circle_center_radius', 'circle_equation', 'parabola_vertex', 'ellipse_center'])

if problem_type == 'circle_center_radius':
    h = random.randint(-5, 5)
    k = random.randint(-5, 5)
    r = random.randint(2, 7)
    
    equation = sp.Eq((x - h)**2 + (y - k)**2, r**2)
    
    question = f"Find the center and radius of the circle: ${sp.latex(equation)}$"
    answer = f"({h}, {k}), {r}"
    difficulty = 1100

elif problem_type == 'circle_equation':
    h = random.randint(-4, 4)
    k = random.randint(-4, 4)
    r = random.randint(2, 6)
    
    question = f"Write the equation of a circle with center $({h}, {k})$ and radius ${r}$"
    answer = sp.latex(sp.Eq((x - h)**2 + (y - k)**2, r**2))
    difficulty = 1150

elif problem_type == 'parabola_vertex':
    h = random.randint(-5, 5)
    k = random.randint(-5, 5)
    a = random.choice([1, -1, 2, -2])
    
    if random.choice([True, False]):
        equation = sp.Eq(y, a*(x - h)**2 + k)
        question = f"Find the vertex of the parabola: ${sp.latex(equation)}$"
        answer = f"({h}, {k})"
    else:
        equation = sp.Eq(x, a*(y - k)**2 + h)
        question = f"Find the vertex of the parabola: ${sp.latex(equation)}$"
        answer = f"({h}, {k})"
    
    difficulty = 1200

elif problem_type == 'ellipse_center':
    h = random.randint(-4, 4)
    k = random.randint(-4, 4)
    a_sq = random.choice([4, 9, 16, 25])
    b_sq = random.choice([4, 9, 16, 25])
    
    while a_sq == b_sq:
        b_sq = random.choice([4, 9, 16, 25])
    
    equation = sp.Eq((x - h)**2/a_sq + (y - k)**2/b_sq, 1)
    
    question = f"Find the center of the ellipse: ${sp.latex(equation)}$"
    answer = f"({h}, {k})"
    difficulty = 1250

result = {
    "question_latex": question,
    "answer_key": answer,
    "difficulty": difficulty,
    "main_topic": "algebra2",
    "subtopic": "conic_sections",
    "grading_mode": "equivalent"
}

print(json.dumps(result))