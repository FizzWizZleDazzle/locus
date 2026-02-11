"""
calculus - applications_of_integration (easy)
Generated: 2026-02-11T22:02:06.064465
"""

import sympy as sp
import random
import json

def generate_problem():
    x = sp.Symbol('x')
    problem_type = random.choice([
        'area_under_curve',
        'area_between_curves',
        'volume_disk',
        'volume_washer'
    ])
    
    if problem_type == 'area_under_curve':
        difficulty = random.choice([1000, 1100, 1200, 1300])
        
        if difficulty <= 1100:
            # Simple polynomial, nice integer bounds
            # Answer: pick a clean integer
            answer = random.randint(4, 20)
            
            # Simple cases: x, x^2, constant
            func_type = random.choice(['linear', 'constant'])
            if func_type == 'constant':
                # integral of c from 0 to b = c*b = answer
                b = random.randint(2, 5)
                c = answer // b
                func = c
                a, b = 0, b
            else:
                # integral of x from 0 to b = b^2/2 = answer
                # so b^2 = 2*answer
                b_squared = 2 * answer
                b = int(b_squared ** 0.5)
                if b * b == b_squared:
                    func = x
                    a, b = 0, b
                else:
                    # Fall back to constant
                    b = random.randint(2, 5)
                    c = answer // b
                    func = c
                    a, b = 0, b
            
            question = f"Find the area under the curve $y = {sp.latex(func)}$ from $x = {a}$ to $x = {b}$."
            answer_key = str(answer)
            elo = 1050
            
        elif difficulty <= 1200:
            # Simple polynomial x^2, clean bounds
            # integral of x^2 from 0 to b = b^3/3
            b = random.choice([3, 6])  # gives integer answers
            func = x**2
            a = 0
            answer = b**3 // 3
            
            question = f"Find the area under the curve $y = {sp.latex(func)}$ from $x = {a}$ to $x = {b}$."
            answer_key = str(answer)
            elo = 1150
            
        else:  # 1300
            # Polynomial with coefficient, still clean
            # integral of c*x^2 from 0 to b = c*b^3/3
            c = random.choice([2, 3])
            b = random.choice([2, 3])
            func = c * x**2
            a = 0
            answer = c * b**3 // 3
            
            question = f"Find the area under the curve $y = {sp.latex(func)}$ from $x = {a}$ to $x = {b}$."
            answer_key = str(answer)
            elo = 1250
    
    elif problem_type == 'area_between_curves':
        difficulty = random.choice([1200, 1300])
        
        if difficulty <= 1200:
            # Simple: horizontal line minus x
            # y = c and y = 0, from 0 to b
            # Answer = c*b
            answer = random.choice([6, 8, 10, 12])
            b = random.choice([2, 3, 4])
            c = answer // b
            
            question = f"Find the area between $y = {c}$ and the $x$-axis from $x = 0$ to $x = {b}$."
            answer_key = str(answer)
            elo = 1200
            
        else:  # 1300
            # Line minus another line
            # y = c and y = x from 0 to b
            # Answer = c*b - b^2/2
            b = random.choice([2, 4])
            c = random.choice([3, 4, 5])
            answer = c * b - b**2 // 2
            
            question = f"Find the area between $y = {c}$ and $y = x$ from $x = 0$ to $x = {b}$."
            answer_key = str(answer)
            elo = 1280
    
    elif problem_type == 'volume_disk':
        difficulty = random.choice([1100, 1200, 1300])
        
        if difficulty <= 1100:
            # Constant function rotated
            # V = pi*r^2*h
            r = random.choice([1, 2, 3])
            h = random.choice([2, 3, 4])
            answer = sp.pi * r**2 * h
            
            question = f"Find the volume when the region under $y = {r}$ from $x = 0$ to $x = {h}$ is rotated about the $x$-axis."
            answer_key = str(answer)
            elo = 1100
            
        elif difficulty <= 1200:
            # y = x rotated, from 0 to b
            # V = pi * integral(x^2) = pi*b^3/3
            b = random.choice([3, 6])
            answer = sp.pi * b**3 // 3
            
            question = f"Find the volume when the region under $y = x$ from $x = 0$ to $x = {b}$ is rotated about the $x$-axis."
            answer_key = str(answer)
            elo = 1180
            
        else:  # 1300
            # y = sqrt(x) rotated
            # V = pi * integral(x) = pi*b^2/2
            b = random.choice([2, 4, 6])
            answer = sp.pi * b**2 // 2
            
            question = f"Find the volume when the region under $y = \\sqrt{{x}}$ from $x = 0$ to $x = {b}$ is rotated about the $x$-axis."
            answer_key = str(answer)
            elo = 1270
    
    else:  # volume_washer
        difficulty = 1300
        # Simple washer: outer radius constant, inner radius x
        # V = pi * integral(R^2 - r^2)
        R = random.choice([2, 3])
        b = random.choice([1, 2])
        # V = pi * integral(R^2 - x^2) from 0 to b = pi*(R^2*b - b^3/3)
        answer = sp.pi * (R**2 * b - b**3 // 3)
        
        question = f"Find the volume when the region between $y = {R}$ and $y = x$ from $x = 0$ to $x = {b}$ is rotated about the $x$-axis."
        answer_key = str(answer)
        elo = 1290
    
    return {
        "question_latex": question,
        "answer_key": answer_key,
        "difficulty": elo,
        "main_topic": "calculus",
        "subtopic": "applications_of_integration",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))