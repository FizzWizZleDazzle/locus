"""LLM service for script generation"""

import httpx
from fastapi import HTTPException


async def generate_script_with_llm(
    llm_config: dict,
    main_topic: str,
    subtopic: str,
    difficulty_level: str,
    grading_mode: str,
    prompt_template: str = None
) -> str:
    """Generate a Python script using LLM"""

    if not llm_config["endpoint"] or not llm_config["api_key"]:
        raise HTTPException(status_code=400, detail="LLM not configured")

    # ELO is RELATIVE TO TOPIC (not absolute mathematical difficulty)
    elo_guide = """
ELO SCALE (Relative to Topic):

1000-1200 (Beginner in this topic):
- Simplest problem type in this subtopic
- Single-step, direct application
- Minimal complexity

1200-1400 (Developing):
- Two-step problems
- Requires one intermediate calculation
- Standard textbook exercise level

1400-1600 (Competent):
- Multi-step problems
- Requires understanding of concept relationships
- Typical homework problem difficulty

1600-1800 (Advanced):
- Complex multi-step problems
- Requires strategic thinking
- Challenging homework or easy test problem

1800-2000 (Expert):
- Very complex problems in this topic
- Requires deep understanding
- Competition or advanced test level

EXAMPLES BY TOPIC:

Algebra1 Linear Equations:
- 1100: "2x = 10" (one-step)
- 1300: "2x + 5 = 13" (two-step)
- 1500: "3(x - 2) + 7 = 16" (distribution + multi-step)
- 1700: Word problem requiring equation setup

Calculus Derivatives:
- 1200: "d/dx[x³]" (power rule only)
- 1400: "d/dx[3x² + 2x - 1]" (polynomial)
- 1600: "d/dx[sin(2x)]" (chain rule)
- 1800: "d/dx[x·eˣ]" (product rule)

Geometry Triangles:
- 1100: "Find missing angle: given 60° and 80°"
- 1300: "Pythagorean theorem with sides 3,4"
- 1500: "Pythagorean with one unknown side"
- 1700: "Area using Heron's formula"
"""

    difficulty_targets = {
        "easy": "EASIER problems for this subtopic (1000-1300 ELO range)",
        "medium": "STANDARD problems for this subtopic (1300-1600 ELO range)",
        "hard": "HARDER problems for this subtopic (1600-1900 ELO range)"
    }

    default_prompt = f"""Generate a Python script that creates random mathematical problems.

{elo_guide}

Topic: {main_topic}
Subtopic: {subtopic}
Target: {difficulty_targets.get(difficulty_level, difficulty_targets['medium'])}
Grading: {grading_mode}

Script Requirements:
1. Use SymPy for symbolic math
2. REVERSE ENGINEER: Pick clean answers first, construct problem backward
3. Randomize parameters for variety
4. ASSIGN ELO based on actual problem complexity (use guide above)
5. Output ONLY valid JSON:

{{
    "question_latex": "...",  // LaTeX string
    "answer_key": "...",      // SymPy expression as string
    "difficulty": 1234,       // ELO rating matching complexity
    "main_topic": "{main_topic}",
    "subtopic": "{subtopic}",
    "grading_mode": "{grading_mode}"
}}

CRITICAL: Rate each generated problem accurately:
- Count the steps needed to solve
- Consider prerequisite knowledge required
- Match ELO to similar problems in the examples above

Output: Self-contained Python script (imports: sympy, random, json). Print ONLY JSON.
No markdown, no explanation, just the script code."""

    prompt = prompt_template or default_prompt

    try:
        async with httpx.AsyncClient(timeout=60.0) as client:
            # Detect API type from endpoint
            is_anthropic = "anthropic.com" in llm_config["endpoint"]

            if is_anthropic:
                # Anthropic API format
                response = await client.post(
                    llm_config["endpoint"],
                    headers={
                        "x-api-key": llm_config["api_key"],
                        "anthropic-version": "2023-06-01",
                        "Content-Type": "application/json",
                    },
                    json={
                        "model": llm_config["model"],
                        "messages": [{"role": "user", "content": prompt}],
                        "max_tokens": 2000,
                    },
                )
            else:
                # OpenAI API format
                response = await client.post(
                    llm_config["endpoint"],
                    headers={
                        "Authorization": f"Bearer {llm_config['api_key']}",
                        "Content-Type": "application/json",
                    },
                    json={
                        "model": llm_config["model"],
                        "messages": [{"role": "user", "content": prompt}],
                        "max_tokens": 2000,
                        "temperature": 0.7,
                    },
                )

            response.raise_for_status()
            data = response.json()

            # Extract script from response (handle both formats)
            if "choices" in data and len(data["choices"]) > 0:
                # OpenAI format
                script = data["choices"][0]["message"]["content"]
            elif "content" in data:
                # Anthropic format
                if isinstance(data["content"], list):
                    script = data["content"][0]["text"]
                else:
                    script = data["content"]
            else:
                script = str(data)

            # Clean up markdown code blocks
            if "```python" in script:
                script = script.split("```python")[1].split("```")[0].strip()
            elif "```" in script:
                script = script.split("```")[1].split("```")[0].strip()

            return script

    except Exception as e:
        raise HTTPException(status_code=500, detail=f"LLM API error: {str(e)}")
