"""
geometry - angles (easy)
Generated: 2026-02-11T21:39:08.654473
"""

import random
import json
from sympy import symbols, Eq, solve, simplify, latex, Integer, Rational

def generate_angle_problem():
    problem_type = random.randint(1, 6)
    
    if problem_type == 1:
        # Type 1: Find supplementary angle (1050-1150 ELO)
        answer = random.randint(30, 150)
        given_angle = 180 - answer
        
        question = f"Two angles are supplementary. If one angle measures ${given_angle}^\\circ$, what is the measure of the other angle in degrees?"
        difficulty = random.randint(1050, 1150)
        
    elif problem_type == 2:
        # Type 2: Find complementary angle (1050-1150 ELO)
        answer = random.randint(20, 70)
        given_angle = 90 - answer
        
        question = f"Two angles are complementary. If one angle measures ${given_angle}^\\circ$, what is the measure of the other angle in degrees?"
        difficulty = random.randint(1050, 1150)
        
    elif problem_type == 3:
        # Type 3: Missing angle in triangle with two given (1100-1200 ELO)
        answer = random.randint(30, 90)
        angle1 = random.randint(30, 80)
        angle2 = 180 - answer - angle1
        
        question = f"In a triangle, two angles measure ${angle1}^\\circ$ and ${angle2}^\\circ$. What is the measure of the third angle in degrees?"
        difficulty = random.randint(1100, 1200)
        
    elif problem_type == 4:
        # Type 4: Vertical angles (1000-1100 ELO)
        answer = random.randint(35, 145)
        
        question = f"Two lines intersect forming vertical angles. If one angle measures ${answer}^\\circ$, what is the measure of the angle vertically opposite to it in degrees?"
        difficulty = random.randint(1000, 1100)
        
    elif problem_type == 5:
        # Type 5: Linear pair with simple equation (1200-1300 ELO)
        x_value = random.randint(10, 40)
        answer = 3 * x_value + random.randint(5, 20)
        other_angle = 180 - answer
        coefficient = 3
        constant = answer - coefficient * x_value
        
        question = f"Two angles form a linear pair. One angle measures ${coefficient}x + {constant}$ degrees and the other measures ${other_angle}^\\circ$. Find the value of the first angle in degrees."
        difficulty = random.randint(1200, 1300)
        
    else:
        # Type 6: Sum of angles around a point (1150-1250 ELO)
        answer = random.randint(40, 100)
        angle1 = random.randint(50, 100)
        angle2 = random.randint(50, 100)
        angle3 = 360 - answer - angle1 - angle2
        
        question = f"Four angles meet at a point. Three of the angles measure ${angle1}^\\circ$, ${angle2}^\\circ$, and ${angle3}^\\circ$. What is the measure of the fourth angle in degrees?"
        difficulty = random.randint(1150, 1250)
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "angles",
        "grading_mode": "equivalent"
    }

problem = generate_angle_problem()
print(json.dumps(problem))