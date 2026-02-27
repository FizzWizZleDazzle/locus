"""LLM service for script generation"""

import httpx
from fastapi import HTTPException


# ELO guide shared between Python and Julia prompts
_ELO_GUIDE = """
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

_DIFFICULTY_TARGETS = {
    "easy": "EASIER problems for this subtopic (1000-1300 ELO range)",
    "medium": "STANDARD problems for this subtopic (1300-1600 ELO range)",
    "hard": "HARDER problems for this subtopic (1600-1900 ELO range)"
}


def _python_prompt(main_topic: str, subtopic: str, difficulty_level: str) -> str:
    return f"""Generate a Python script that creates random math problems.

{_ELO_GUIDE}

Topic: {main_topic}
Subtopic: {subtopic}
Target: {_DIFFICULTY_TARGETS.get(difficulty_level, _DIFFICULTY_TARGETS['medium'])}

""" + r"""The script must start with `from problem_utils import *` which provides:

ALREADY IMPORTED: All standard SymPy functions (latex, solve, simplify, expand, factor,
diff, integrate, sqrt, sin, cos, tan, exp, log, Rational, Matrix, FiniteSet, Eq, etc.)
and random helpers (randint, choice, uniform).

PRE-DECLARED SYMBOLS: x, y, z, t, n, m, k, a, b, c, d

HELPERS:
- problem(question, answer, difficulty, topic, solution, *, grading_mode, answer_type, calculator) -> dict
  - question: LaTeX string
  - answer: any Python/SymPy value (auto-converted to string)
  - difficulty: int or (lo, hi) tuple for random ELO
  - topic: "main/sub" e.g. "calculus/derivatives"
  - solution: step-by-step LaTeX (use steps() helper)
  - grading_mode: "equivalent" (default), "factor", "expand"
  - answer_type: auto-detected if omitted (bool->boolean, int/Number->numeric,
    Matrix->matrix, FiniteSet->set, tuple->tuple, list->list, Eq->equation, else->expression)
  - calculator: "none" (default), "scientific", "graphing", "cas"
- emit(dict) — prints JSON to stdout
- steps(*strings) — joins with <br> for solution_latex
- SVG diagrams: `from svg_utils import Diagram, Graph` (optional)
  Pass rendered SVG to problem(..., image=d.render())

  Diagram — geometry diagrams (math coords, y-up, auto-scaled):
    d = Diagram(width=300, height=250, padding=40)
    d.line(p1, p2, dashed=False)                     # line segment
    d.polygon(points, labels=None, fill=None)         # closed shape; labels list matches points
    d.circle(center, radius, fill=None)               # fill e.g. "currentColor"
    d.arc(center, radius, start_deg, end_deg)         # circular arc
    d.point(x, y, label=None)                         # dot with optional label
    d.angle_arc(vertex, p1, p2, label=None)           # arc marking angle at vertex
    d.right_angle(vertex, p1, p2)                     # right-angle square marker
    d.segment_label(p1, p2, text)                     # label at midpoint of segment
    d.tick_marks(p1, p2, count=1)                     # equal-length tick marks on segment
    d.text(x, y, text)                                # free text label
    svg = d.render()

  Graph — function plots on a coordinate grid:
    g = Graph(x_range=(-5, 5), y_range=(-5, 5), width=300, height=300)
    g.plot(sympy_expr, color=None, dashed=False)      # plot SymPy expr in x
    g.point(x, y, label=None)                         # labeled point
    g.vline(x, dashed=True)                           # vertical line (e.g. asymptote)
    g.hline(y, dashed=True)                           # horizontal line
    svg = g.render()
- nonzero(lo, hi) — random int in [lo,hi] excluding 0
- fmt_set, fmt_tuple, fmt_list, fmt_matrix — format answer strings for special types
- fmt_interval(left, right, left_open, right_open) — "open:1,closed:7" format
- fmt_equation(lhs, rhs) — "lhs = rhs"

EXAMPLE 1 (expression answer):
```
from problem_utils import *

def generate():
    n = randint(2, 7)
    coeff = nonzero(-12, 12)
    expr = coeff * x**n
    ans = diff(expr, x)
    return problem(
        question=f"\\frac{{d}}{{dx}}\\left[{latex(expr)}\\right]",
        answer=ans,
        difficulty=(1000, 1200),
        topic="calculus/derivatives",
        solution=steps(
            f"Apply power rule to ${latex(expr)}$",
            f"${latex(ans)}$",
        ),
    )

emit(generate())
```

EXAMPLE 2 (numeric answer):
```
from problem_utils import *

def generate():
    a_val, b_val = nonzero(-50, 50), nonzero(-50, 50)
    ans = a_val + b_val
    return problem(
        question=f"${{{a_val}}} + {{{b_val}}} = ?$",
        answer=ans,
        difficulty=(1000, 1100),
        topic="arithmetic/addition_subtraction",
        solution=f"${{{a_val}}} + {{{b_val}}} = {{{ans}}}$",
    )

emit(generate())
```

EXAMPLE 3 (factored form):
```
from problem_utils import *

def generate():
    # Reverse-engineer: pick roots first, expand to get messy-but-factorable polynomial
    a = choice([1, 2, 3, 4])          # leading factor for first root
    b = choice([1, 2, 3, 4])          # leading factor for second root
    r1, r2 = nonzero(-10, 10), nonzero(-10, 10)
    expr = expand((a*x - r1) * (b*x - r2))
    ans = factor(expr)
    return problem(
        question=f"Factor ${latex(expr)}$",
        answer=ans,
        difficulty=(1200, 1400),
        topic="algebra1/factoring",
        grading_mode="factor",
        solution=steps(
            f"Use the AC method or factor by grouping",
            f"${latex(ans)}$",
        ),
    )

emit(generate())
```

RULES:
1. REVERSE ENGINEER: Pick clean answers first, construct the problem backward
2. Randomize parameters for variety — use LARGE ranges. Numbers don't need to be small.
   - Coefficients: randint(-20, 20) or wider, not just (-3, 3)
   - Factorable polynomials: pick roots first (e.g. r1=randint(-12,12), r2=randint(-12,12)),
     then expand — 12x²+31x+20 is perfectly valid since it factors to (4x+5)(3x+4)
   - Exponents: up to 6 or 8 for medium/hard
   - Bounds and constants: scale with difficulty — hard problems should have harder numbers
3. Always include a solution using steps()
4. ELO must match actual complexity (see ELO guide above)
5. Default calculator to "none" unless computation is heavy and not the focus
6. Use Diagram/Graph when a visual would help (geometry, graphing, coordinate problems)
7. Always set time= in problem() to the expected solve time in seconds.
   Guidelines: easy 30-90s, medium 60-180s, hard 120-300s. Scale by problem complexity.

Output ONLY the Python script. No markdown fences, no explanation."""


def _julia_prompt(main_topic: str, subtopic: str, difficulty_level: str) -> str:
    return f"""Generate a Julia script that creates random math problems.

{_ELO_GUIDE}

Topic: {main_topic}
Subtopic: {subtopic}
Target: {_DIFFICULTY_TARGETS.get(difficulty_level, _DIFFICULTY_TARGETS['medium'])}

""" + r"""The script must start with:
```
include(joinpath(@__DIR__, "..", "..", "..", "julia", "src", "ProblemUtils.jl"))
using .ProblemUtils
```
which provides SymEngine.jl CAS and these helpers:

AVAILABLE VIA ProblemUtils: All SymEngine functions (@vars, diff, expand, factor, subs,
sin, cos, tan, exp, log, sqrt, abs, Basic, free_symbols),
Latexify (via tex() helper), JSON, and Random.

HELPERS:
- problem(; question, answer, difficulty, topic, solution, grading_mode, answer_type, calculator, image, time) -> Dict
  - question: LaTeX string
  - answer: any Julia/SymEngine value (auto-converted to string)
  - difficulty: Int or (lo, hi) tuple for random ELO
  - topic: "main/sub" e.g. "calculus/derivatives"
  - solution: step-by-step LaTeX (use steps() helper)
  - grading_mode: "equivalent" (default), "factor", "expand"
  - answer_type: auto-detected if omitted (Bool->boolean, Int/Float->numeric,
    Vector{Vector}->matrix, Set->set, Tuple->tuple, Vector->list, else->expression)
  - calculator: "none" (default), "scientific", "graphing", "cas"
- emit(dict) — prints JSON line to stdout (JSONL)
- run_batch(generate) — parses --count N from ARGS, calls generate() N times with emit()
- steps(strings...) — joins with <br> for solution_latex
- tex(expr) — converts SymEngine expression to LaTeX string via Latexify
- nonzero(lo, hi) — random int in [lo,hi] excluding 0
- randint(lo, hi) — random int in [lo,hi] (inclusive)
- choice(collection) — pick random element
- fmt_set, fmt_tuple, fmt_list, fmt_matrix — format answer strings for special types
- fmt_interval(left, right; left_open, right_open) — "open:1,closed:7" format
- fmt_equation(lhs, rhs) — "lhs = rhs"
- compress_svg(svg), decompress_svg(s) — SVG compression

KEY SYMENGINE.JL DIFFERENCES FROM SYMPY:
- Declare symbols: @vars x y z  (not symbols('x y z'))
- LaTeX: tex(expr)  (not latex(expr))
- Rational: Basic(a) // Basic(b)  (not Rational(a, b))
- SymEngine has limited simplify; use expand() instead
- solve(expr, x) returns a vector
- No Matrix type; use Vector{Vector} for matrices
- No FiniteSet; use Set([...]) for sets

EXAMPLE 1 (expression answer):
```
include(joinpath(@__DIR__, "..", "..", "..", "julia", "src", "ProblemUtils.jl"))
using .ProblemUtils

function generate()
    @vars x
    n = randint(2, 7)
    coeff = nonzero(-12, 12)
    expr = coeff * x^n
    ans = diff(expr, x)
    return problem(
        question="\\frac{d}{dx}\\left[$(tex(expr))\\right]",
        answer=ans,
        difficulty=(1000, 1200),
        topic="calculus/derivatives",
        solution=steps(
            "Apply power rule to \$$(tex(expr))\$",
            "\$$(tex(ans))\$",
        ),
    )
end

run_batch(generate)
```

EXAMPLE 2 (numeric answer):
```
include(joinpath(@__DIR__, "..", "..", "..", "julia", "src", "ProblemUtils.jl"))
using .ProblemUtils

function generate()
    a_val, b_val = nonzero(-50, 50), nonzero(-50, 50)
    ans = a_val + b_val
    return problem(
        question="\$$(a_val) + $(b_val) = ?\$",
        answer=ans,
        difficulty=(1000, 1100),
        topic="arithmetic/addition_subtraction",
        solution="\$$(a_val) + $(b_val) = $(ans)\$",
    )
end

run_batch(generate)
```

EXAMPLE 3 (factored form):
```
include(joinpath(@__DIR__, "..", "..", "..", "julia", "src", "ProblemUtils.jl"))
using .ProblemUtils

function generate()
    @vars x
    a = choice([1, 2, 3, 4])
    b = choice([1, 2, 3, 4])
    r1, r2 = nonzero(-10, 10), nonzero(-10, 10)
    expr = expand((a*x - r1) * (b*x - r2))
    ans = factor(expr)
    return problem(
        question="Factor \$$(tex(expr))\$",
        answer=ans,
        difficulty=(1200, 1400),
        topic="algebra1/factoring",
        grading_mode="factor",
        solution=steps(
            "Use the AC method or factor by grouping",
            "\$$(tex(ans))\$",
        ),
    )
end

run_batch(generate)
```

RULES:
1. REVERSE ENGINEER: Pick clean answers first, construct the problem backward
2. Randomize parameters for variety — use LARGE ranges. Numbers don't need to be small.
   - Coefficients: randint(-20, 20) or wider, not just (-3, 3)
   - Factorable polynomials: pick roots first (e.g. r1=randint(-12,12), r2=randint(-12,12)),
     then expand — 12x²+31x+20 is perfectly valid since it factors to (4x+5)(3x+4)
   - Exponents: up to 6 or 8 for medium/hard
   - Bounds and constants: scale with difficulty — hard problems should have harder numbers
3. Always include a solution using steps()
4. ELO must match actual complexity (see ELO guide above)
5. Default calculator to "none" unless computation is heavy and not the focus
6. Always set time= in problem() to the expected solve time in seconds.
   Guidelines: easy 30-90s, medium 60-180s, hard 120-300s. Scale by problem complexity.
7. Always end the script with run_batch(generate) — this handles --count N for batch execution

Output ONLY the Julia script. No markdown fences, no explanation."""


async def generate_script_with_llm(
    llm_config: dict,
    main_topic: str,
    subtopic: str,
    difficulty_level: str,
    prompt_template: str = None,
    language: str = "julia",
) -> str:
    """Generate a problem script using LLM"""

    if not llm_config["endpoint"] or not llm_config["api_key"]:
        raise HTTPException(status_code=400, detail="LLM not configured")

    if prompt_template:
        prompt = prompt_template
    elif language == "julia":
        prompt = _julia_prompt(main_topic, subtopic, difficulty_level)
    else:
        prompt = _python_prompt(main_topic, subtopic, difficulty_level)

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
                        "max_tokens": 4096,
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
                        "max_tokens": 4096,
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
            fence_lang = "julia" if language == "julia" else "python"
            if f"```{fence_lang}" in script:
                script = script.split(f"```{fence_lang}")[1].split("```")[0].strip()
            elif "```" in script:
                script = script.split("```")[1].split("```")[0].strip()

            return script

    except Exception as e:
        raise HTTPException(status_code=500, detail=f"LLM API error: {str(e)}")
