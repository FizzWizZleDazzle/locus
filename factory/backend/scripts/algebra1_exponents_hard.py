"""
algebra1 - exponents (hard)
Generated: 2026-02-11T21:36:35.683344
"""

import random
import json
from sympy import symbols, simplify, latex, powdenest, expand, factor, sqrt, Rational

def generate_hard_exponent_problem():
    x, y, a, b = symbols('x y a b', positive=True, real=True)
    
    problem_type = random.choice([
        'rational_exponents_complex',
        'negative_exponents_multistep',
        'power_of_power_algebraic',
        'combined_rules_complex',
        'simplify_radical_exponents',
        'mixed_operations_hard'
    ])
    
    if problem_type == 'rational_exponents_complex':
        # (x^(a/b))^c * x^(d/e) type problems
        num1, denom1 = random.choice([(2,3), (3,4), (4,3), (3,2), (5,2)])
        num2, denom2 = random.choice([(1,2), (1,3), (2,3), (3,4)])
        mult = random.randint(2, 4)
        
        # Work backwards: final answer x^(p/q)
        final_num = num1 * mult + num2 * denom1
        final_denom = denom1
        from math import gcd
        g = gcd(final_num, final_denom)
        final_num //= g
        final_denom //= g
        
        question = f"\\left(x^{{\\frac{{{num1}}}{{{denom1}}}}}\\right)^{{{mult}}} \\cdot x^{{\\frac{{{num2}}}{{{denom2}}}}}"
        if final_denom == 1:
            answer = f"x**{final_num}"
        else:
            answer = f"x**({final_num}/{final_denom})"
        difficulty = 1650
        
    elif problem_type == 'negative_exponents_multistep':
        # (x^-a / y^-b)^c type
        exp_a = random.randint(2, 4)
        exp_b = random.randint(2, 4)
        power = random.randint(2, 3)
        
        final_x_exp = exp_a * power
        final_y_exp = exp_b * power
        
        question = f"\\left(\\frac{{x^{{-{exp_a}}}}}{{y^{{-{exp_b}}}}}\\right)^{{{power}}}"
        answer = f"y**{final_y_exp}/x**{final_x_exp}"
        difficulty = 1700
        
    elif problem_type == 'power_of_power_algebraic':
        # ((x^a * y^b)^c) / (x^d * y^e)
        a, b = random.randint(2, 4), random.randint(2, 3)
        c = random.randint(2, 3)
        d, e = random.randint(1, 3), random.randint(1, 2)
        
        final_x = a * c - d
        final_y = b * c - e
        
        question = f"\\frac{{\\left(x^{{{a}}} y^{{{b}}}\\right)^{{{c}}}}}{{x^{{{d}}} y^{{{e}}}}}"
        
        if final_x > 0 and final_y > 0:
            answer = f"x**{final_x}*y**{final_y}"
        elif final_x > 0 and final_y <= 0:
            answer = f"x**{final_x}/y**{abs(final_y)}" if final_y < 0 else f"x**{final_x}"
        elif final_x <= 0 and final_y > 0:
            answer = f"y**{final_y}/x**{abs(final_x)}" if final_x < 0 else f"y**{final_y}"
        else:
            answer = f"1/(x**{abs(final_x)}*y**{abs(final_y)})"
        
        difficulty = 1750
        
    elif problem_type == 'combined_rules_complex':
        # (a^m * b^n)^p / (a^q * b^r)^s
        m, n = random.randint(2, 3), random.randint(2, 3)
        p = random.randint(2, 3)
        q, r = random.randint(1, 2), random.randint(1, 2)
        s = random.randint(2, 3)
        
        final_a = m * p - q * s
        final_b = n * p - r * s
        
        question = f"\\frac{{\\left(a^{{{m}}} b^{{{n}}}\\right)^{{{p}}}}}{{\\left(a^{{{q}}} b^{{{r}}}\\right)^{{{s}}}}}"
        
        if final_a > 0 and final_b > 0:
            answer = f"a**{final_a}*b**{final_b}"
        elif final_a > 0 and final_b <= 0:
            answer = f"a**{final_a}/b**{abs(final_b)}" if final_b < 0 else f"a**{final_a}"
        elif final_a <= 0 and final_b > 0:
            answer = f"b**{final_b}/a**{abs(final_a)}" if final_a < 0 else f"b**{final_b}"
        else:
            if final_a == 0 and final_b == 0:
                answer = "1"
            elif final_a == 0:
                answer = f"1/b**{abs(final_b)}"
            elif final_b == 0:
                answer = f"1/a**{abs(final_a)}"
            else:
                answer = f"1/(a**{abs(final_a)}*b**{abs(final_b)})"
        
        difficulty = 1800
        
    elif problem_type == 'simplify_radical_exponents':
        # Convert between radical and exponential form
        base_inner = random.randint(2, 4)
        root = random.choice([2, 3, 4])
        power = random.randint(2, 3)
        
        final_exp_num = power
        final_exp_denom = root
        from math import gcd
        g = gcd(final_exp_num, final_exp_denom)
        final_exp_num //= g
        final_exp_denom //= g
        
        if root == 2:
            question = f"\\left(\\sqrt{{x^{{{base_inner}}}}}\\right)^{{{power}}}"
        else:
            question = f"\\left(\\sqrt[{root}]{{x^{{{base_inner}}}}}\\right)^{{{power}}}"
        
        total_exp = base_inner * power
        final_num = total_exp
        final_denom = root
        g = gcd(final_num, final_denom)
        final_num //= g
        final_denom //= g
        
        if final_denom == 1:
            answer = f"x**{final_num}"
        else:
            answer = f"x**({final_num}/{final_denom})"
        
        difficulty = 1680
        
    else:  # mixed_operations_hard
        # (x^a)^b * (x^c)^d / x^e
        a, b = random.randint(2, 3), random.randint(2, 3)
        c, d = random.randint(1, 2), random.randint(2, 3)
        e = random.randint(