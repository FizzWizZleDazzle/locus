# 🏭 Locus Factory - Quick Start Guide

Get up and running with the AI problem generator in 5 minutes!

## Prerequisites

- Python 3.8+ installed
- Locus backend running (`cargo run --bin locus-backend`)
- An LLM API key (OpenAI, Anthropic, etc.)

## Installation

```bash
# 1. Navigate to factory directory
cd factory/backend

# 2. Create virtual environment (if needed)
python3 -m venv venv
source venv/bin/activate

# 3. Install dependencies
pip install -r requirements.txt

# 4. Start the backend
python main.py
```

The backend will start on **http://localhost:8001**

## Open the UI

In another terminal:

```bash
cd factory/frontend
python3 -m http.server 8080
```

Open your browser to **http://localhost:8080**

## First-Time Setup

1. **Configure LLM**:
   - Endpoint: `https://api.openai.com/v1/chat/completions`
   - API Key: Your OpenAI API key (`sk-...`)
   - Model: `gpt-4` or `gpt-3.5-turbo`
   - Click **Save LLM Config**

2. **Configure Locus**:
   - Backend URL: `http://localhost:3000` (default)
   - API Key: `development-factory-key-change-in-production`
   - Click **Save Locus Config**

## Generate Your First Problem

1. **Set Parameters**:
   - Main Topic: `arithmetic`
   - Subtopic: `addition_subtraction`
   - Min Difficulty: `1000`
   - Max Difficulty: `1500`
   - Grading Mode: `equivalent`

2. **Generate Script**:
   - Click **🤖 Generate Script with AI**
   - Wait 10-30 seconds for AI to create a Python script
   - Review the generated script

3. **Test Script**:
   - Click **🧪 Test Script (1 problem)**
   - Verify the output looks correct
   - Check the problem preview

4. **Batch Generate**:
   - Click **🚀 Batch Generate (1000 problems)**
   - Confirm the dialog
   - Wait 2-5 minutes for generation
   - Review success rate

## Alternative: Use Example Scripts

Instead of generating with AI, you can test with pre-made examples:

```bash
cd factory/examples

# Test arithmetic example
python3 arithmetic_addition.py

# Test calculus example (requires sympy)
python3 calculus_derivatives.py
```

Load an example script into the UI:
1. Copy the contents of `examples/arithmetic_addition.py`
2. Paste into the Script Editor in the UI
3. Click **Test Script**
4. Click **Batch Generate**

## What's Next?

- Generate problems for different topics (algebra1, calculus, geometry)
- Modify example scripts to create variations
- Check the Locus database to see your submitted problems
- Use the Locus frontend to solve the generated problems

## Troubleshooting

**Backend won't start:**
- Make sure port 8001 is not in use
- Check you activated the virtual environment
- Verify all dependencies installed

**LLM not configured error:**
- Make sure you clicked "Save LLM Config"
- Verify your API key is correct

**Submission errors:**
- Make sure Locus backend is running on port 3000
- Verify the Factory API key matches `.env.example`
- Check Locus backend logs for errors

**Script execution timeout:**
- Simplify the script
- Remove expensive computations
- Ensure imports are at the top

## System Requirements

- **Memory**: 512MB+ for backend
- **Disk**: 100MB for dependencies
- **Network**: Internet connection for LLM API calls
- **Time**: 2-5 minutes for 1000 problems

## Tips

- Start with simple problems (arithmetic) before complex ones (calculus)
- Test scripts thoroughly before batch generation
- Monitor the Locus database size if generating many thousands of problems
- Use lower difficulty ranges for testing (800-1200)
- Review AI-generated scripts for correctness before running

## Example Workflow

```bash
# Terminal 1: Locus Backend
cd Locus
cargo run --bin locus-backend

# Terminal 2: Factory Backend
cd Locus/factory/backend
source venv/bin/activate
python main.py

# Terminal 3: Factory Frontend
cd Locus/factory/frontend
python3 -m http.server 8080

# Browser: http://localhost:8080
# Configure → Generate → Test → Batch Generate → Done!
```

Enjoy generating problems! 🎉
