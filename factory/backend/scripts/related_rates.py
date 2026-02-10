"""
related_rates
Generated: 2026-02-10T20:22:43.933929
"""

import random
import json
from sympy import *

def generate_related_rates_problem():
    random.seed()
    
    problem_type = random.choice(['ladder', 'cone', 'rectangle', 'shadow'])
    
    if problem_type == 'ladder':
        # Ladder sliding down wall problem
        # Pick clean answer for dy/dt
        dy_dt_answer = -random.choice([1, 2, 3, 4, 5])  # negative since sliding down
        
        # Pick ladder length (hypotenuse)
        L = random.choice([13, 15, 17, 20, 25])
        
        # Pick x position
        x = random.choice([5, 9, 12, 15])
        if x >= L:
            x = L - random.randint(3, 8)
        
        # Calculate y from Pythagorean theorem
        y_val = int(sqrt(L**2 - x**2))
        
        # Pick dx/dt
        dx_dt = random.choice([1, 2, 3, 4])
        
        # From x^2 + y^2 = L^2, differentiate: 2x(dx/dt) + 2y(dy/dt) = 0
        # dy/dt = -x(dx/dt)/y
        dy_dt_answer = Rational(-x * dx_dt, y_val)
        
        question = f"A ladder of length ${L}$ meters is leaning against a wall. "
        question += f"The bottom of the ladder is sliding away from the wall at a rate of ${dx_dt}$ m/s. "
        question += f"At what rate is the top of the ladder sliding down the wall when the bottom is ${x}$ meters from the wall? "
        question += f"(Give your answer in m/s, negative indicates downward motion)"
        
        answer_key = str(dy_dt_answer)
        difficulty = random.randint(1100, 1300)
        
    elif problem_type == 'cone':
        # Water draining from cone
        # Pick dV/dt (rate of volume change)
        dV_dt = -random.choice([2, 3, 4, 5, 6])  # negative for draining
        
        # Cone dimensions (radius/height ratio)
        r_ratio = random.choice([1, 2, 3])
        h_ratio = random.choice([2, 3, 4])
        
        # Height at which we measure
        h_val = random.choice([4, 6, 8, 10])
        
        # V = (1/3)πr²h, with r = (r_ratio/h_ratio)h
        # V = (1/3)π(r_ratio/h_ratio)²h³
        # dV/dt = π(r_ratio/h_ratio)²h²(dh/dt)
        
        # dh/dt = dV/dt / [π(r_ratio/h_ratio)²h²]
        dh_dt_answer = Rational(dV_dt * h_ratio**2, pi * r_ratio**2 * h_val**2)
        
        question = f"Water is draining from a conical tank at a rate of ${abs(dV_dt)}$ cubic meters per minute. "
        question += f"The tank has a height-to-radius ratio of ${h_ratio}:{r_ratio}$. "
        question += f"At what rate is the water level falling when the water is ${h_val}$ meters deep? "
        question += f"(Express your answer as a rational multiple of $\\pi$, in m/min)"
        
        # Simplify the answer to be in form a/b where we divide by pi separately
        numerator = dV_dt * h_ratio**2
        denominator = r_ratio**2 * h_val**2
        answer_key = str(Rational(numerator, denominator)) + "/pi"
        answer_key = str(Rational(numerator, denominator * pi))
        
        # Actually, let's make this simpler
        dh_dt_numerical = float(dV_dt * h_ratio**2) / float(pi * r_ratio**2 * h_val**2)
        answer_key = str(Rational(dV_dt * h_ratio**2, r_ratio**2 * h_val**2).limit_denominator()) + "/pi"
        
        difficulty = random.randint(1200, 1400)
        
    elif problem_type == 'rectangle':
        # Expanding rectangle
        # Pick rates
        dx_dt = random.choice([2, 3, 4, 5])
        dy_dt = random.choice([1, 2, 3])
        
        # Pick dimensions at measurement time
        x_val = random.choice([5, 6, 8, 10])
        y_val = random.choice([4, 5, 6, 8])
        
        # A = xy, dA/dt = x(dy/dt) + y(dx/dt)
        dA_dt_answer = x_val * dy_dt + y_val * dx_dt
        
        question = f"The length of a rectangle is increasing at ${dx_dt}$ cm/s "
        question += f"and the width is increasing at ${dy_dt}$ cm/s. "
        question += f"At what rate is the area increasing when the length is ${x_val}$ cm "
        question += f"and the width is ${y_val}$ cm? (Give your answer in cm²/s)"
        
        answer_key = str(dA_dt_answer)
        difficulty = random.randint(1000, 1200)
        
    else:  # shadow
        # Person walking away from streetlight
        # Pick heights
        light_height = random.choice([15, 18, 20, 24])
        person_height = random.choice([5, 6])
        
        # Pick walking speed
        dx_dt = random.choice([2, 3, 4, 5])
        
        # Shadow length s satisfies: s/(s+x) = person_height/light_height
        # s*light_height = person_height*(s+x)
        # s*(light_height - person_height) = person_height*x
        # s = person_height*x/(light_height - person_height)
        # ds/dt = person_height*dx_dt/(light_height - person_height)
        
        ds_dt_answer = Rational(person_height * dx_dt, light_height - person_height)
        
        question = f"A person ${person_height}$ feet tall is walking away from a streetlight that is ${light_height}$ feet tall "
        question += f"at a rate of ${dx_dt}$ ft/s. "
        question += f"At what rate is the length of the person's shadow increasing? (Give your answer in ft/s)"
        
        answer_key = str(ds_dt_answer)
        difficulty = random.randint(1150, 1350)
    
    return {
        "question_latex": question,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "calculus",
        "subtopic": "Related Rates",
        "grading_mode": "equivalent"
    }

print(json.dumps(generate_related_rates_problem()))