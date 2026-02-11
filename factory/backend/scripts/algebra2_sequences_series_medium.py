"""
algebra2 - sequences_series (medium)
Generated: 2026-02-11T21:50:05.301210
"""

import sympy as sp
import random
import json

def generate_problem():
    problem_type = random.choice([
        'arithmetic_nth_term',
        'arithmetic_sum',
        'geometric_nth_term',
        'geometric_sum',
        'arithmetic_word_problem',
        'geometric_word_problem'
    ])
    
    if problem_type == 'arithmetic_nth_term':
        a1 = random.randint(-10, 10)
        d = random.randint(2, 8)
        n = random.randint(8, 20)
        answer = a1 + (n - 1) * d
        question = f"Find the {n}th term of the arithmetic sequence with first term {a1} and common difference {d}."
        difficulty = 1300
        
    elif problem_type == 'arithmetic_sum':
        a1 = random.randint(1, 10)
        d = random.randint(2, 6)
        n = random.randint(10, 25)
        an = a1 + (n - 1) * d
        answer = n * (a1 + an) // 2
        question = f"Find the sum of the first {n} terms of the arithmetic sequence with first term {a1} and common difference {d}."
        difficulty = 1400
        
    elif problem_type == 'geometric_nth_term':
        a1 = random.randint(2, 8)
        r = random.choice([2, 3, -2])
        n = random.randint(5, 8)
        answer = a1 * (r ** (n - 1))
        question = f"Find the {n}th term of the geometric sequence with first term {a1} and common ratio {r}."
        difficulty = 1350
        
    elif problem_type == 'geometric_sum':
        a1 = random.randint(2, 6)
        r = random.choice([2, 3])
        n = random.randint(5, 8)
        answer = a1 * (r**n - 1) // (r - 1)
        question = f"Find the sum of the first {n} terms of the geometric sequence with first term {a1} and common ratio {r}."
        difficulty = 1450
        
    elif problem_type == 'arithmetic_word_problem':
        initial = random.randint(15, 30) * 1000
        increase = random.randint(2, 5) * 1000
        years = random.randint(8, 12)
        salary_year = initial + (years - 1) * increase
        answer = salary_year
        question = f"A person's starting salary is \\${initial}. If they receive an annual raise of \\${increase}, what will their salary be in year {years}?"
        difficulty = 1500
        
    else:  # geometric_word_problem
        initial = random.choice([100, 200, 500])
        r_percent = random.choice([50, 100])
        n = random.randint(4, 6)
        r = 1 + r_percent / 100
        answer_float = initial * (r ** (n - 1))
        answer = int(answer_float)
        question = f"A population starts at {initial} and increases by {r_percent}\\% each year. What will the population be at the start of year {n}?"
        difficulty = 1550
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "sequences_series",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))