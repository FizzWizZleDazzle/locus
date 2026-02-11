"""
calculus - applications_of_integration (medium)
Generated: 2026-02-11T22:02:22.819535
"""

import sympy as sp
import random
import json

def generate_problem():
    x = sp.Symbol('x')
    
    problem_types = [
        'area_between_curves',
        'volume_disk',
        'volume_washer',
        'average_value'
    ]
    
    problem_type = random.choice(problem_types)
    
    if problem_type == 'area_between_curves':
        a = random.randint(0, 2)
        b = random.randint(3, 5)
        
        k1 = random.randint(1, 3)
        k2 = random.randint(1, 3)
        
        if random.choice([True, False]):
            f = k1 * x
            g = k2 * x**2
            area = sp.integrate(f - g, (x, a, b))
            question = f"\\text{{Find the area between }} f(x) = {sp.latex(f)} \\text{{ and }} g(x) = {sp.latex(g)} \\text{{ from }} x = {a} \\text{{ to }} x = {b}."
            difficulty = 1400
        else:
            c1 = random.randint(2, 6)
            c2 = random.randint(1, 3)
            f = c1
            g = c2 * x
            b = random.randint(1, 3)
            a = 0
            area = sp.integrate(f - g, (x, a, b))
            question = f"\\text{{Find the area between }} f(x) = {c1} \\text{{ and }} g(x) = {sp.latex(g)} \\text{{ from }} x = {a} \\text{{ to }} x = {b}."
            difficulty = 1300
    
    elif problem_type == 'volume_disk':
        c = random.randint(1, 4)
        a = random.randint(0, 2)
        b = random.randint(3, 5)
        
        if random.choice([True, False]):
            f = c * x
            volume = sp.pi * sp.integrate(f**2, (x, a, b))
            question = f"\\text{{Find the volume when }} y = {sp.latex(f)} \\text{{ is rotated about the x-axis from }} x = {a} \\text{{ to }} x = {b}."
            difficulty = 1400
        else:
            f = c
            b = random.randint(2, 5)
            a = 0
            volume = sp.pi * sp.integrate(f**2, (x, a, b))
            question = f"\\text{{Find the volume when }} y = {c} \\text{{ is rotated about the x-axis from }} x = {a} \\text{{ to }} x = {b}."
            difficulty = 1300
        
        area = volume
    
    elif problem_type == 'volume_washer':
        R = random.randint(3, 6)
        r = random.randint(1, 2)
        b = random.randint(2, 4)
        a = 0
        
        outer = R
        inner = r
        
        volume = sp.pi * sp.integrate(outer**2 - inner**2, (x, a, b))
        question = f"\\text{{Find the volume of the solid with outer radius }} R = {R} \\text{{ and inner radius }} r = {r} \\text{{ from }} x = {a} \\text{{ to }} x = {b} \\text{{ (washer method)}}."
        difficulty = 1500
        area = volume
    
    elif problem_type == 'average_value':
        c = random.randint(1, 4)
        b = random.randint(2, 5)
        a = 0
        
        if random.choice([True, False]):
            f = c * x**2
            avg = sp.integrate(f, (x, a, b)) / (b - a)
            question = f"\\text{{Find the average value of }} f(x) = {sp.latex(f)} \\text{{ on }} [{a}, {b}]."
            difficulty = 1400
        else:
            f = c * x
            avg = sp.integrate(f, (x, a, b)) / (b - a)
            question = f"\\text{{Find the average value of }} f(x) = {sp.latex(f)} \\text{{ on }} [{a}, {b}]."
            difficulty = 1300
        
        area = avg
    
    result = {
        "question_latex": question,
        "answer_key": str(sp.simplify(area)),
        "difficulty": difficulty,
        "main_topic": "calculus",
        "subtopic": "applications_of_integration",
        "grading_mode": "equivalent"
    }
    
    return result

problem = generate_problem()
print(json.dumps(problem))