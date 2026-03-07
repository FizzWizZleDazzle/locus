"""LLM service for script generation"""

import httpx
from fastapi import HTTPException


# ELO guide shared between Python and Julia prompts
_ELO_GUIDE = """
ELO SCALE (100-5000):

100-400 Foundational: recognition/recall, single-operation (count objects, identify shapes)
400-700 Elementary: one clear step, basic definitions (1-step arithmetic, name a property)
700-1000 Pre-Competent: one-two steps, standard drills (2x+1=5, basic fraction ops)
1000-1400 Developing: textbook exercises, direct concept application (multi-step equations, standard integrals)
1400-1800 Competent: multi-step, concept relationships, strategic thinking (word problems, related rates)
1800-2200 Expert: deep understanding, creative approaches (challenging proofs, tricky optimization)
2200-3000 Competition: AMC/AIME level, multiple insights required
3000-4000 Olympiad: USAMO/IMO, novel proof strategies, deep combinatorics
4000-5000 Research-Adjacent: hardest competition problems, open-problem flavored

EXAMPLES:
- 200: "What is 3 + 4?"
- 600: "Solve x + 5 = 12"
- 900: "Solve 2x + 5 = 13"
- 1200: "Solve 3(x - 2) + 7 = 16"
- 1600: "Word problem requiring equation setup + multi-step solve"
- 2000: "Tricky optimization with constraints"
- 2500: "AMC 12 #20-level combinatorics"
- 3500: "USAMO proof problem"
"""

_DIFFICULTY_TARGETS = {
    "very_easy": "FOUNDATIONAL/ELEMENTARY problems (100-700 ELO range)",
    "easy": "PRE-COMPETENT to BEGINNER problems (700-1200 ELO range)",
    "medium": "DEVELOPING to COMPETENT problems (1200-1800 ELO range)",
    "hard": "EXPERT to COMPETITION ENTRY problems (1800-2500 ELO range)",
    "very_hard": "COMPETITION to OLYMPIAD problems (2500-3500 ELO range)",
    "competition": "OLYMPIAD to RESEARCH-ADJACENT problems (3500-5000 ELO range)",
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
include(joinpath(@__DIR__, "..", "julia", "src", "ProblemUtils.jl"))
using .ProblemUtils
```

SCRIPT STRUCTURE — use @script macro (declares variables, wraps body, calls run_batch):
```
@script x y begin
    set_topic!("main/sub")
    # body returns a problem() Dict
    problem(question=..., answer=..., difficulty=..., solution=...)
end
```

API SUMMARY:
- @script x y begin ... end — declares @variables, wraps body in generate(), calls run_batch
- set_topic!("main/sub") — set default topic for all problem() calls (call once at top of @script body)
- problem(; question, answer, difficulty, solution, topic, grading_mode, answer_type, calculator, image, time) -> Dict
  topic defaults to set_topic!() value. answer_type auto-detected. difficulty: Int or (lo,hi) tuple.
- steps(strings...) — join solution steps with <br>
- sol("Label", expr) -> "Label: $\LaTeX$", sol(expr) -> "$\LaTeX$" — for use inside steps()
- tex(expr) — convert Symbolics expression to LaTeX string

RANDOM EXPRESSION GENERATORS (return named tuples):
- rand_linear(x) -> (expr=ax+b, a, b). Keywords: a=(-9,9), b=(-9,9), nonzero_a=true
- rand_quadratic(x) -> (expr=ax²+bx+c, a, b, c). Keywords: a=(-5,5), b=(-9,9), c=(-9,9), nonzero_a=true
- rand_factorable(x) -> (expr=expanded a(x-r1)(x-r2), a, r1, r2). Keywords: a=(1,1), roots=(-9,9)
- rand_poly(x, n) -> (expr, coeffs::Vector). Keywords: coeff=(-9,9), nonzero_leading=true
  Access fields: q = rand_quadratic(x); q.expr, q.a, q.b, q.c

CAS: diff, expand, simplify, substitute(expr, x => val), solve (use ~ for equations)
Random: randint(lo,hi), nonzero(lo,hi), choice(collection)
Formatting: fmt_set, fmt_tuple, fmt_list, fmt_matrix, fmt_interval, fmt_equation, fmt_multipart
SVG: DiagramObj (geometry), GraphObj (plots), NumberLine (intervals)
  - Diagram: line!, arrow!, polygon!, circle!, arc!, point!, angle_arc!, right_angle!, segment_label!, tick_marks!, text!
  - Graph: plot!, fill_between!(g, expr1, expr2), point!, vline!, hline!
  - NumberLine: open_point!, closed_point!, shade!, shade_left!, shade_right!
  - All: svg = render(obj), pass to problem(..., image=svg)

KEY SYMBOLICS.JL DIFFERENCES FROM SYMPY:
- @variables x y z (not symbols), tex(expr) (not latex), // for rationals
- Equations use ~ (x^2 ~ 1), not Eq(). solve wraps Symbolics.solve_for
- No FiniteSet (use Set([])), no Matrix type (use Vector{Vector})

EXAMPLE 1 — derivative with rand_quadratic + sol:
```
include(joinpath(@__DIR__, "..", "julia", "src", "ProblemUtils.jl"))
using .ProblemUtils

@script x begin
    set_topic!("calculus/derivatives")
    q = rand_quadratic(x)
    df = diff(q.expr, x)
    problem(
        question="Find \\frac{d}{dx}[$(tex(q.expr))]",
        answer=df,
        difficulty=(1000, 1200),
        solution=steps(sol("Given", q.expr), "Apply power rule", sol("Answer", df)),
        time=60,
    )
end
```

EXAMPLE 2 — factoring with rand_factorable:
```
include(joinpath(@__DIR__, "..", "julia", "src", "ProblemUtils.jl"))
using .ProblemUtils

@script x begin
    set_topic!("algebra1/factoring")
    q = rand_factorable(x; roots=(-10,10))
    problem(
        question="Factor \$$(tex(q.expr))\$",
        answer=q.expr,
        difficulty=(1200, 1400),
        grading_mode="factor",
        solution=steps(
            sol("Expression", q.expr),
            "Find two numbers that multiply to $(q.r1 * q.r2) and add to $(-(q.r1 + q.r2))",
            "Roots: $(q.r1), $(q.r2)",
        ),
        time=120,
    )
end
```

EXAMPLE 3 — manual coefficients (no rand_ helper needed):
```
include(joinpath(@__DIR__, "..", "julia", "src", "ProblemUtils.jl"))
using .ProblemUtils

@script x begin
    set_topic!("algebra1/linear_equations")
    ans = randint(-20, 20)
    a = nonzero(-5, 5)
    b = randint(-10, 10)
    lhs = expand(a*(x - ans) + b)
    problem(
        question="Solve \$$(tex(lhs)) = $(b)\$",
        answer=ans,
        difficulty=(700, 1000),
        solution=steps(sol("Given", lhs ~ b), "Solve for x", sol("Answer", ans)),
        time=60,
    )
end
```

RULES:
1. REVERSE ENGINEER: Pick clean answers first, construct the problem backward
2. Randomize with LARGE ranges (coefficients ±20+, roots ±12, exponents up to 6-8)
3. Always include a solution using steps() with sol() helpers
4. ELO must match actual complexity (see ELO guide above)
5. Default calculator to "none" unless computation is heavy and not the focus
6. Always set time= in problem() (easy 30-90s, medium 60-180s, hard 120-300s)
7. Use rand_linear/rand_quadratic/rand_factorable/rand_poly when generating random polynomials
8. Always use @script macro — never write run_batch(generate) manually

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
