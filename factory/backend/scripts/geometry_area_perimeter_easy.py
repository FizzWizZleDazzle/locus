"""
geometry - area_perimeter (easy)
Generated: 2026-02-11T21:41:59.601707
"""

import random
import json
from sympy import symbols, sympify, pi, sqrt, Rational

def generate_problem():
    problem_type = random.choice([
        'rectangle_area',
        'rectangle_perimeter',
        'square_area',
        'square_perimeter',
        'triangle_area',
        'circle_area',
        'circle_circumference',
        'composite_rectangle'
    ])
    
    if problem_type == 'rectangle_area':
        length = random.randint(3, 15)
        width = random.randint(3, 12)
        answer = length * width
        question = f"Find the area of a rectangle with length ${length}$ units and width ${width}$ units."
        difficulty = 1050
        
    elif problem_type == 'rectangle_perimeter':
        length = random.randint(4, 15)
        width = random.randint(3, 12)
        answer = 2 * (length + width)
        question = f"Find the perimeter of a rectangle with length ${length}$ units and width ${width}$ units."
        difficulty = 1100
        
    elif problem_type == 'square_area':
        side = random.randint(4, 12)
        answer = side * side
        question = f"Find the area of a square with side length ${side}$ units."
        difficulty = 1000
        
    elif problem_type == 'square_perimeter':
        side = random.randint(5, 15)
        answer = 4 * side
        question = f"Find the perimeter of a square with side length ${side}$ units."
        difficulty = 1050
        
    elif problem_type == 'triangle_area':
        base = random.randint(4, 16)
        height = random.randint(4, 14)
        answer = Rational(base * height, 2)
        question = f"Find the area of a triangle with base ${base}$ units and height ${height}$ units."
        difficulty = 1150
        
    elif problem_type == 'circle_area':
        radius = random.randint(3, 10)
        answer = pi * radius**2
        question = f"Find the area of a circle with radius ${radius}$ units. Express your answer in terms of $\\pi$."
        difficulty = 1200
        
    elif problem_type == 'circle_circumference':
        radius = random.randint(4, 12)
        answer = 2 * pi * radius
        question = f"Find the circumference of a circle with radius ${radius}$ units. Express your answer in terms of $\\pi$."
        difficulty = 1200
        
    elif problem_type == 'composite_rectangle':
        length = random.randint(5, 12)
        width = random.randint(4, 10)
        perimeter = 2 * (length + width)
        question = f"A rectangle has length ${length}$ units and width ${width}$ units. Find both the area and perimeter. What is the sum of the area and perimeter?"
        answer = length * width + perimeter
        difficulty = 1300
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "area_perimeter",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))