# Locus Factory - AI Problem Generator

An AI-powered mathematical problem generation system that creates problems using LLM-generated Python scripts.

## Architecture

- **Frontend**: Standalone HTML/JavaScript web UI
- **Backend**: Python FastAPI service
- **Integration**: Submits problems to Locus backend via Factory API

## Workflow

1. **Configure**: Set up LLM endpoint (OpenAI, Anthropic, etc.) and Locus backend
2. **Generate Script**: AI generates a Python script that creates random problems
3. **Test**: Run the script once to verify it works correctly
4. **Approve**: Review the generated problem
5. **Batch Generate**: Run the script 1000 times to create problem variations
6. **Submit**: All problems are automatically submitted to the Locus database

## Setup

### Prerequisites

- Python 3.8+
- Locus backend running on http://localhost:3000
- LLM API key (OpenAI, Anthropic Claude, or compatible)

### Installation

1. Install Python dependencies:
```bash
cd factory/backend
pip install -r requirements.txt
```

2. Start the Factory backend:
```bash
python main.py
```

The backend will start on `http://localhost:8001`

3. Open the frontend:
```bash
cd factory/frontend
# Open index.html in your browser, or use a simple HTTP server:
python -m http.server 8080
```

Then navigate to `http://localhost:8080`

## Configuration

### LLM Endpoints

The Factory supports any OpenAI-compatible API endpoint:

**OpenAI:**
- Endpoint: `https://api.openai.com/v1/chat/completions`
- Model: `gpt-4` or `gpt-3.5-turbo`

**Anthropic Claude (via OpenAI-compatible wrapper):**
- Endpoint: `https://api.anthropic.com/v1/messages`
- Model: `claude-3-sonnet-20240229`

**Local (Ollama):**
- Endpoint: `http://localhost:11434/v1/chat/completions`
- Model: `llama2` or any installed model

### Locus Backend

- Backend URL: `http://localhost:3000` (default)
- API Key: `development-factory-key-change-in-production` (from .env)

## Usage

1. **Configure LLM**:
   - Enter your LLM API endpoint and key
   - Select your model
   - Click "Save LLM Config"

2. **Configure Locus**:
   - Verify Locus backend URL (should already be correct)
   - Verify Factory API key matches `.env.example`
   - Click "Save Locus Config"

3. **Set Problem Parameters**:
   - Choose main topic (e.g., Calculus)
   - Enter subtopic (e.g., derivatives)
   - Set difficulty range (ELO: 1000-1500)
   - Select grading mode (equivalent or factor)

4. **Generate Script**:
   - Click "Generate Script with AI"
   - Wait for the AI to create a Python script
   - Review the generated script in the editor

5. **Test Script**:
   - Click "Test Script (1 problem)"
   - Verify the problem output looks correct
   - Check question_latex, answer_key, and difficulty

6. **Batch Generate**:
   - Click "Batch Generate (1000 problems)"
   - Wait for generation and submission (may take 2-5 minutes)
   - Review the success rate and any errors

## Example Script Output

A valid script should output JSON like this:

```json
{
  "question_latex": "Find the derivative: $\\frac{d}{dx}(3x^2 + 2x - 1)$",
  "answer_key": "6*x + 2",
  "difficulty": 1200,
  "main_topic": "calculus",
  "subtopic": "derivatives",
  "grading_mode": "equivalent"
}
```

## Script Requirements

Generated Python scripts must:

1. **Use SymPy** for symbolic mathematics
2. **Include randomization** so each run produces different problems
3. **Output valid JSON** to stdout
4. **Include all required fields**: question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode
5. **Be self-contained** (no external file dependencies)
6. **Execute quickly** (under 10 seconds per run)

## Example Script

```python
import sympy as sp
import random
import json

# Create a random derivative problem
x = sp.Symbol('x')

# Random coefficients
a = random.randint(1, 10)
b = random.randint(1, 10)
c = random.randint(-10, 10)

# Create expression
expr = a*x**2 + b*x + c

# Calculate derivative
derivative = sp.diff(expr, x)

# Generate problem
problem = {
    "question_latex": f"Find the derivative: $\\frac{{d}}{{dx}}({sp.latex(expr)})$",
    "answer_key": str(derivative),
    "difficulty": random.randint(1000, 1500),
    "main_topic": "calculus",
    "subtopic": "derivatives",
    "grading_mode": "equivalent"
}

print(json.dumps(problem))
```

## Troubleshooting

### "LLM not configured" error
Make sure you've clicked "Save LLM Config" after entering your API credentials.

### "Locus backend not configured" error
Make sure you've clicked "Save Locus Config" and the Locus backend is running.

### Script execution timeout
Scripts must complete in under 10 seconds. Simplify the script or reduce computation.

### Invalid JSON output
Make sure the script uses `print(json.dumps(problem))` and doesn't print anything else.

### Submission errors
Verify the Locus backend is running and the API key matches the one in `.env`.

## Security Notes

- **Never commit API keys** to version control
- The backend runs scripts using `subprocess` - only run trusted scripts
- In production, implement proper sandboxing for script execution
- Use environment variables for API keys instead of hardcoding

## Future Enhancements

- [ ] Script templates library
- [ ] SVG diagram generation support
- [ ] Problem validation with SymPy
- [ ] Progress tracking for batch generation
- [ ] Script sandboxing (Docker/containers)
- [ ] Problem quality metrics
- [ ] Database of successful scripts
