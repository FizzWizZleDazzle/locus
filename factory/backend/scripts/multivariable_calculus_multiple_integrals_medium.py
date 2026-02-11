"""
multivariable_calculus - multiple_integrals (medium)
Generated: 2026-02-11T22:04:26.526347
"""

import sympy as sp
import random
import json

def generate_multiple_integral_problem():
    x, y, z = sp.symbols('x y z')
    
    problem_type = random.choice([
        'double_integral_rectangular',
        'double_integral_type1',
        'double_integral_type2',
        'triple_integral_rectangular',
        'double_integral_polar'
    ])
    
    if problem_type == 'double_integral_rectangular':
        a = random.randint(0, 2)
        b = random.randint(a + 1, 4)
        c = random.randint(0, 2)
        d = random.randint(c + 1, 4)
        
        answer = random.randint(1, 50)
        
        x_power = random.randint(1, 3)
        y_power = random.randint(1, 3)
        
        needed_coeff = answer * (x_power + 1) * (y_power + 1) / ((b**(x_power+1) - a**(x_power+1)) * (d**(y_power+1) - c**(y_power+1)))
        coeff = sp.Rational(needed_coeff).limit_denominator(10)
        
        integrand = coeff * x**x_power * y**y_power
        
        question = f"\\int_{{{a}}}^{{{b}}} \\int_{{{c}}}^{{{d}}} {sp.latex(integrand)} \\, dy \\, dx"
        
        actual_answer = sp.integrate(sp.integrate(integrand, (y, c, d)), (x, a, b))
        
        difficulty = 1300 + 50 * (x_power + y_power)
        
    elif problem_type == 'double_integral_type1':
        a = random.randint(0, 2)
        b = random.randint(a + 1, 4)
        
        lower_power = random.randint(1, 2)
        upper_power = random.randint(1, 2)
        
        lower_bound = a * x**lower_power
        upper_bound = b * x**upper_power if upper_power != lower_power else (b + random.randint(1, 2)) * x**lower_power
        
        x_lower = 0
        x_upper = random.randint(1, 3)
        
        integrand_power_x = random.randint(0, 2)
        integrand_power_y = random.randint(0, 2)
        
        coeff = random.randint(1, 4)
        integrand = coeff * x**integrand_power_x * y**integrand_power_y
        
        question = f"\\int_{{{x_lower}}}^{{{x_upper}}} \\int_{{{sp.latex(lower_bound)}}}^{{{sp.latex(upper_bound)}}} {sp.latex(integrand)} \\, dy \\, dx"
        
        inner = sp.integrate(integrand, y)
        inner_eval = inner.subs(y, upper_bound) - inner.subs(y, lower_bound)
        actual_answer = sp.integrate(inner_eval, (x, x_lower, x_upper))
        actual_answer = sp.simplify(actual_answer)
        
        difficulty = 1450
        
    elif problem_type == 'double_integral_type2':
        c = random.randint(0, 2)
        d = random.randint(c + 1, 4)
        
        lower_power = random.randint(1, 2)
        upper_power = random.randint(1, 2)
        
        lower_bound = c * y**lower_power
        upper_bound = d * y**upper_power if upper_power != lower_power else (d + random.randint(1, 2)) * y**lower_power
        
        y_lower = 0
        y_upper = random.randint(1, 3)
        
        integrand_power_x = random.randint(0, 2)
        integrand_power_y = random.randint(0, 2)
        
        coeff = random.randint(1, 4)
        integrand = coeff * x**integrand_power_x * y**integrand_power_y
        
        question = f"\\int_{{{y_lower}}}^{{{y_upper}}} \\int_{{{sp.latex(lower_bound)}}}^{{{sp.latex(upper_bound)}}} {sp.latex(integrand)} \\, dx \\, dy"
        
        inner = sp.integrate(integrand, x)
        inner_eval = inner.subs(x, upper_bound) - inner.subs(x, lower_bound)
        actual_answer = sp.integrate(inner_eval, (y, y_lower, y_upper))
        actual_answer = sp.simplify(actual_answer)
        
        difficulty = 1450
        
    elif problem_type == 'triple_integral_rectangular':
        a = random.randint(0, 1)
        b = random.randint(a + 1, 3)
        c = random.randint(0, 1)
        d = random.randint(c + 1, 3)
        e = random.randint(0, 1)
        f = random.randint(e + 1, 3)
        
        x_power = random.randint(0, 2)
        y_power = random.randint(0, 2)
        z_power = random.randint(0, 2)
        
        coeff = random.randint(1, 3)
        integrand = coeff * x**x_power * y**y_power * z**z_power
        
        question = f"\\int_{{{a}}}^{{{b}}} \\int_{{{c}}}^{{{d}}} \\int_{{{e}}}^{{{f}}} {sp.latex(integrand)} \\, dz \\, dy \\, dx"
        
        actual_answer = sp.integrate(sp.integrate(sp.integrate(integrand, (z, e, f)), (y, c, d)), (x, a, b))
        
        difficulty = 1350 + 40 * (x_power + y_power + z_power)
        
    else:  # double_integral_polar
        r_lower = 0
        r_upper = random.randint(1, 3)
        theta_lower = 0
        theta_upper_mult = random.choice([1, 2])
        
        r, theta = sp.symbols('r theta', positive=True, real=True)
        
        r_power = random.randint(1, 3)
        coeff = random.randint(1, 4)
        
        integrand = coeff * r**r_power
        
        if theta_upper_mult == 1:
            theta_upper = sp.pi
            theta_upper_latex = "\\pi"
        else:
            theta_upper = 2 * sp.pi
            theta_upper_latex = "2\\pi"
        
        question = f"\\int_{{{theta_lower}}}^{{{theta_upper_latex}}} \\int_{{{r_lower}}}^{{{r_upper}}} {sp.latex(integrand)} \\, r \\, dr \\, d\\theta"
        
        integrand_with_jacobian = integrand * r
        actual_answer = sp.integrate(sp.integrate(integrand_with_jacobian, (r, r_lower, r_upper)), (theta, theta_lower, theta_upper))
        
        difficulty = 1500
    
    return {
        "question_latex": question,
        "answer_key": str(actual_answer),
        "difficulty": difficulty,
        "main_topic": "multivariable_calculus",