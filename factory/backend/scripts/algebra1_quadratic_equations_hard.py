"""
algebra1 - quadratic_equations (hard)
Generated: 2026-02-11T21:38:55.301473
"""

import sympy as sp
import random
import json

x = sp.Symbol('x')

def generate_hard_quadratic():
    problem_type = random.choice([
        'word_problem_projectile',
        'word_problem_geometry',
        'system_with_quadratic',
        'rational_equation_quadratic',
        'complex_factoring',
        'quadratic_inequality',
        'applied_vertex_form'
    ])
    
    if problem_type == 'word_problem_projectile':
        h0 = random.choice([0, 5, 10, 20])
        v0 = random.choice([40, 48, 60, 80, 96])
        g = 16
        t_target = random.randint(2, 4)
        
        question = f"A projectile is launched upward from a height of {h0} feet with an initial velocity of {v0} feet per second. Its height h(t) in feet after t seconds is given by $h(t) = -{g}t^2 + {v0}t + {h0}$. At what time(s) will the projectile be at a height of {-g*t_target**2 + v0*t_target + h0} feet?"
        
        height_target = -g*t_target**2 + v0*t_target + h0
        eq = sp.Eq(-g*x**2 + v0*x + h0, height_target)
        solutions = sp.solve(eq, x)
        answer = sp.FiniteSet(*[sol for sol in solutions if sol >= 0])
        
        return question, str(answer), 1700
    
    elif problem_type == 'word_problem_geometry':
        width = random.randint(3, 8)
        length = width + random.randint(2, 5)
        area = width * length
        
        question = f"A rectangle has a length that is {length - width} meters more than its width. If the area of the rectangle is {area} square meters, find the dimensions of the rectangle."
        
        eq = sp.Eq(x * (x + (length - width)), area)
        solutions = sp.solve(eq, x)
        positive_sol = [sol for sol in solutions if sol > 0][0]
        answer = sp.FiniteSet(positive_sol, positive_sol + (length - width))
        
        return question, str(answer), 1650
    
    elif problem_type == 'system_with_quadratic':
        x_sol = random.randint(-3, 5)
        y_sol = random.randint(1, 8)
        
        a = random.choice([1, 2, -1])
        c = a * x_sol**2 - y_sol
        
        m = random.choice([1, 2, 3, -1, -2])
        b = y_sol - m * x_sol
        
        question = f"Solve the system of equations: $y = {sp.latex(a*x**2 + c)}$ and $y = {sp.latex(m*x + b)}$. Find the x-coordinate(s) of the solution(s)."
        
        eq = sp.Eq(a*x**2 + c, m*x + b)
        solutions = sp.solve(eq, x)
        answer = sp.FiniteSet(*solutions)
        
        return question, str(answer), 1750
    
    elif problem_type == 'rational_equation_quadratic':
        root1 = random.randint(1, 5)
        root2 = random.randint(-4, -1)
        
        numerator_const = random.randint(2, 8)
        denom_const = random.choice([1, 2, 3])
        
        question = f"Solve for x: $\\frac{{{numerator_const}}}{{x - {root2}}} + \\frac{{{denom_const}}}{{x}} = {numerator_const + denom_const}$"
        
        eq = sp.Eq(numerator_const/(x - root2) + denom_const/x, numerator_const + denom_const)
        solutions = sp.solve(eq, x)
        valid_solutions = [sol for sol in solutions if sol != 0 and sol != root2]
        answer = sp.FiniteSet(*valid_solutions) if valid_solutions else sp.EmptySet
        
        return question, str(answer), 1800
    
    elif problem_type == 'complex_factoring':
        root1 = random.randint(1, 6)
        root2 = random.randint(-5, -1)
        a = random.choice([2, 3, 4])
        
        expanded = a * (x - root1) * (x - root2)
        expanded = sp.expand(expanded)
        
        question = f"Solve by factoring: ${sp.latex(expanded)} = 0$"
        
        solutions = sp.solve(expanded, x)
        answer = sp.FiniteSet(*solutions)
        
        return question, str(answer), 1600
    
    elif problem_type == 'quadratic_inequality':
        root1 = random.randint(-3, 1)
        root2 = root1 + random.randint(3, 6)
        
        poly = (x - root1) * (x - root2)
        poly = sp.expand(poly)
        
        question = f"Solve the inequality: ${sp.latex(poly)} < 0$"
        
        answer = f"({root1}, {root2})"
        
        return question, answer, 1850
    
    else:  # applied_vertex_form
        h = random.randint(-5, 5)
        k = random.randint(10, 30)
        a = random.choice([-1, -2, -3])
        
        question = f"A parabola has equation $y = {a}(x - {h})^2 + {k}$. Find the x-intercepts of this parabola."
        
        eq = sp.Eq(a*(x - h)**2 + k, 0)
        solutions = sp.solve(eq, x)
        answer = sp.FiniteSet(*solutions)
        
        return question, str(answer), 1700

question_latex, answer_key, difficulty = generate_hard_quadratic()

output = {
    "question_latex": question_latex,
    "answer_key": answer_key,
    "difficulty": difficulty,
    "main_topic": "algebra1",
    "subtopic": "quadratic_equations",
    "grading_mode": "equivalent"
}

print(json.dumps(output))