"""
Word Problem: Distance = Rate × Time
Generated: Template Script
Creates word problems about distance, speed, and time with random scenarios.
"""

import sympy as sp
import random
import json

# Random parameters (reverse engineer for clean answers)
speed = random.choice([30, 40, 50, 60, 70, 80])  # km/h
time = random.randint(2, 8)  # hours
distance = speed * time  # km

# Random scenario
vehicles = ["car", "bus", "train", "bicycle", "spaceship", "boat"]
names = ["Alice", "Bob", "Chen", "Diana", "Ethan", "Fatima"]
vehicle = random.choice(vehicles)
name = random.choice(names)

# Create question
question_latex = (f"{name} travels in a {vehicle} at ${speed}$ km/h "
                  f"for ${time}$ hours. How far does {name} travel?")

# Answer
answer_key = str(distance)

# Difficulty based on numbers
difficulty = 900 + min(speed + time * 10, 500)

problem = {
    "question_latex": question_latex,
    "answer_key": answer_key,
    "difficulty": difficulty,
    "main_topic": "arithmetic",
    "subtopic": "word_problems",
    "grading_mode": "equivalent"
}

print(json.dumps(problem))
