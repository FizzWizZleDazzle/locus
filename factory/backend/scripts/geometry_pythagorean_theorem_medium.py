"""
geometry - pythagorean_theorem (medium)
Generated: 2026-02-11T21:43:52.895228
"""

import sympy as sp
import random
import json

def generate_pythagorean_problem():
    problem_type = random.choice(['find_hypotenuse', 'find_leg', 'word_problem_basic', 'word_problem_multi'])
    
    if problem_type == 'find_hypotenuse':
        # ELO: 1300-1350 (two-step: square two numbers, add, take square root)
        # Pick clean Pythagorean triple or clean result
        triples = [(3,4,5), (5,12,13), (8,15,17), (7,24,25), (6,8,10), (9,12,15), (12,16,20)]
        a, b, c = random.choice(triples)
        
        question_latex = f"A right triangle has legs of length {a} and {b}. Find the length of the hypotenuse."
        answer_key = str(c)
        difficulty = 1320
        
    elif problem_type == 'find_leg':
        # ELO: 1350-1400 (requires rearranging formula, then calculation)
        triples = [(3,4,5), (5,12,13), (8,15,17), (7,24,25), (6,8,10), (9,12,15), (12,16,20)]
        a, b, c = random.choice(triples)
        
        # Ask for one of the legs
        if random.choice([True, False]):
            question_latex = f"A right triangle has a hypotenuse of length {c} and one leg of length {a}. Find the length of the other leg."
            answer_key = str(b)
        else:
            question_latex = f"A right triangle has a hypotenuse of length {c} and one leg of length {b}. Find the length of the other leg."
            answer_key = str(a)
        
        difficulty = 1380
        
    elif problem_type == 'word_problem_basic':
        # ELO: 1450-1500 (requires translating word problem to equation, then solving)
        triples = [(3,4,5), (5,12,13), (8,15,17), (6,8,10), (9,12,15)]
        a, b, c = random.choice(triples)
        
        scenarios = [
            (f"A ladder is leaning against a wall. The base of the ladder is {a} feet from the wall, and the top of the ladder reaches {b} feet up the wall. How long is the ladder (in feet)?", c),
            (f"A rectangular field is {a} meters long and {b} meters wide. What is the length of the diagonal across the field (in meters)?", c),
            (f"A television screen is {a} inches wide and {b} inches tall. What is the diagonal measurement of the screen (in inches)?", c)
        ]
        
        scenario, ans = random.choice(scenarios)
        question_latex = scenario
        answer_key = str(ans)
        difficulty = 1470
        
    else:  # word_problem_multi
        # ELO: 1500-1550 (multi-step: extract information, set up equation, solve)
        triples = [(3,4,5), (5,12,13), (8,15,17), (6,8,10)]
        a, b, c = random.choice(triples)
        
        # Problem requiring finding one leg when given hypotenuse and other leg in context
        scenarios = [
            (f"A 13-foot ladder is leaning against a building. If the base of the ladder is 5 feet from the building, how high up the building does the ladder reach (in feet)?", 12),
            (f"A baseball diamond is a square with sides of 90 feet. What is the distance from home plate to second base (in feet)?", int(90 * sp.sqrt(2))),
        ]
        
        # Use specific values that work cleanly
        if random.choice([True, False]):
            question_latex = f"A {c}-foot ladder is leaning against a building. If the base of the ladder is {a} feet from the building, how high up the building does the ladder reach (in feet)?"
            answer_key = str(b)
            difficulty = 1520
        else:
            side = random.choice([6, 8, 9, 10, 12])
            diagonal = int(side * sp.sqrt(2))
            question_latex = f"A square has sides of length {side} units. Find the length of the diagonal. Express your answer in simplest radical form."
            answer_key = str(side * sp.sqrt(2))
            difficulty = 1550

    return {
        "question_latex": question_latex,
        "answer_key": answer_key,
        "difficulty": difficulty,
        "main_topic": "geometry",
        "subtopic": "pythagorean_theorem",
        "grading_mode": "equivalent"
    }

problem = generate_pythagorean_problem()
print(json.dumps(problem))