#!/usr/bin/env python3
"""
Example Problem Generator Script: Arithmetic Addition

This script generates simple addition problems for arithmetic practice.
It can be used as a template for creating other problem types.
"""

import random
import json

# Generate random numbers for addition
num1 = random.randint(1, 100)
num2 = random.randint(1, 100)

# Calculate answer
answer = num1 + num2

# Determine difficulty based on number size
# Smaller numbers = easier, larger numbers = harder
avg_size = (num1 + num2) / 2
if avg_size < 20:
    difficulty = random.randint(800, 1000)
elif avg_size < 50:
    difficulty = random.randint(1000, 1200)
else:
    difficulty = random.randint(1200, 1500)

# Create problem
problem = {
    "question_latex": f"${num1} + {num2}$",
    "answer_key": str(answer),
    "difficulty": difficulty,
    "main_topic": "arithmetic",
    "subtopic": "addition_subtraction",
    "grading_mode": "equivalent"
}

# Output JSON
print(json.dumps(problem))
