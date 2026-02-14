# Factory Developer Guide

Complete guide for creating custom problem generators for the Locus platform.

## Overview

The Factory system generates mathematical problems using **AI-assisted Python scripts**. This guide shows you how to:
- Create custom problem generators
- Use SymPy for symbolic mathematics
- Design effective LLM prompts
- Validate generated problems
- Integrate with the Locus database

## Quick Start

### 1. Create a New Generator Script

```python
#!/usr/bin/env python3
import sympy as sp
import random
import json

# Your logic here

problem = {
    "question_latex": "...",
    "answer_key": "...",
    "difficulty": 1200,
    "main_topic": "...",
    "subtopic": "...",
    "grading_mode": "equivalent"
}

print(json.dumps(problem))
```

### 2. Test the Script

```bash
python your_script.py
# Should output valid JSON
```

### 3. Use in Factory UI

1. Open Factory web interface
2. Paste script into editor
3. Click "Test Script"
4. If successful, click "Batch Generate"

## Problem Structure

### Required Fields

```python
problem = {
    # LaTeX-formatted question (displayed to user)
    "question_latex": "Solve: $x^2 - 4 = 0$",

    # Expected answer (string format for SymEngine)
    "answer_key": "2",

    # ELO difficulty (800-2400)
    "difficulty": 1200,

    # Main topic (see Topics section)
    "main_topic": "algebra1",

    # Subtopic (see Topics section)
    "subtopic": "quadratic_equations",

    # Grading mode: "equivalent", "factor", or "expand"
    "grading_mode": "equivalent"
}
```

### Field Specifications

#### question_latex
- **Format:** LaTeX math inside `$...$` or `$$...$$`
- **Purpose:** Rendered with KaTeX in frontend
- **Examples:**
  - Inline: `"What is $2 + 2$?"`
  - Display: `"Solve: $$x^2 - 1 = 0$$"`
  - Fractions: `"Simplify: $\\frac{x^2 - 1}{x + 1}$"`
  - Derivatives: `"Find $\\frac{d}{dx}(x^3)$"`

**Common LaTeX Commands:**
```latex
\frac{a}{b}       # Fraction
x^2               # Superscript
x_1               # Subscript
\sqrt{x}          # Square root
\left(...\right)  # Auto-sized parentheses
\sin, \cos, \tan  # Trig functions
\log, \ln         # Logarithms
\int, \sum        # Integral, summation
```

#### answer_key
- **Format:** String parseable by SymEngine
- **Purpose:** Used for answer validation
- **Examples:**
  - Number: `"42"`
  - Expression: `"2*x + 1"`
  - Fraction: `"1/2"`
  - Factored: `"(x+1)*(x-1)"`

**Important:** Use SymPy's `str()` to convert symbolic expressions:
```python
derivative = sp.diff(expr, x)
answer_key = str(derivative)  # "2*x + 3"
```

#### difficulty
- **Range:** 800-2400 (ELO rating)
- **Guidelines:**
  - 800-1000: Elementary arithmetic
  - 1000-1200: Basic algebra
  - 1200-1500: Intermediate algebra/geometry
  - 1500-1800: Advanced algebra/precalculus
  - 1800-2100: Calculus
  - 2100-2400: Advanced calculus/linear algebra

**Dynamic Difficulty:**
```python
# Base difficulty on problem complexity
if degree == 1:
    difficulty = random.randint(1000, 1200)
elif degree == 2:
    difficulty = random.randint(1200, 1500)
else:
    difficulty = random.randint(1500, 1800)
```

#### main_topic
Valid values:
- `"arithmetic"`
- `"algebra1"`
- `"geometry"`
- `"algebra2"`
- `"precalculus"`
- `"calculus"`
- `"multivariable_calculus"`
- `"linear_algebra"`

#### subtopic
**Arithmetic:**
- `"addition_subtraction"`
- `"multiplication_division"`
- `"fractions"`
- `"decimals"`

**Algebra 1:**
- `"linear_equations"`
- `"systems_of_equations"`
- `"polynomials"`
- `"quadratic_equations"`

**Geometry:**
- `"triangles"`
- `"circles"`
- `"area_perimeter"`
- `"coordinate_geometry"`

**Algebra 2:**
- `"exponentials"`
- `"logarithms"`
- `"rational_functions"`
- `"sequences_series"`

**Precalculus:**
- `"trig_functions"`
- `"trig_identities"`
- `"polar_coordinates"`
- `"complex_numbers"`

**Calculus:**
- `"limits"`
- `"derivatives"`
- `"integrals"`
- `"applications"`

**Multivariable Calculus:**
- `"partial_derivatives"`
- `"multiple_integrals"`
- `"vector_calculus"`

**Linear Algebra:**
- `"matrices"`
- `"determinants"`
- `"eigenvalues"`
- `"vector_spaces"`

#### grading_mode

**"equivalent"** - Accept any mathematically correct form:
```python
"grading_mode": "equivalent"
# Accepts: x^2 - 1, (x+1)(x-1), x*x - 1, -1 + x^2
```

**"factor"** - Require factored form:
```python
"grading_mode": "factor"
# Accepts: (x+1)(x-1), (x-1)(x+1)
# Rejects: x^2 - 1
```

**"expand"** - Require expanded form:
```python
"grading_mode": "expand"
# Accepts: x^2 - 1, x*x - 1, -1 + x^2
# Rejects: (x+1)(x-1)
```

## Using SymPy

### Basic Setup

```python
import sympy as sp

# Define symbols
x = sp.Symbol('x')
y = sp.Symbol('y')
t = sp.Symbol('t')

# Or multiple at once
x, y, z = sp.symbols('x y z')
```

### Creating Expressions

```python
# Polynomials
expr = x**2 + 2*x + 1
expr = sp.Poly(x**2 + 2*x + 1, x)

# Fractions
expr = sp.Rational(1, 2)  # Exact: 1/2
expr = 1/2                # Float: 0.5

# Trig functions
expr = sp.sin(x) + sp.cos(x)
expr = sp.tan(2*x)

# Exponentials and logs
expr = sp.exp(x)
expr = sp.log(x)  # Natural log
expr = sp.log(x, 10)  # Log base 10

# Radicals
expr = sp.sqrt(x)
expr = sp.root(x, 3)  # Cube root
```

### Manipulating Expressions

```python
# Expand
expr = (x + 1)*(x - 1)
expanded = sp.expand(expr)  # x**2 - 1

# Factor
expr = x**2 - 1
factored = sp.factor(expr)  # (x - 1)*(x + 1)

# Simplify
expr = (x**2 - 1)/(x + 1)
simplified = sp.simplify(expr)  # x - 1

# Solve equations
solutions = sp.solve(x**2 - 4, x)  # [-2, 2]

# Differentiate
derivative = sp.diff(x**3, x)  # 3*x**2

# Integrate
integral = sp.integrate(x**2, x)  # x**3/3
definite = sp.integrate(x**2, (x, 0, 1))  # 1/3
```

### Converting to LaTeX

```python
expr = x**2 + 2*x + 1

# Basic LaTeX
latex_str = sp.latex(expr)  # "x^{2} + 2 x + 1"

# In question
question_latex = f"Simplify: ${sp.latex(expr)}$"

# Fractions in display mode
fraction = sp.Rational(1, 2)
question_latex = f"What is $$\\frac{{1}}{{2}} + \\frac{{1}}{{3}}$$?"
```

### Converting to String (for answer_key)

```python
expr = sp.diff(x**3, x)

# Convert to string
answer_key = str(expr)  # "3*x**2"

# Explicit SymEngine format
answer_key = sp.srepr(expr)  # More explicit form
```

## Example Generators

### Example 1: Linear Equations

```python
#!/usr/bin/env python3
import sympy as sp
import random
import json

x = sp.Symbol('x')

# Generate random linear equation: ax + b = c
a = random.randint(2, 10)
b = random.randint(-20, 20)
c = random.randint(-20, 20)

# Solve for x
solution = sp.solve(a*x + b - c, x)[0]

# Create question
lhs = sp.latex(a*x + b)
question_latex = f"Solve for $x$: ${lhs} = {c}$"

# Answer as fraction or integer
if solution.is_integer:
    answer_key = str(int(solution))
else:
    answer_key = str(solution)

problem = {
    "question_latex": question_latex,
    "answer_key": answer_key,
    "difficulty": random.randint(1000, 1200),
    "main_topic": "algebra1",
    "subtopic": "linear_equations",
    "grading_mode": "equivalent"
}

print(json.dumps(problem))
```

### Example 2: Quadratic Factoring

```python
#!/usr/bin/env python3
import sympy as sp
import random
import json

x = sp.Symbol('x')

# Generate factorable quadratic: (x + a)(x + b)
a = random.randint(-10, 10)
b = random.randint(-10, 10)

# Expanded form
expanded = sp.expand((x + a) * (x + b))

# Factored form (answer)
factored = sp.factor(expanded)

question_latex = f"Factor completely: ${sp.latex(expanded)}$"
answer_key = str(factored)

problem = {
    "question_latex": question_latex,
    "answer_key": answer_key,
    "difficulty": random.randint(1200, 1500),
    "main_topic": "algebra1",
    "subtopic": "quadratic_equations",
    "grading_mode": "factor"  # Require factored form
}

print(json.dumps(problem))
```

### Example 3: Derivative with Product Rule

```python
#!/usr/bin/env python3
import sympy as sp
import random
import json

x = sp.Symbol('x')

# Generate two simpler functions
a = random.randint(1, 5)
n = random.randint(2, 4)

f = a * x**n
g = sp.sin(x)

# Product
product = f * g

# Derivative
derivative = sp.diff(product, x)

question_latex = f"Find $\\frac{{d}}{{dx}}\\left({sp.latex(product)}\\right)$"
answer_key = str(derivative)

problem = {
    "question_latex": question_latex,
    "answer_key": answer_key,
    "difficulty": random.randint(1500, 1800),
    "main_topic": "calculus",
    "subtopic": "derivatives",
    "grading_mode": "equivalent"
}

print(json.dumps(problem))
```

### Example 4: Geometric Problem

```python
#!/usr/bin/env python3
import random
import json
import math

# Right triangle with sides a, b, hypotenuse c
a = random.randint(3, 12)
b = random.randint(3, 12)

# Pythagorean theorem
c = math.sqrt(a**2 + b**2)

# Round if very close to integer
if abs(c - round(c)) < 0.01:
    c = round(c)
    answer_key = str(int(c))
else:
    # Simplify radical if possible
    import sympy as sp
    c_exact = sp.sqrt(a**2 + b**2)
    answer_key = str(c_exact)

question_latex = (
    f"A right triangle has legs of length ${a}$ and ${b}$. "
    f"Find the length of the hypotenuse."
)

problem = {
    "question_latex": question_latex,
    "answer_key": answer_key,
    "difficulty": random.randint(1100, 1400),
    "main_topic": "geometry",
    "subtopic": "triangles",
    "grading_mode": "equivalent"
}

print(json.dumps(problem))
```

## AI Prompt Engineering

### Prompting the LLM

The Factory uses AI to generate generator scripts. Effective prompts:

**Good Prompt:**
```
Create a Python script that generates problems for simplifying rational expressions.
The expressions should be quotients of polynomials that can be simplified by
factoring and canceling common terms. Use SymPy. Include random coefficients
between -10 and 10. Difficulty should be 1400-1700. Main topic is algebra2,
subtopic is rational_functions. Use grading_mode "equivalent".
```

**Elements of Good Prompts:**
1. **Specific problem type** - "simplifying rational expressions"
2. **Constraints** - "coefficients between -10 and 10"
3. **Tool usage** - "Use SymPy"
4. **Difficulty range** - "1400-1700"
5. **Topic info** - "algebra2, rational_functions"
6. **Grading mode** - "equivalent"

**Template:**
```
Create a Python script that generates [PROBLEM_TYPE] problems.
[DESCRIPTION OF PROBLEM STRUCTURE]
Use SymPy for symbolic math.
Include randomization with [CONSTRAINTS].
Difficulty should be [MIN-MAX].
Main topic: [TOPIC]
Subtopic: [SUBTOPIC]
Grading mode: [MODE]
Output valid JSON with required fields.
```

### Iterating on Generated Scripts

1. **Test the script**:
   ```bash
   python generated_script.py
   ```

2. **Check output**:
   - Valid JSON?
   - LaTeX renders correctly?
   - Answer is correct?
   - Difficulty appropriate?

3. **Refine prompt** if needed:
   - "Make expressions more complex"
   - "Avoid negative coefficients"
   - "Include mixed fractions"

4. **Manual edits**:
   - Fix LaTeX formatting
   - Adjust difficulty ranges
   - Add edge case handling

## Validation Best Practices

### 1. Self-Check Answers

```python
# Generate problem
expr = a*x**2 + b*x + c
derivative = sp.diff(expr, x)

# Verify answer
expected = 2*a*x + b
assert sp.simplify(derivative - expected) == 0, "Derivative incorrect!"

answer_key = str(derivative)
```

### 2. Validate Difficulty

```python
def estimate_difficulty(expr, operation):
    """Estimate problem difficulty based on complexity."""
    degree = sp.degree(expr)
    num_terms = len(sp.Add.make_args(expr))

    base_difficulty = {
        'derivative': 1500,
        'integral': 1600,
        'simplify': 1200,
    }[operation]

    # Adjust for complexity
    difficulty = base_difficulty
    difficulty += (degree - 1) * 100
    difficulty += (num_terms - 2) * 50

    # Clamp to valid range
    return max(800, min(2400, difficulty))
```

### 3. Avoid Degenerate Cases

```python
# Bad: Could generate 0*x = 0
a = random.randint(0, 10)

# Good: Never zero
a = random.randint(1, 10)

# Bad: Could generate 0/0
numerator = random.randint(-5, 5)
denominator = random.randint(-5, 5)

# Good: Avoid zero denominator
numerator = random.randint(-5, 5)
denominator = random.choice([i for i in range(-5, 6) if i != 0])
```

### 4. Test Edge Cases

```python
# Test your script multiple times
for _ in range(10):
    # Run your generation logic
    problem = generate_problem()

    # Validate
    assert problem['answer_key'], "Empty answer!"
    assert 800 <= problem['difficulty'] <= 2400, "Invalid difficulty!"
    assert '$' in problem['question_latex'], "Missing LaTeX delimiters!"
```

## Common Patterns

### Random Selection

```python
# Random choice
operation = random.choice(['+', '-', '*', '/'])
function = random.choice([sp.sin, sp.cos, sp.tan])

# Weighted random
difficulty_weights = {
    'easy': 0.5,
    'medium': 0.3,
    'hard': 0.2,
}
difficulty = random.choices(
    ['easy', 'medium', 'hard'],
    weights=[0.5, 0.3, 0.2]
)[0]
```

### Generating Integer Solutions

```python
# Quadratic with integer roots
root1 = random.randint(-10, 10)
root2 = random.randint(-10, 10)

# Build from roots
expr = (x - root1) * (x - root2)
expanded = sp.expand(expr)

# Roots are guaranteed integers
answer_key = f"{root1}, {root2}"
```

### Simplification Problems

```python
# Create simplifiable expression
numerator = (x + 2) * (x - 3)
denominator = (x + 2) * (x + 5)

fraction = numerator / denominator

# Expanded form for question
num_expanded = sp.expand(numerator)
den_expanded = sp.expand(denominator)

question_latex = f"Simplify: $\\frac{{{sp.latex(num_expanded)}}}{{{sp.latex(den_expanded)}}}$"

# Simplified form for answer
simplified = sp.simplify(fraction)
answer_key = str(simplified)
```

## Debugging

### Common Issues

**Issue: Invalid JSON**
```python
# Problem: Unescaped quotes
question_latex = "What is \"x\"?"  # BREAKS JSON

# Solution: Use single quotes
question_latex = "What is 'x'?"

# Or escape
question_latex = "What is \\\"x\\\"?"
```

**Issue: LaTeX Not Rendering**
```python
# Problem: Forgot $ delimiters
question_latex = "Solve x^2 = 4"

# Solution: Add delimiters
question_latex = "Solve $x^2 = 4$"
```

**Issue: Answer Format Mismatch**
```python
# Problem: SymPy fraction not string
answer = sp.Rational(1, 2)
answer_key = answer  # TypeError in json.dumps()

# Solution: Convert to string
answer_key = str(answer)  # "1/2"
```

### Testing Locally

```bash
# Run script and check output
python my_generator.py

# Validate JSON
python my_generator.py | python -m json.tool

# Test multiple runs
for i in {1..10}; do python my_generator.py | jq '.difficulty'; done
```

## Advanced Topics

### Parametric Problems

```python
# Problem family with parameter
def generate_polynomial_problem(degree):
    x = sp.Symbol('x')

    # Generate polynomial of given degree
    coeffs = [random.randint(-10, 10) for _ in range(degree + 1)]
    expr = sum(c * x**i for i, c in enumerate(coeffs))

    # ...
```

### Multi-Step Problems

```python
# Problem with multiple parts
question_latex = (
    "Consider $f(x) = x^3 - 3x$. "
    "Find $f'(x)$ and $f''(x)$."
)

# Answer with multiple parts
first_derivative = sp.diff(expr, x)
second_derivative = sp.diff(first_derivative, x)

answer_key = f"{first_derivative}, {second_derivative}"
```

### Using External Data

```python
import requests

# Fetch data for word problems (if needed)
# Note: Requires network access, may slow generation
data = requests.get('https://api.example.com/data').json()
```

## Integration with Locus

### Factory API Endpoint

Problems are submitted to:
```
POST http://localhost:3000/api/internal/problems/bulk
Authorization: Bearer {FACTORY_API_KEY}

Body:
[
  {
    "question_latex": "...",
    "answer_key": "...",
    "difficulty": 1200,
    "main_topic": "...",
    "subtopic": "...",
    "grading_mode": "equivalent"
  },
  // ... more problems
]
```

### Validation on Backend

Backend validates:
- All required fields present
- `difficulty` in range 800-2400
- `main_topic` is valid enum
- `subtopic` is valid for topic
- `grading_mode` is "equivalent", "factor", or "expand"
- `question_latex` is non-empty
- `answer_key` is non-empty

Invalid problems are rejected with error details.

## Best Practices Summary

1. **Use SymPy** for all symbolic math
2. **Validate answers** programmatically
3. **Test multiple runs** to catch edge cases
4. **Avoid hardcoding** - use randomization
5. **Clear LaTeX** - test rendering in KaTeX
6. **Appropriate difficulty** - match problem complexity
7. **Correct grading mode** - factor/expand when needed
8. **Non-zero denominators** - avoid division by zero
9. **Integer solutions** when possible - easier to grade
10. **Comment your code** - explain complex logic

## Further Reading

- [SymPy Documentation](https://docs.sympy.org/)
- [KaTeX Supported Functions](https://katex.org/docs/supported.html)
- [LaTeX Math Symbols](https://www.overleaf.com/learn/latex/List_of_Greek_letters_and_math_symbols)
- [Factory README](README.md) - Setup and usage guide
