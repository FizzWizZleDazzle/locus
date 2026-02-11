"""
algebra2 - sequences_series (easy)
Generated: 2026-02-11T21:49:52.216578
"""

import random
import json
from sympy import symbols, sympify, latex

def generate_problem():
    problem_type = random.choice([
        'arithmetic_term',
        'arithmetic_sum_n',
        'geometric_term',
        'geometric_sum_n',
        'find_common_diff',
        'find_first_term'
    ])
    
    if problem_type == 'arithmetic_term':
        # Find nth term of arithmetic sequence
        # a_n = a_1 + (n-1)d
        a1 = random.randint(-10, 10)
        d = random.randint(1, 5)
        n = random.randint(5, 12)
        
        answer = a1 + (n - 1) * d
        
        question = f"Find the {n}\\text{{th}} term of the arithmetic sequence with first term {a1} and common difference {d}."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1100,
            "main_topic": "algebra2",
            "subtopic": "sequences_series",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'arithmetic_sum_n':
        # Sum of first n terms of arithmetic sequence
        # S_n = n/2 * (2a_1 + (n-1)d) or n/2 * (a_1 + a_n)
        a1 = random.randint(1, 10)
        d = random.randint(1, 4)
        n = random.randint(5, 10)
        
        answer = n * (2 * a1 + (n - 1) * d) // 2
        
        question = f"Find the sum of the first {n} terms of the arithmetic sequence with first term {a1} and common difference {d}."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1250,
            "main_topic": "algebra2",
            "subtopic": "sequences_series",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'geometric_term':
        # Find nth term of geometric sequence
        # a_n = a_1 * r^(n-1)
        a1 = random.choice([1, 2, 3, 5])
        r = random.choice([2, 3])
        n = random.randint(4, 6)
        
        answer = a1 * (r ** (n - 1))
        
        question = f"Find the {n}\\text{{th}} term of the geometric sequence with first term {a1} and common ratio {r}."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1150,
            "main_topic": "algebra2",
            "subtopic": "sequences_series",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'geometric_sum_n':
        # Sum of first n terms of geometric sequence
        # S_n = a_1 * (1 - r^n) / (1 - r) for r != 1
        a1 = random.choice([1, 2, 3])
        r = random.choice([2, 3])
        n = random.randint(4, 6)
        
        answer = a1 * (1 - r**n) // (1 - r)
        
        question = f"Find the sum of the first {n} terms of the geometric sequence with first term {a1} and common ratio {r}."
        
        return {
            "question_latex": question,
            "answer_key": str(answer),
            "difficulty": 1300,
            "main_topic": "algebra2",
            "subtopic": "sequences_series",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'find_common_diff':
        # Given two terms, find common difference
        a1 = random.randint(2, 15)
        d = random.randint(2, 6)
        n = random.randint(5, 10)
        an = a1 + (n - 1) * d
        
        question = f"An arithmetic sequence has first term {a1} and {n}\\text{{th}} term {an}. Find the common difference."
        
        return {
            "question_latex": question,
            "answer_key": str(d),
            "difficulty": 1200,
            "main_topic": "algebra2",
            "subtopic": "sequences_series",
            "grading_mode": "equivalent"
        }
    
    elif problem_type == 'find_first_term':
        # Given nth term and common difference, find first term
        a1 = random.randint(3, 12)
        d = random.randint(2, 5)
        n = random.randint(5, 8)
        an = a1 + (n - 1) * d
        
        question = f"An arithmetic sequence has {n}\\text{{th}} term {an} and common difference {d}. Find the first term."
        
        return {
            "question_latex": question,
            "answer_key": str(a1),
            "difficulty": 1250,
            "main_topic": "algebra2",
            "subtopic": "sequences_series",
            "grading_mode": "equivalent"
        }

problem = generate_problem()
print(json.dumps(problem))