"""
algebra2 - conic_sections (medium)
Generated: 2026-02-11T21:50:53.708666
"""

import sympy as sp
import random
import json

x, y = sp.symbols('x y')

def generate_problem():
    problem_type = random.choice([
        'circle_standard_form',
        'ellipse_identify',
        'parabola_vertex',
        'hyperbola_identify',
        'complete_square_circle',
        'ellipse_features',
        'parabola_equation'
    ])
    
    if problem_type == 'circle_standard_form':
        h = random.randint(-5, 5)
        k = random.randint(-5, 5)
        r = random.randint(2, 6)
        
        equation = sp.Eq((x - h)**2 + (y - k)**2, r**2)
        question = f"Identify the center and radius of the circle: ${sp.latex(equation)}$"
        answer = f"({h}, {k}), r={r}"
        difficulty = 1300
        
        return question, answer, difficulty
    
    elif problem_type == 'ellipse_identify':
        a_sq = random.choice([9, 16, 25, 36, 49])
        b_sq = random.choice([4, 9, 16, 25])
        if a_sq <= b_sq:
            a_sq, b_sq = b_sq, a_sq
        
        h = random.randint(-4, 4)
        k = random.randint(-4, 4)
        
        orientation = random.choice(['horizontal', 'vertical'])
        
        if orientation == 'horizontal':
            equation = sp.Eq((x - h)**2 / a_sq + (y - k)**2 / b_sq, 1)
        else:
            equation = sp.Eq((x - h)**2 / b_sq + (y - k)**2 / a_sq, 1)
        
        question = f"Find the center of the ellipse: ${sp.latex(equation)}$"
        answer = f"({h}, {k})"
        difficulty = 1350
        
        return question, answer, difficulty
    
    elif problem_type == 'parabola_vertex':
        h = random.randint(-5, 5)
        k = random.randint(-5, 5)
        a = random.choice([-2, -1, 1, 2, 3])
        
        if random.choice([True, False]):
            equation = sp.Eq(y, a * (x - h)**2 + k)
            question = f"Find the vertex of the parabola: ${sp.latex(equation)}$"
        else:
            equation = sp.Eq(x, a * (y - k)**2 + h)
            question = f"Find the vertex of the parabola: ${sp.latex(equation)}$"
        
        answer = f"({h}, {k})"
        difficulty = 1300
        
        return question, answer, difficulty
    
    elif problem_type == 'hyperbola_identify':
        a_sq = random.choice([4, 9, 16, 25])
        b_sq = random.choice([4, 9, 16, 25])
        h = random.randint(-3, 3)
        k = random.randint(-3, 3)
        
        orientation = random.choice(['horizontal', 'vertical'])
        
        if orientation == 'horizontal':
            equation = sp.Eq((x - h)**2 / a_sq - (y - k)**2 / b_sq, 1)
        else:
            equation = sp.Eq((y - k)**2 / a_sq - (x - h)**2 / b_sq, 1)
        
        question = f"Identify the center of the hyperbola: ${sp.latex(equation)}$"
        answer = f"({h}, {k})"
        difficulty = 1400
        
        return question, answer, difficulty
    
    elif problem_type == 'complete_square_circle':
        h = random.randint(-4, 4)
        k = random.randint(-4, 4)
        r = random.randint(2, 5)
        
        expanded = sp.expand((x - h)**2 + (y - k)**2 - r**2)
        equation = sp.Eq(expanded, 0)
        
        question = f"Complete the square to find the radius of the circle: ${sp.latex(equation)}$"
        answer = str(r)
        difficulty = 1500
        
        return question, answer, difficulty
    
    elif problem_type == 'ellipse_features':
        a_sq = random.choice([16, 25, 36, 49])
        b_sq = random.choice([4, 9, 16])
        if a_sq <= b_sq:
            a_sq, b_sq = b_sq, a_sq
        
        a = sp.sqrt(a_sq)
        
        equation = sp.Eq(x**2 / a_sq + y**2 / b_sq, 1)
        
        question = f"Find the length of the major axis of the ellipse: ${sp.latex(equation)}$"
        answer = str(2 * a)
        difficulty = 1450
        
        return question, answer, difficulty
    
    elif problem_type == 'parabola_equation':
        p = random.choice([1, 2, 3, 4])
        h = random.randint(-3, 3)
        k = random.randint(-3, 3)
        
        direction = random.choice(['right', 'up'])
        
        if direction == 'right':
            question = f"Find the value of $p$ for the parabola $(y - {k})^2 = {4*p}(x - {h})$"
        else:
            question = f"Find the value of $p$ for the parabola $(x - {h})^2 = {4*p}(y - {k})$"
        
        answer = str(p)
        difficulty = 1550
        
        return question, answer, difficulty

question_latex, answer_key, difficulty = generate_problem()

output = {
    "question_latex": question_latex,
    "answer_key": answer_key,
    "difficulty": difficulty,
    "main_topic": "algebra2",
    "subtopic": "conic_sections",
    "grading_mode": "equivalent"
}

print(json.dumps(output))