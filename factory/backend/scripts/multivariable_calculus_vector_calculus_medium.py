"""
multivariable_calculus - vector_calculus (medium)
Generated: 2026-02-11T22:05:26.591448
"""

import sympy as sp
import random
import json

def generate_vector_calculus_problem():
    x, y, z, t = sp.symbols('x y z t')
    
    problem_type = random.choice([
        'gradient',
        'divergence',
        'curl',
        'line_integral',
        'conservative_field'
    ])
    
    if problem_type == 'gradient':
        a = random.randint(1, 4)
        b = random.randint(1, 4)
        c = random.randint(1, 4)
        
        power_x = random.randint(2, 3)
        power_y = random.randint(2, 3)
        
        f = a*x**power_x + b*y**power_y + c*x*y
        
        grad_f = sp.Matrix([sp.diff(f, x), sp.diff(f, y)])
        
        question = f"\\text{{Find the gradient of }} f(x,y) = {sp.latex(f)}"
        answer = str(grad_f)
        difficulty = 1350
        
    elif problem_type == 'divergence':
        a = random.randint(1, 3)
        b = random.randint(1, 3)
        c = random.randint(1, 3)
        
        use_xyz = random.choice([True, False])
        
        if use_xyz:
            F1 = a*x*y
            F2 = b*y*z
            F3 = c*x*z
            
            div_F = sp.diff(F1, x) + sp.diff(F2, y) + sp.diff(F3, z)
            
            question = f"\\text{{Find the divergence of }} \\mathbf{{F}} = {sp.latex(F1)}\\mathbf{{i}} + {sp.latex(F2)}\\mathbf{{j}} + {sp.latex(F3)}\\mathbf{{k}}"
            difficulty = 1400
        else:
            F1 = a*x**2
            F2 = b*y**2
            
            div_F = sp.diff(F1, x) + sp.diff(F2, y)
            
            question = f"\\text{{Find the divergence of }} \\mathbf{{F}} = {sp.latex(F1)}\\mathbf{{i}} + {sp.latex(F2)}\\mathbf{{j}}"
            difficulty = 1300
        
        answer = str(div_F)
        
    elif problem_type == 'curl':
        a = random.randint(1, 3)
        b = random.randint(1, 3)
        c = random.randint(1, 3)
        
        difficulty_level = random.choice(['simple', 'moderate'])
        
        if difficulty_level == 'simple':
            F1 = a*y
            F2 = b*x
            F3 = 0
            
            curl_z = sp.diff(F2, x) - sp.diff(F1, y)
            curl_F = sp.Matrix([0, 0, curl_z])
            
            question = f"\\text{{Find the curl of }} \\mathbf{{F}} = {sp.latex(F1)}\\mathbf{{i}} + {sp.latex(F2)}\\mathbf{{j}}"
            difficulty = 1350
        else:
            F1 = a*y*z
            F2 = b*x*z
            F3 = c*x*y
            
            curl_x = sp.diff(F3, y) - sp.diff(F2, z)
            curl_y = sp.diff(F1, z) - sp.diff(F3, x)
            curl_z = sp.diff(F2, x) - sp.diff(F1, y)
            
            curl_F = sp.Matrix([curl_x, curl_y, curl_z])
            
            question = f"\\text{{Find the curl of }} \\mathbf{{F}} = {sp.latex(F1)}\\mathbf{{i}} + {sp.latex(F2)}\\mathbf{{j}} + {sp.latex(F3)}\\mathbf{{k}}"
            difficulty = 1500
        
        answer = str(curl_F)
        
    elif problem_type == 'line_integral':
        a = random.randint(1, 3)
        b = random.randint(1, 3)
        
        t_start = 0
        t_end = random.choice([1, 2])
        
        r_x = a*t
        r_y = b*t**2
        
        F1 = x
        F2 = y
        
        dx_dt = sp.diff(r_x, t)
        dy_dt = sp.diff(r_y, t)
        
        F1_param = F1.subs([(x, r_x), (y, r_y)])
        F2_param = F2.subs([(x, r_x), (y, r_y)])
        
        integrand = F1_param * dx_dt + F2_param * dy_dt
        result = sp.integrate(integrand, (t, t_start, t_end))
        
        question = f"\\text{{Evaluate the line integral }} \\int_C \\mathbf{{F}} \\cdot d\\mathbf{{r}} \\text{{ where }} \\mathbf{{F}} = {sp.latex(F1)}\\mathbf{{i}} + {sp.latex(F2)}\\mathbf{{j}} \\text{{ and }} C \\text{{ is given by }} \\mathbf{{r}}(t) = {sp.latex(r_x)}\\mathbf{{i}} + {sp.latex(r_y)}\\mathbf{{j}}, \\ {t_start} \\leq t \\leq {t_end}"
        answer = str(result)
        difficulty = 1550
        
    else:  # conservative_field
        a = random.randint(1, 4)
        b = random.randint(1, 4)
        
        f = a*x**2*y + b*y**3
        
        F1 = sp.diff(f, x)
        F2 = sp.diff(f, y)
        
        curl_z = sp.diff(F2, x) - sp.diff(F1, y)
        
        if curl_z == 0:
            answer_text = "conservative"
        else:
            answer_text = "not_conservative"
        
        question = f"\\text{{Determine if the vector field }} \\mathbf{{F}} = ({sp.latex(F1)})\\mathbf{{i}} + ({sp.latex(F2)})\\mathbf{{j}} \\text{{ is conservative}}"
        answer = answer_text
        difficulty = 1450
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "multivariable_calculus",
        "subtopic": "vector_calculus",
        "grading_mode": "equivalent"
    }

problem = generate_vector_calculus_problem()
print(json.dumps(problem))