"""
algebra2 - complex_numbers (hard)
Generated: 2026-02-11T21:46:14.665904
"""

import sympy as sp
import random
import json

def generate_complex_numbers_problem():
    random.seed()
    
    problem_type = random.choice([
        'powers_of_complex',
        'complex_equation',
        'polar_conversion_hard',
        'complex_roots',
        'division_simplification'
    ])
    
    i = sp.I
    
    if problem_type == 'powers_of_complex':
        a = random.randint(1, 3)
        b = random.randint(1, 3)
        n = random.randint(3, 5)
        
        z = a + b*i
        answer = z**n
        answer = sp.expand(answer)
        
        question = f"Compute $({a} + {b}i)^{{{n}}}$ and write your answer in standard form $a + bi$."
        difficulty = 1650 + (n-3)*50
        
    elif problem_type == 'complex_equation':
        real_root = random.randint(-3, 3)
        imag_root = random.choice([1, 2, -1, -2])
        root1 = real_root + imag_root*i
        root2 = sp.conjugate(root1)
        
        x = sp.Symbol('x')
        poly = sp.expand((x - root1)*(x - root2))
        
        a_coeff = random.randint(1, 3)
        poly = sp.expand(a_coeff * poly)
        
        answer = root1
        
        poly_latex = sp.latex(poly)
        question = f"Find the complex solution to ${poly_latex} = 0$ with positive imaginary part."
        difficulty = 1700
        
    elif problem_type == 'polar_conversion_hard':
        r = random.choice([2, 3, 4])
        theta_num = random.choice([1, 2, 3, 5])
        theta_den = random.choice([3, 4, 6])
        
        theta = sp.pi * theta_num / theta_den
        z = r * (sp.cos(theta) + i * sp.sin(theta))
        z = sp.simplify(z)
        
        n = random.randint(2, 4)
        answer = sp.simplify(z**n)
        
        question = f"Convert $z = {r}\\left(\\cos\\left(\\frac{{{theta_num}\\pi}}{{{theta_den}}}\\right) + i\\sin\\left(\\frac{{{theta_num}\\pi}}{{{theta_den}}}\\right)\\right)$ to rectangular form, then compute $z^{{{n}}}$ in rectangular form."
        difficulty = 1750
        
    elif problem_type == 'complex_roots':
        n = random.choice([3, 4])
        real_part = random.choice([8, 27, 16, 81])
        
        if random.choice([True, False]):
            num = real_part
        else:
            num = real_part * i
        
        roots = [sp.simplify(num**(sp.Rational(1, n)) * sp.exp(2*sp.pi*i*k/n)) for k in range(n)]
        
        root_idx = random.randint(1, n-1)
        answer = sp.simplify(roots[root_idx])
        
        num_latex = sp.latex(num)
        question = f"Find all {n} complex {n}th roots of ${num_latex}$. Give the root with the smallest positive argument (angle) that is not the principal (real) root."
        difficulty = 1800
        
    else:  # division_simplification
        a1 = random.randint(1, 4)
        b1 = random.randint(1, 4)
        a2 = random.randint(1, 3)
        b2 = random.randint(1, 3)
        
        z1 = a1 + b1*i
        z2 = a2 + b2*i
        
        result = sp.simplify(z1 / z2)
        answer = result
        
        z1_latex = sp.latex(z1)
        z2_latex = sp.latex(z2)
        
        question = f"Simplify $\\frac{{{z1_latex}}}{{{z2_latex}}}$ and write your answer in standard form $a + bi$."
        difficulty = 1650
    
    problem = {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "complex_numbers",
        "grading_mode": "equivalent"
    }
    
    return problem

result = generate_complex_numbers_problem()
print(json.dumps(result))