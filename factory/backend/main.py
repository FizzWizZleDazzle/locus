"""
Locus Factory - AI Problem Generation Service

This service generates mathematical problems using AI-generated Python scripts.
The workflow:
1. AI generates a Python script that creates random problems
2. Test the script (run once to verify it works)
3. Batch generate (run script 1000 times)
4. Submit all problems to Locus backend
"""

from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Optional
import httpx
import json
import subprocess
import tempfile
import os
from pathlib import Path

app = FastAPI(title="Locus Factory")

# CORS for standalone frontend
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # In production, restrict this
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Configuration storage (in production, use a database)
llm_config = {
    "endpoint": None,
    "api_key": None,
    "model": None,
}

locus_config = {
    "backend_url": "http://localhost:3000",
    "api_key": "development-factory-key-change-in-production",
}


class LLMConfig(BaseModel):
    endpoint: str
    api_key: str
    model: str


class LocusConfig(BaseModel):
    backend_url: str
    api_key: str


class GenerateScriptRequest(BaseModel):
    main_topic: str
    subtopic: str
    difficulty_min: int
    difficulty_max: int
    grading_mode: str = "equivalent"
    prompt_template: Optional[str] = None


class TestScriptRequest(BaseModel):
    script: str


class BatchGenerateRequest(BaseModel):
    script: str
    count: int = 1000


@app.get("/")
async def root():
    return {"message": "Locus Factory API", "status": "running"}


@app.post("/config/llm")
async def configure_llm(config: LLMConfig):
    """Configure LLM endpoint and credentials"""
    llm_config["endpoint"] = config.endpoint
    llm_config["api_key"] = config.api_key
    llm_config["model"] = config.model
    return {"message": "LLM configuration updated"}


@app.post("/config/locus")
async def configure_locus(config: LocusConfig):
    """Configure Locus backend connection"""
    locus_config["backend_url"] = config.backend_url
    locus_config["api_key"] = config.api_key
    return {"message": "Locus configuration updated"}


@app.get("/config")
async def get_config():
    """Get current configuration (masked)"""
    return {
        "llm": {
            "endpoint": llm_config["endpoint"],
            "api_key": "***" if llm_config["api_key"] else None,
            "model": llm_config["model"],
        },
        "locus": {
            "backend_url": locus_config["backend_url"],
            "api_key": "***" if locus_config["api_key"] else None,
        },
    }


@app.post("/generate-script")
async def generate_script(request: GenerateScriptRequest):
    """Generate a Python script using AI"""
    if not llm_config["endpoint"] or not llm_config["api_key"]:
        raise HTTPException(status_code=400, detail="LLM not configured")

    # Build the prompt for the AI
    default_prompt = f"""Generate a Python script that creates a random mathematical problem.

Requirements:
- Topic: {request.main_topic}
- Subtopic: {request.subtopic}
- Difficulty range: {request.difficulty_min}-{request.difficulty_max} (ELO rating)
- Grading mode: {request.grading_mode}

The script should:
1. Use SymPy for symbolic math
2. Generate random parameters to create problem variations
3. Output JSON with these exact fields:
   - question_latex: LaTeX string for the question
   - answer_key: SymPy expression as string (e.g., "2*x + 3")
   - difficulty: integer between {request.difficulty_min} and {request.difficulty_max}
   - main_topic: "{request.main_topic}"
   - subtopic: "{request.subtopic}"
   - grading_mode: "{request.grading_mode}"

Example output format:
{{
    "question_latex": "Solve for $x$: $2x + 3 = 7$",
    "answer_key": "2",
    "difficulty": 1200,
    "main_topic": "{request.main_topic}",
    "subtopic": "{request.subtopic}",
    "grading_mode": "{request.grading_mode}"
}}

The script should be self-contained and print ONLY valid JSON to stdout.
Include proper randomization so each run generates a different problem.

Generate ONLY the Python script, no explanation."""

    prompt = request.prompt_template or default_prompt

    # Call LLM API (supports OpenAI-compatible format)
    try:
        async with httpx.AsyncClient(timeout=60.0) as client:
            # Try OpenAI-compatible format
            response = await client.post(
                llm_config["endpoint"],
                headers={
                    "Authorization": f"Bearer {llm_config['api_key']}",
                    "Content-Type": "application/json",
                },
                json={
                    "model": llm_config["model"],
                    "messages": [
                        {"role": "user", "content": prompt}
                    ],
                    "max_tokens": 2000,
                    "temperature": 0.7,
                },
            )
            response.raise_for_status()
            data = response.json()

            # Extract script from response
            if "choices" in data and len(data["choices"]) > 0:
                script = data["choices"][0]["message"]["content"]
            else:
                script = data.get("content", str(data))

            # Clean up script (remove markdown code blocks if present)
            if "```python" in script:
                script = script.split("```python")[1].split("```")[0].strip()
            elif "```" in script:
                script = script.split("```")[1].split("```")[0].strip()

            return {
                "script": script,
                "message": "Script generated successfully"
            }

    except Exception as e:
        raise HTTPException(status_code=500, detail=f"LLM API error: {str(e)}")


@app.post("/test-script")
async def test_script(request: TestScriptRequest):
    """Test a script by running it once"""
    try:
        # Create temporary file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
            f.write(request.script)
            script_path = f.name

        try:
            # Run the script in a subprocess with timeout
            result = subprocess.run(
                ['python3', script_path],
                capture_output=True,
                text=True,
                timeout=10,
                cwd=tempfile.gettempdir(),
            )

            if result.returncode != 0:
                return {
                    "success": False,
                    "error": result.stderr,
                    "stdout": result.stdout,
                }

            # Parse the JSON output
            try:
                problem = json.loads(result.stdout.strip())

                # Validate required fields
                required_fields = ["question_latex", "answer_key", "difficulty",
                                 "main_topic", "subtopic", "grading_mode"]
                missing_fields = [f for f in required_fields if f not in problem]

                if missing_fields:
                    return {
                        "success": False,
                        "error": f"Missing required fields: {', '.join(missing_fields)}",
                        "output": result.stdout,
                    }

                return {
                    "success": True,
                    "problem": problem,
                    "message": "Script executed successfully"
                }

            except json.JSONDecodeError as e:
                return {
                    "success": False,
                    "error": f"Invalid JSON output: {str(e)}",
                    "output": result.stdout,
                }

        finally:
            # Clean up temp file
            os.unlink(script_path)

    except subprocess.TimeoutExpired:
        return {
            "success": False,
            "error": "Script execution timed out (10 seconds)",
        }
    except Exception as e:
        return {
            "success": False,
            "error": f"Execution error: {str(e)}",
        }


@app.post("/batch-generate")
async def batch_generate(request: BatchGenerateRequest):
    """Run script multiple times and submit all problems to Locus backend"""
    if not locus_config["backend_url"] or not locus_config["api_key"]:
        raise HTTPException(status_code=400, detail="Locus backend not configured")

    problems = []
    errors = []

    # Create temporary file for script
    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
        f.write(request.script)
        script_path = f.name

    try:
        # Generate problems
        for i in range(request.count):
            try:
                result = subprocess.run(
                    ['python3', script_path],
                    capture_output=True,
                    text=True,
                    timeout=10,
                    cwd=tempfile.gettempdir(),
                )

                if result.returncode == 0:
                    try:
                        problem = json.loads(result.stdout.strip())
                        problems.append(problem)
                    except json.JSONDecodeError:
                        errors.append(f"Run {i+1}: Invalid JSON output")
                else:
                    errors.append(f"Run {i+1}: {result.stderr[:100]}")

            except subprocess.TimeoutExpired:
                errors.append(f"Run {i+1}: Timeout")
            except Exception as e:
                errors.append(f"Run {i+1}: {str(e)[:100]}")

            # Progress feedback every 100 problems
            if (i + 1) % 100 == 0:
                print(f"Generated {i+1}/{request.count} problems")

    finally:
        os.unlink(script_path)

    # Submit problems to Locus backend
    submitted = 0
    submission_errors = []

    async with httpx.AsyncClient(timeout=30.0) as client:
        for i, problem in enumerate(problems):
            try:
                response = await client.post(
                    f"{locus_config['backend_url']}/api/internal/problems",
                    headers={
                        "X-API-Key": locus_config["api_key"],
                        "Content-Type": "application/json",
                    },
                    json=problem,
                )
                response.raise_for_status()
                submitted += 1
            except Exception as e:
                submission_errors.append(f"Problem {i+1}: {str(e)[:100]}")

    return {
        "generated": len(problems),
        "submitted": submitted,
        "generation_errors": errors[:10],  # First 10 errors
        "submission_errors": submission_errors[:10],
        "message": f"Successfully submitted {submitted}/{len(problems)} problems"
    }


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8001)
