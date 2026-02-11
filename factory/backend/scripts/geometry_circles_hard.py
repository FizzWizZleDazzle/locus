"""
geometry - circles (hard)
Generated: 2026-02-11T21:41:49.258995
"""

import sympy as sp
import random
import json

def generate_circle_problem():
    problem_type = random.choice([
        'chord_distance',
        'tangent_chord_angle',
        'power_of_point',
        'arc_sector_complex',
        'inscribed_angle_system',
        'circle_intersection'
    ])
    
    if problem_type == 'chord_distance':
        radius = random.randint(10, 20)
        chord_length = random.randint(12, 2*radius-4)
        if chord_length % 2 == 1:
            chord_length += 1
        
        distance_squared = radius**2 - (chord_length/2)**2
        distance = sp.sqrt(distance_squared)
        
        if distance == sp.sqrt(distance_squared).simplify() and distance.is_integer:
            question = f"A circle has radius {radius}. A chord of length {chord_length} is drawn. Find the distance from the center of the circle to the chord."
            answer = str(distance)
            difficulty = 1650
        else:
            question = f"A circle has radius {radius}. A chord of length {chord_length} is drawn. Find the square of the distance from the center of the circle to the chord."
            answer = str(distance_squared)
            difficulty = 1620
    
    elif problem_type == 'tangent_chord_angle':
        radius = random.randint(8, 15)
        arc_angle = random.choice([60, 80, 100, 120, 140])
        inscribed_angle = arc_angle / 2
        
        question = f"A circle has radius {radius}. A tangent line is drawn at point A, and a chord AB subtends an arc of {arc_angle}° at the center. Find the angle between the tangent at A and the chord AB in degrees."
        answer = str(int(90 - inscribed_angle))
        difficulty = 1720
    
    elif problem_type == 'power_of_point':
        radius = random.randint(8, 15)
        external_distance = random.randint(radius + 3, radius + 10)
        
        tangent_length_squared = external_distance**2 - radius**2
        tangent_length = sp.sqrt(tangent_length_squared)
        
        if tangent_length.is_integer:
            question = f"From a point P located {external_distance} units from the center of a circle with radius {radius}, a tangent line is drawn to the circle. Find the length of the tangent segment from P to the point of tangency."
            answer = str(tangent_length)
            difficulty = 1680
        else:
            question = f"From a point P located {external_distance} units from the center of a circle with radius {radius}, a tangent line is drawn to the circle. Find the square of the length of the tangent segment from P to the point of tangency."
            answer = str(tangent_length_squared)
            difficulty = 1650
    
    elif problem_type == 'arc_sector_complex':
        radius = random.randint(6, 12)
        central_angle = random.choice([45, 60, 90, 120, 135])
        
        arc_length = sp.Rational(central_angle, 180) * sp.pi * radius
        chord_length_expr = 2 * radius * sp.sin(sp.rad(central_angle/2))
        
        question = f"A sector of a circle with radius {radius} has a central angle of {central_angle}°. Find the difference between the arc length and the chord length. Express your answer in the form a\\pi + b\\sqrt{{c}} where a, b, c are rational."
        
        chord_length_simplified = chord_length_expr.simplify()
        difference = (arc_length - chord_length_simplified).simplify()
        answer = str(difference)
        difficulty = 1780
    
    elif problem_type == 'inscribed_angle_system':
        a = random.randint(15, 30)
        b = random.randint(10, 25)
        
        x = (180 - a - b) // 2
        
        question = f"In a circle, two chords AB and CD intersect at point E inside the circle. The inscribed angle ∠CAB = {a}° and ∠ACD = {b}°. Find the measure of ∠AEC in degrees."
        answer = str(a + b)
        difficulty = 1820
    
    elif problem_type == 'circle_intersection':
        r1 = random.randint(8, 13)
        r2 = random.randint(8, 13)
        
        d_values = []
        for d_test in range(abs(r1-r2)+1, r1+r2):
            radical_expr = (r1**2 - r2**2 + d_test**2) / (2*d_test)
            if abs(radical_expr) < r1:
                d_values.append(d_test)
        
        if d_values:
            d = random.choice(d_values[:len(d_values)//2+1])
            
            x = sp.Rational(r1**2 - r2**2 + d**2, 2*d)
            h_squared = r1**2 - x**2
            
            chord_length_squared = 4 * h_squared
            
            question = f"Two circles with radii {r1} and {r2} have centers that are {d} units apart. They intersect at two points. Find the square of the length of their common chord."
            answer = str(chord_length_squared)
            difficulty = 1850
        else:
            radius = random.randint(10, 18)
            chord_length = random.randint(12, 2*radius-4)
            if chord_length % 2 == 1:
                chord_length += 1
            
            distance_squared = radius**2 - (chord_length/2)**2
            question = f"A circle has radius {radius}. A chord of length {chord_length} is drawn. Find the square of the distance from the center of the circle to the chord."
            answer = str(distance_squared)
            difficulty = 1620
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "circles",
        "grading_mode": "equivalent"
    }

problem = generate_circle_problem()
print(json.dumps(problem))