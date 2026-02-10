"""
related_rates_harder
Generated: 2026-02-10T20:40:51.532303
"""

import sympy as sp
import random
import json

def generate_related_rates_problem():
    random.seed()
    
    # Choose problem type
    problem_type = random.choice(['ladder', 'cone', 'shadow'])
    
    if problem_type == 'ladder':
        # Ladder sliding down a wall problem with twist
        # Pick clean answer first
        answer_dh_dt = random.choice([-3, -4, -5, -6, -2])  # rate wall height changes (negative, going down)
        
        # Pick parameters
        ladder_length = random.choice([13, 17, 25, 29])  # hypotenuse
        wall_height_at_instant = random.choice([5, 12])  # height at the instant
        
        # Ensure valid triangle
        if wall_height_at_instant >= ladder_length:
            wall_height_at_instant = ladder_length - 1
        
        # Calculate base distance at that instant using Pythagorean theorem
        base_dist_at_instant = int(sp.sqrt(ladder_length**2 - wall_height_at_instant**2))
        
        # Given dx/dt (how fast base moves away), find dh/dt
        # x^2 + h^2 = L^2
        # 2x(dx/dt) + 2h(dh/dt) = 0
        # dh/dt = -(x/h)(dx/dt)
        
        # Reverse engineer dx/dt from our chosen dh/dt
        dx_dt = -(wall_height_at_instant * answer_dh_dt) / base_dist_at_instant
        
        # Make dx/dt a clean number
        if dx_dt != int(dx_dt):
            # Adjust to make it work
            dx_dt = random.choice([2, 3, 4, 5])
            answer_dh_dt = -(base_dist_at_instant * dx_dt) / wall_height_at_instant
        else:
            dx_dt = int(dx_dt)
        
        question = (
            f"A ladder ${ladder_length}$ meters long is leaning against a wall. "
            f"The bottom of the ladder is sliding away from the wall at a constant rate of ${abs(int(dx_dt))}$ m/s. "
            f"At the instant when the bottom of the ladder is ${base_dist_at_instant}$ meters from the wall, "
            f"how fast is the top of the ladder sliding down the wall? "
            f"Give your answer in m/s (use negative if moving down)."
        )
        
        answer = sp.Rational(answer_dh_dt).limit_denominator()
        
    elif problem_type == 'cone':
        # Water filling/draining cone problem
        # V = (1/3)πr^2h, with r/h = k (constant ratio)
        
        # Pick clean answer first (dh/dt at a specific height)
        answer_dh_dt = sp.Rational(random.choice([2, 3, 4, 6, 8, 9, 12]), random.choice([5, 7, 9, 11, 25]))
        
        # Pick cone dimensions (r/h ratio)
        radius_top = random.choice([3, 4, 5, 6])
        height_total = random.choice([9, 10, 12, 15, 18])
        
        k = sp.Rational(radius_top, height_total)  # r/h ratio
        
        # Pick dV/dt (rate of volume change)
        dV_dt = random.choice([2, 3, 4, 5, 6])  # cubic meters per minute
        
        # Pick height at which we want to find dh/dt
        h_instant = random.choice([3, 6, 9])
        if h_instant >= height_total:
            h_instant = height_total // 2
        
        # V = (1/3)π(kh)^2 h = (1/3)πk^2 h^3
        # dV/dt = πk^2 h^2 (dh/dt)
        # dh/dt = dV/dt / (πk^2 h^2)
        
        answer_dh_dt = sp.Rational(dV_dt) / (sp.pi * k**2 * h_instant**2)
        
        question = (
            f"Water is being pumped into an inverted conical tank at a rate of ${dV_dt}$ cubic meters per minute. "
            f"The tank has a height of ${height_total}$ meters and a radius at the top of ${radius_top}$ meters. "
            f"At what rate is the water level rising when the water is ${h_instant}$ meters deep? "
            f"Give your answer in meters per minute."
        )
        
        answer = answer_dh_dt
        
    else:  # shadow
        # Person walking away from streetlight, shadow length changing
        # Pick clean answer
        answer_ds_dt = sp.Rational(random.choice([3, 4, 5, 6, 8, 9, 10]), random.choice([2, 3, 4, 5]))
        
        # Pick parameters
        light_height = random.choice([6, 8, 10, 12])  # meters
        person_height = random.choice([2])  # meters (keep simple)
        person_speed = random.choice([2, 3, 4, 5])  # m/s away from light
        
        # By similar triangles: s/h_person = (x+s)/h_light
        # s*h_light = (x+s)*h_person
        # s(h_light - h_person) = x*h_person
        # s = x*h_person/(h_light - h_person)
        # ds/dt = (h_person/(h_light - h_person)) * dx/dt
        
        answer_ds_dt = sp.Rational(person_height * person_speed, light_height - person_height)
        
        question = (
            f"A person ${person_height}$ meters tall is walking away from a streetlight that is ${light_height}$ meters high. "
            f"The person is walking at a constant speed of ${person_speed}$ m/s. "
            f"How fast is the length of the person's shadow increasing? "
            f"Give your answer in m/s."
        )
        
        answer = answer_ds_dt
    
    difficulty = random.randint(2500, 3000)
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "calculus",
        "subtopic": "Related Rates",
        "grading_mode": "equivalent"
    }

problem = generate_related_rates_problem()
print(json.dumps(problem))