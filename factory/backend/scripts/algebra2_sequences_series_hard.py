"""
algebra2 - sequences_series (hard)
Generated: 2026-02-11T21:50:24.922476
"""

import random
import json
from sympy import symbols, simplify, expand, solve, sqrt, Rational, factorial, summation, Symbol

def generate_problem():
    n, k, i = symbols('n k i')
    problem_type = random.choice([
        'arithmetic_sum_find_n',
        'geometric_sum_formula',
        'recursive_sequence',
        'mixed_sequence',
        'sum_of_squares',
        'arithmetic_geometric_combined'
    ])
    
    if problem_type == 'arithmetic_sum_find_n':
        # Given sum, first term, last term, find n
        # S_n = n/2 * (a_1 + a_n)
        target_n = random.randint(15, 40)
        a1 = random.randint(-10, 10)
        d = random.randint(2, 8)
        an = a1 + (target_n - 1) * d
        target_sum = target_n * (a1 + an) // 2
        
        question = f"An arithmetic sequence has first term $a_1 = {a1}$, last term $a_n = {an}$, and the sum of all terms is ${target_sum}$. Find the number of terms $n$."
        answer = str(target_n)
        difficulty = 1650
        
    elif problem_type == 'geometric_sum_formula':
        # Sum of geometric series with algebra
        a = random.randint(2, 5)
        r_num = random.randint(2, 4)
        r_den = random.randint(r_num + 1, r_num + 3)
        terms = random.randint(8, 12)
        
        # S_n = a(1 - r^n)/(1 - r)
        r_frac = Rational(r_num, r_den)
        sum_val = a * (1 - r_frac**terms) / (1 - r_frac)
        sum_val = simplify(sum_val)
        
        question = f"Find the sum of the first ${terms}$ terms of the geometric sequence with first term ${a}$ and common ratio $\\frac{{{r_num}}}{{{r_den}}}$."
        answer = str(sum_val)
        difficulty = 1620
        
    elif problem_type == 'recursive_sequence':
        # a_n = 2a_{n-1} + k, find a_n given a_1 and n
        a1_val = random.randint(1, 5)
        k_val = random.randint(1, 4)
        target_term = random.randint(4, 6)
        
        # Calculate explicitly
        current = a1_val
        for _ in range(1, target_term):
            current = 2 * current + k_val
        
        question = f"A sequence is defined recursively by $a_1 = {a1_val}$ and $a_n = 2a_{{n-1}} + {k_val}$ for $n \\geq 2$. Find $a_{{{target_term}}}$."
        answer = str(current)
        difficulty = 1680
        
    elif problem_type == 'mixed_sequence':
        # Arithmetic series where we need to find d given S_n, a_1, and n
        n_val = random.randint(10, 20)
        a1_val = random.randint(2, 8)
        d_val = random.randint(3, 7)
        
        # S_n = n/2 * (2a_1 + (n-1)d)
        sum_val = n_val * (2 * a1_val + (n_val - 1) * d_val) // 2
        
        question = f"An arithmetic sequence has first term $a_1 = {a1_val}$ and the sum of the first ${n_val}$ terms is ${sum_val}$. Find the common difference $d$."
        answer = str(d_val)
        difficulty = 1700
        
    elif problem_type == 'sum_of_squares':
        # Sum of squares formula: sum(k^2) = n(n+1)(2n+1)/6
        n_val = random.randint(8, 15)
        sum_val = n_val * (n_val + 1) * (2 * n_val + 1) // 6
        
        question = f"Find the sum $1^2 + 2^2 + 3^2 + \\cdots + {n_val}^2$."
        answer = str(sum_val)
        difficulty = 1750
        
    else:  # arithmetic_geometric_combined
        # Find n such that sum of arithmetic sequence equals a specific value
        a1_val = random.randint(3, 8)
        d_val = random.randint(2, 5)
        target_n = random.randint(12, 18)
        
        # Calculate what the sum would be
        target_sum = target_n * (2 * a1_val + (target_n - 1) * d_val) // 2
        
        question = f"For an arithmetic sequence with first term ${a1_val}$ and common difference ${d_val}$, find the smallest value of $n$ such that the sum of the first $n$ terms equals ${target_sum}$."
        answer = str(target_n)
        difficulty = 1820
    
    return {
        "question_latex": question,
        "answer_key": answer,
        "difficulty": difficulty,
        "main_topic": "algebra2",
        "subtopic": "sequences_series",
        "grading_mode": "equivalent"
    }

problem = generate_problem()
print(json.dumps(problem))