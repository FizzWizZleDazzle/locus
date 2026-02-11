"""
algebra1 - quadratic_equations (medium)
Generated: 2026-02-11T21:38:31.983627
"""

import random
import json
from sympy import symbols, expand, latex, solve, Rational, sqrt, simplify

def generate_quadratic_problem():
    x = symbols('x')
    problem_type = random.choice(['factored', 'completing_square', 'standard_form', 'word_problem'])
    
    if problem_type == 'factored':
        # Pick clean integer solutions
        root1 = random.randint(-8, 8)
        root2 = random.randint(-8, 8)
        while root1 == root2:
            root2 = random.randint(-8, 8)
        
        # Construct from roots
        equation = expand((x - root1) * (x - root2))
        
        # Randomize presentation
        presentation = random.choice(['solve', 'factor'])
        if presentation == 'solve':
            question = f"Solve for $x$: ${latex(equation)} = 0$"
            difficulty = 1300
        else:
            question = f"Factor: ${latex(equation)}$"
            difficulty = 1350
        
        answer = f"{{{min(root1, root2)}, {max(root1, root2)}}}"
        
    elif problem_type == 'completing_square':
        # Pick a clean perfect square form
        h = random.randint(-6, 6)
        k = random.randint(-10, 10)
        a = random.choice([1, 2, 3])
        
        # (x - h)^2 = k/a, so x = h ± sqrt(k/a)
        # Make k/a a perfect square
        perfect_sq = random.randint(1, 9)
        k = a * perfect_sq
        
        root1 = h + perfect_sq**0.5
        root2 = h - perfect_sq**0.5
        
        # Expand to standard form
        equation = expand(a * (x - h)**2 - k)
        
        question = f"Solve by completing the square: ${latex(equation)} = 0$"
        difficulty = 1500
        
        if perfect_sq == int(perfect_sq**0.5)**2:
            answer = f"{{{h - int(perfect_sq**0.5)}, {h + int(perfect_sq**0.5)}}}"
        else:
            answer = f"{{{h - sqrt(perfect_sq)}, {h + sqrt(perfect_sq)}}}"
    
    elif problem_type == 'standard_form':
        # Pick coefficients for ax^2 + bx + c = 0
        a = random.choice([1, 2, 3])
        
        # Ensure discriminant is a perfect square
        root1 = random.randint(-7, 7)
        root2 = random.randint(-7, 7)
        
        # Construct from roots: a(x - root1)(x - root2) = 0
        equation = expand(a * (x - root1) * (x - root2))
        
        method = random.choice(['quadratic_formula', 'just_solve'])
        
        if method == 'quadratic_formula':
            question = f"Use the quadratic formula to solve: ${latex(equation)} = 0$"
            difficulty = 1450
        else:
            question = f"Solve for $x$: ${latex(equation)} = 0$"
            difficulty = 1350
        
        answer = f"{{{min(root1, root2)}, {max(root1, root2)}}}"
    
    else:  # word_problem
        # Area or projectile problem
        prob_subtype = random.choice(['area', 'consecutive'])
        
        if prob_subtype == 'area':
            width = random.randint(3, 12)
            length = width + random.randint(2, 8)
            area = width * length
            
            question = f"A rectangle has a length that is {length - width} units more than its width. If the area is {area} square units, find the width."
            difficulty = 1550
            answer = str(width)
        
        else:  # consecutive integers
            n = random.randint(5, 15)
            sum_val = n * (n + 1)
            
            question = f"Two consecutive positive integers have a product of {sum_val}. Find the smaller integer."
            difficulty = 1500
            answer = str(n)
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "algebra1",
        "subtopic": "quadratic_equations",
        "grading_mode": "equivalent"
    }

problem = generate_quadratic_problem()
print(json.dumps(problem))