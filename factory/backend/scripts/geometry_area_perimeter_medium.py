"""
geometry - area_perimeter (medium)
Generated: 2026-02-11T21:42:14.395438
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'rectangle_area',
        'rectangle_perimeter', 
        'triangle_area',
        'circle_area',
        'circle_circumference',
        'composite_shape',
        'word_problem_fence',
        'word_problem_garden'
    ])
    
    if problem_type == 'rectangle_area':
        length = random.randint(5, 20)
        width = random.randint(3, 15)
        answer = length * width
        question = f"Find the area of a rectangle with length {length} units and width {width} units."
        difficulty = 1300
        
    elif problem_type == 'rectangle_perimeter':
        length = random.randint(6, 18)
        width = random.randint(4, 14)
        answer = 2 * (length + width)
        question = f"Find the perimeter of a rectangle with length {length} units and width {width} units."
        difficulty = 1300
        
    elif problem_type == 'triangle_area':
        base = random.randint(6, 20)
        height = random.randint(4, 16)
        answer = sp.Rational(base * height, 2)
        question = f"Find the area of a triangle with base {base} units and height {height} units."
        difficulty = 1350
        
    elif problem_type == 'circle_area':
        radius = random.randint(3, 12)
        answer = sp.pi * radius**2
        question = f"Find the area of a circle with radius {radius} units. Express your answer in terms of \\pi."
        difficulty = 1400
        
    elif problem_type == 'circle_circumference':
        radius = random.randint(4, 15)
        answer = 2 * sp.pi * radius
        question = f"Find the circumference of a circle with radius {radius} units. Express your answer in terms of \\pi."
        difficulty = 1350
        
    elif problem_type == 'composite_shape':
        rect_length = random.randint(10, 20)
        rect_width = random.randint(6, 12)
        triangle_base = rect_length
        triangle_height = random.randint(4, 10)
        rect_area = rect_length * rect_width
        triangle_area = sp.Rational(triangle_base * triangle_height, 2)
        answer = rect_area + triangle_area
        question = f"A shape consists of a rectangle with length {rect_length} units and width {rect_width} units, with a triangle on top. The triangle has a base equal to the rectangle's length and a height of {triangle_height} units. Find the total area of the composite shape."
        difficulty = 1550
        
    elif problem_type == 'word_problem_fence':
        length = random.randint(15, 30)
        width = random.randint(10, 25)
        perimeter = 2 * (length + width)
        answer = perimeter
        question = f"A rectangular garden has a length of {length} meters and a width of {width} meters. How many meters of fencing are needed to enclose the entire garden?"
        difficulty = 1450
        
    elif problem_type == 'word_problem_garden':
        side1 = random.randint(8, 16)
        side2 = random.randint(6, 14)
        area = side1 * side2
        cost_per_sqm = random.randint(3, 8)
        answer = area * cost_per_sqm
        question = f"A rectangular plot of land measuring {side1} meters by {side2} meters needs to be turfed. If turf costs \\${cost_per_sqm} per square meter, what is the total cost?"
        difficulty = 1500
    
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