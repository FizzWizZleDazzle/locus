# Locus Factory - Quick Start Guide

## Setup

1. **Configure environment** (optional - can also use UI):
   ```bash
   cd factory/backend
   cp .env.example .env
   # Edit .env with your LLM API key
   ```

2. **Start the backend**:
   ```bash
   cd factory/backend
   source venv/bin/activate
   python main.py
   ```
   Backend runs on `http://localhost:8001`

3. **Open the frontend**:
   ```bash
   cd factory/frontend
   # Open index.html in browser, or:
   python -m http.server 8080
   ```
   Then go to `http://localhost:8080`

## Workflow

### 1. Configure (first time only)
- Go to **Config** tab
- Enter LLM endpoint, API key, and model
- Click "Save LLM Config"

### 2. Generate a Script
- Go to **Generate Script** tab
- Enter topic (e.g., `calculus`) and subtopic (e.g., `derivatives`)
- Set difficulty range (e.g., 1000-1500)
- Choose grading mode
- Click **Generate Script with AI**
- Wait for LLM to create the Python script

### 3. Test & Save
- Click **Test Script** to run it once and verify output
- If it works, click **Save Script**
- Enter a name (e.g., `calculus_derivatives`)
- Add optional description

### 4. Run Script to Generate Problems
- In **Script Library** (left sidebar), find your saved script
- Click the **RUN** button
- Enter how many problems to generate (e.g., 10)
- Problems appear in **Review Problems** tab

### 5. Review & Approve
- Go to **Review Problems** tab
- See each problem with LaTeX rendering
- Click **Approve** on good problems (adds to staging)
- Click **Reject** to discard
- Or click **Approve All**

### 6. Export to SQL
- Check **Staging Area** (right sidebar) - shows count
- Click **SQL** button to export
- Downloads `.sql` file ready for PostgreSQL
- File has INSERT statements for the `problems` table

## Tips

- **One script, many problems**: A good script generates random variations each time it runs
- **Reverse engineering**: Best scripts pick clean answers first, then build the problem backward
- **SymPy validation**: Scripts should use SymPy to ensure answer_key is valid
- **Word problems**: Include narrative context with random names, scenarios, values

## Example Script Structure

```python
import sympy as sp
import random
import json

x = sp.Symbol('x')

# Reverse engineer: pick a clean answer
solution = random.randint(-10, 10)

# Build problem from answer
a = random.randint(2, 8)
b = random.randint(-15, 15)
c = a * solution + b

# Format
question_latex = f"Solve for $x$: ${a}x + {b} = {c}$"
answer_key = str(solution)

problem = {
    "question_latex": question_latex,
    "answer_key": answer_key,
    "difficulty": random.randint(1000, 1300),
    "main_topic": "algebra1",
    "subtopic": "linear_equations",
    "grading_mode": "equivalent"
}

print(json.dumps(problem))
```

## Endpoints

- `GET /scripts` - List saved scripts
- `POST /scripts/save` - Save a script
- `GET /scripts/{name}` - Load a script
- `POST /generate-script` - Generate script with LLM
- `POST /test-script` - Test a script (run once)
- `POST /run-script` - Run script multiple times
- `POST /confirm-problem` - Approve/reject a problem
- `GET /staged` - Get staged problems
- `POST /export` - Export to SQL or JSON
