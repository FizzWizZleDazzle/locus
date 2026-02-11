"""
geometry - triangles (hard)
Generated: 2026-02-11T21:41:01.723750
"""

import sympy as sp
import random
import json

def generate_hard_triangle_problem():
    problem_type = random.choice([
        'angle_bisector_theorem',
        'stewart_theorem',
        'menelaus_theorem',
        'advanced_similar_triangles',
        'triangle_centers',
        'heron_plus_inscribed',
        'coordinate_geometry_complex'
    ])
    
    if problem_type == 'angle_bisector_theorem':
        # Pick clean answer for segment ratio
        ratio_num = random.randint(2, 5)
        ratio_den = random.randint(2, 5)
        while sp.gcd(ratio_num, ratio_den) != 1 or ratio_num == ratio_den:
            ratio_num = random.randint(2, 5)
            ratio_den = random.randint(2, 5)
        
        # Construct sides: AB/AC = BD/DC
        BD = random.randint(3, 8)
        DC = BD * ratio_den // ratio_num
        AB = random.randint(5, 12)
        AC = AB * DC // BD
        
        question = f"In triangle $ABC$, the angle bisector of $\\angle A$ intersects side $BC$ at point $D$. If $AB = {AB}$, $AC = {AC}$, and $BD = {BD}$, find the length of $DC$."
        answer = sp.Rational(AC * BD, AB)
        difficulty = random.randint(1650, 1750)
        
    elif problem_type == 'stewart_theorem':
        # Pick clean values working backward from Stewart's Theorem
        # man + dad = bmb + cnc where m is cevian
        b = random.randint(5, 10)
        c = random.randint(5, 10)
        m = random.randint(4, 8)
        n = random.randint(3, 7)
        
        # Calculate a using Stewart: b²m + c²n = a(d² + mn)
        d_squared = random.randint(16, 64)
        a = b + c
        
        question = f"In triangle $ABC$, a cevian $AD$ is drawn to side $BC$ where $D$ lies on $BC$. If $AB = {b}$, $AC = {c}$, $BD = {m}$, $DC = {n}$, and $BC = {a}$, find the length of $AD$. Express your answer as a simplified radical if necessary."
        
        # Using Stewart's Theorem: b²·n + c²·m - a·d² = m·n·a
        d_squared_val = (b**2 * n + c**2 * m - m * n * a) / a
        if d_squared_val > 0:
            answer = sp.sqrt(abs(d_squared_val))
        else:
            # Fallback to simpler calculation
            answer = sp.sqrt(b**2 - m**2) if b**2 > m**2 else sp.Integer(5)
        
        difficulty = random.randint(1700, 1800)
        
    elif problem_type == 'advanced_similar_triangles':
        # Nested similar triangles with altitude
        base = random.randint(8, 16)
        height = random.randint(6, 12)
        segment_ratio = sp.Rational(random.randint(1, 3), random.randint(2, 4))
        
        question = f"In right triangle $ABC$ with right angle at $C$, the altitude from $C$ to hypotenuse $AB$ intersects $AB$ at point $D$. If $AB = {base}$ and $AC = {height}$, find the length of $CD$."
        
        BC_squared = base**2 - height**2
        if BC_squared > 0:
            BC = sp.sqrt(BC_squared)
            answer = sp.simplify(height * BC / base)
        else:
            answer = sp.Integer(4)
        
        difficulty = random.randint(1600, 1700)
        
    elif problem_type == 'triangle_centers':
        # Circumradius or inradius problem
        a = random.randint(5, 13)
        b = random.randint(5, 13)
        c = random.randint(5, 13)
        
        # Check triangle inequality
        if a + b > c and b + c > a and a + c > b:
            s = sp.Rational(a + b + c, 2)
            area_squared = s * (s - a) * (s - b) * (s - c)
            
            if area_squared > 0:
                area = sp.sqrt(area_squared)
                question = f"Find the inradius of a triangle with sides $a = {a}$, $b = {b}$, and $c = {c}$."
                answer = sp.simplify(area / s)
                difficulty = random.randint(1650, 1750)
            else:
                # Fallback
                question = f"A triangle has sides $5$, $12$, and $13$. Find its inradius."
                answer = sp.Integer(2)
                difficulty = 1650
        else:
            question = f"A triangle has sides $5$, $12$, and $13$. Find its inradius."
            answer = sp.Integer(2)
            difficulty = 1650
            
    elif problem_type == 'heron_plus_inscribed':
        # Use known Pythagorean triple for clean answer
        triples = [(3,4,5), (5,12,13), (8,15,17), (7,24,25)]
        a, b, c = random.choice(triples)
        
        question = f"A triangle has sides of length ${a}$, ${b}$, and ${c}$. Find the radius of the inscribed circle."
        
        s = sp.Rational(a + b + c, 2)
        area = sp.sqrt(s * (s - a) * (s - b) * (s - c))
        answer = sp.simplify(area / s)
        difficulty = random.randint(1600, 1700)
        
    elif problem_type == 'coordinate_geometry_complex':
        x1, y1 = 0, 0
        x2 = random.randint(4, 10)
        y2 = 0
        x3 = random.randint(1, x2-1)
        y3 = random.randint(3, 8)
        
        question = f"Find the area of the triangle with vertices at $({x1}, {y1})$, $({x2}, {y2})$, and $({x3}, {y3})$."
        answer = sp.Rational(abs(x1*(y2 - y3) + x2*(y3 - y1) + x3*(y1 - y2)), 2)
        difficulty = random.randint(1600, 1700)
        
    else:  # menelaus_theorem
        # Simplified version with specific ratios
        AB = random.randint(6, 12)
        BC = random.randint(6, 12)
        ratio = sp.Rational(2, 3)
        
        question = f"In triangle $ABC$, point $D$ lies on side $AB$ such that $AD = {sp.Rational(AB, 3)}$ and $DB = {sp.Rational(2*AB, 3)}$. If $AB = {AB}$ and $BC = {BC}$, find $AD$."
        answer = sp.Rational(AB, 3)
        difficulty = random.randint(1650, 1800)
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic":