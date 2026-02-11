"""
arithmetic - percentages (hard)
Generated: 2026-02-11T21:25:48.850296
"""

import sympy as sp
import random
import json

def generate_hard_percentage_problem():
    problem_type = random.choice([
        'compound_percent_change',
        'reverse_percentage',
        'successive_discounts',
        'percent_of_percent',
        'multi_step_business'
    ])
    
    if problem_type == 'compound_percent_change':
        final_value = random.choice([120, 150, 180, 200, 240, 300, 360, 400, 450])
        increase1 = random.randint(15, 40)
        increase2 = random.randint(15, 40)
        
        original = sp.Rational(final_value * 10000, (100 + increase1) * (100 + increase2))
        
        question = f"A quantity is increased by {increase1}\\% and then the result is increased by {increase2}\\%. If the final value is {final_value}, what was the original value?"
        answer = original
        difficulty = random.randint(1650, 1750)
        
    elif problem_type == 'reverse_percentage':
        final_amount = random.choice([340, 420, 560, 680, 720, 840])
        percent_decrease = random.randint(15, 40)
        
        original = sp.Rational(final_amount * 100, 100 - percent_decrease)
        
        question = f"After a {percent_decrease}\\% decrease, a price becomes \\${final_amount}. What was the original price?"
        answer = original
        difficulty = random.randint(1600, 1680)
        
    elif problem_type == 'successive_discounts':
        original_price = random.choice([200, 250, 300, 400, 500, 600, 800])
        discount1 = random.randint(20, 40)
        discount2 = random.randint(10, 25)
        
        after_first = sp.Rational(original_price * (100 - discount1), 100)
        final_price = sp.Rational(after_first * (100 - discount2), 100)
        overall_discount_amount = original_price - final_price
        overall_discount_percent = sp.Rational(overall_discount_amount * 100, original_price)
        
        question = f"An item originally costs \\${original_price}. It receives a {discount1}\\% discount, then an additional {discount2}\\% discount is applied to the reduced price. What is the overall percentage discount from the original price?"
        answer = overall_discount_percent
        difficulty = random.randint(1700, 1800)
        
    elif problem_type == 'percent_of_percent':
        total = random.choice([800, 1000, 1200, 1500, 2000])
        percent1 = random.randint(30, 60)
        percent2 = random.randint(20, 50)
        
        first_amount = sp.Rational(total * percent1, 100)
        final_amount = sp.Rational(first_amount * percent2, 100)
        
        question = f"What is {percent2}\\% of {percent1}\\% of {total}?"
        answer = final_amount
        difficulty = random.randint(1620, 1700)
        
    else:  # multi_step_business
        cost = random.choice([60, 80, 100, 120, 150, 200])
        markup_percent = random.randint(40, 80)
        discount_percent = random.randint(15, 30)
        
        marked_price = sp.Rational(cost * (100 + markup_percent), 100)
        selling_price = sp.Rational(marked_price * (100 - discount_percent), 100)
        profit = selling_price - cost
        profit_percent = sp.Rational(profit * 100, cost)
        
        question = f"A merchant buys an item for \\${cost} and marks it up by {markup_percent}\\%. During a sale, the item is discounted by {discount_percent}\\% from the marked price. What is the profit percentage based on the cost price?"
        answer = profit_percent
        difficulty = random.randint(1750, 1850)
    
    return {
        "question_latex": question,
        "answer_key": str(answer),
        "difficulty": difficulty,
        "main_topic": "arithmetic",
        "subtopic": "percentages",
        "grading_mode": "equivalent"
    }

problem = generate_hard_percentage_problem()
print(json.dumps(problem))