# Factory

LLM-powered problem generation pipeline. Generates Julia or Python scripts that produce math problems, validates them, and inserts directly into the Locus PostgreSQL database.

## Architecture

```
factory/
  backend/                    FastAPI app (port 9090)
    main.py                   Entry point, route registration
    config.py                 LLM + database configuration
    models.py                 Pydantic request/response models
    routes/
      config_routes.py        LLM and backend config endpoints
      script_routes.py        Script generation, saving, testing, execution
      problem_routes.py       Problem staging, approval, export
    services/                 LLM client, script runner, DB access, validator
    scripts/                  Git submodule → locus-scripts repo
      src/                    324+ generated problem scripts (Python + Julia)
      python/                 Python utilities (problem_utils.py, svg_utils.py)
      julia/                  Julia project (Symbolics.jl CAS, SvgUtils.jl)
        Project.toml          Runtime deps (Symbolics, Latexify, JSON, Random)
        src/ProblemUtils.jl   Shared utilities + SvgUtils for Julia scripts
        build/                PackageCompiler sysimage build
      run.sh                  Standalone runner for scripts repo
  frontend/                   TypeScript UI (port 9091)
    index.html                Entry point
    factory.ts                Main TypeScript source
    factory.css               Styles
  exports/                    Generated SQL/JSON/SQLite outputs
  automate_pipeline.py        Full automation script
  import_db.py                SQLite -> PostgreSQL importer
  publish_db.py               SQL -> SQLite -> GitHub Release publisher
  topup.py                    Re-run scripts for thin subtopics
  start.sh                    Multi-process startup
```

## Setup

```bash
cd factory

# Copy and configure environment
cp backend/.env.example backend/.env
# Edit backend/.env with LLM credentials and backend URL

# Start everything
./start.sh
```

`start.sh` creates a Python venv, installs dependencies, compiles TypeScript, sets up Julia (installs deps + downloads or builds sysimage), starts the FastAPI backend (uvicorn with hot reload) and an HTTP server for the frontend.

### Prerequisites

- **Python 3** — required for backend and Python scripts
- **Julia** — optional but recommended for Julia scripts (faster generation)
  - Install from https://julialang.org/downloads/
  - On first run, `start.sh` tries to download a pre-built sysimage from GitHub Releases, falling back to a local build (~2 min)

### Environment Variables

| Variable | Description |
|---|---|
| `LLM_ENDPOINT` | `https://api.openai.com/v1/chat/completions` or `https://api.anthropic.com/v1/messages` |
| `LLM_API_KEY` | API key for the LLM provider |
| `LLM_MODEL` | Model name (e.g. `gpt-4`, `claude-sonnet-4-5-20250929`) |
| `DATABASE_URL` | Postgres connection string (same DB as main backend) |
| `SCRIPTS_REPO_PATH` | Override scripts repo location (default: `backend/scripts`) |

## Dual-Language Support

Scripts can be written in **Julia** (default for new scripts) or **Python**. Both produce identical JSON output.

### Why Julia?

1. **Symbolics.jl** — full-featured native Julia CAS (simplify, solve, factor)
2. **Batch execution** — one Julia process generates N problems (amortizes JIT startup)
3. **Math-native syntax** — LLMs generate cleaner math code in Julia

### Execution Model

| Language | Execution | Per 100 problems |
|---|---|---|
| Python | `python3 script.py` x 100 (one process per problem) | 100 subprocess spawns |
| Julia | `julia script.jl --count 100` (one process, JSONL output) | 1 subprocess spawn |

The Julia sysimage (built or downloaded by `start.sh`) reduces startup from ~3s to ~0.3s.

### Julia Script API (ProblemUtils.jl)

Scripts begin with:
```julia
include(joinpath(@__DIR__, "..", "julia", "src", "ProblemUtils.jl"))
using .ProblemUtils
```

**Available:**
- `@variables x y z` — declare symbolic variables
- `diff(expr, x)`, `expand(expr)`, `simplify(expr)`, `substitute(expr, x => val)`, `solve(eq, x)` — CAS
- `tex(expr)` — LaTeX output via Latexify.jl
- `problem(; question, answer, difficulty, topic, solution, ...)` — build problem dict
- `emit(dict)` — print one JSON line to stdout
- `run_batch(generate)` — parse `--count N`, call `generate()` N times
- `steps(s1, s2, ...)`, `nonzero(lo, hi)`, `randint(lo, hi)`, `choice(v)`
- `fmt_set`, `fmt_tuple`, `fmt_list`, `fmt_matrix`, `fmt_interval`, `fmt_equation`
- `compress_svg(svg)`, `decompress_svg(s)`
- `DiagramObj`, `GraphObj`, `NumberLine` — SVG builders
- `arrow!`, `fill_between!`, `open_point!`, `closed_point!`, `shade!`, `shade_left!`, `shade_right!`

### Simplified API

The `@script` macro, random expression generators, and `step()` helper reduce boilerplate significantly:

**Before:**
```julia
push!(LOAD_PATH, joinpath(@__DIR__, "src"))
include(joinpath(@__DIR__, "src", "ProblemUtils.jl"))
using .ProblemUtils

@variables x

function generate()
    a = nonzero(-5, 5)
    b = randint(-9, 9)
    c = randint(-9, 9)
    f = a*x^2 + b*x + c
    df = diff(f, x)

    return problem(
        question="Find \\frac{d}{dx}[$(tex(f))]",
        answer=df,
        difficulty=(1200, 1400),
        topic="calculus/derivatives",
        solution=steps(
            "Given: \$$(tex(f))\$",
            "Apply power rule",
            "Answer: \$$(tex(df))\$"
        )
    )
end

run_batch(generate)
```

**After:**
```julia
push!(LOAD_PATH, joinpath(@__DIR__, "src"))
include(joinpath(@__DIR__, "src", "ProblemUtils.jl"))
using .ProblemUtils

@script x begin
    set_topic!("calculus/derivatives")
    q = rand_quadratic(x)
    df = diff(q.expr, x)

    problem(
        question="Find \\frac{d}{dx}[$(tex(q.expr))]",
        answer=df,
        difficulty=(1200, 1400),
        solution=steps(step("Given", q.expr), "Apply power rule", step("Answer", df))
    )
end
```

**Helpers:**
- `@script x y begin ... end` — declares variables, wraps body in generate function, calls `run_batch`
- `rand_linear(x)` → `(expr=ax+b, a, b)` with keyword ranges
- `rand_quadratic(x)` → `(expr=ax²+bx+c, a, b, c)`
- `rand_factorable(x)` → `(expr=a(x-r1)(x-r2) expanded, a, r1, r2)`
- `rand_poly(x, n)` → `(expr, coeffs::Vector)`
- `step("Label", expr)` → `"Label: $\LaTeX$"`, `step(expr)` → `"$\LaTeX$"`
- `set_topic!("main/sub")` — set default topic for all `problem()` calls in a script

### SymPy -> Symbolics.jl Cheat Sheet

| Python (SymPy) | Julia (Symbolics.jl) |
|---|---|
| `symbols('x y')` | `@variables x y` |
| `latex(expr)` | `tex(expr)` |
| `Rational(a, b)` | `a // b` (Julia built-in) |
| `simplify(expr)` | `simplify(expr)` |
| `solve(eq, x)` | `solve(eq, x)` (provided by ProblemUtils) |
| `Eq(lhs, rhs)` | `lhs ~ rhs` |
| `subs(expr, {x: 2})` | `substitute(expr, x => 2)` |
| `randint(lo, hi)` | `randint(lo, hi)` (provided by ProblemUtils) |
| `choice([a,b])` | `choice([a,b])` (provided by ProblemUtils) |
| `emit(generate())` | `run_batch(generate)` (handles --count N) |
| `Diagram()` / `Graph()` | `DiagramObj()` / `GraphObj()` |

## Workflow

### Interactive (Web UI)

1. Open `http://localhost:9091`
2. Configure LLM provider and model
3. Select topic and subtopic
4. Generate a script (LLM writes a Julia/Python function that produces problems)
5. Test the script (runs it, validates output)
6. Save to `scripts/src/`
7. Run to generate problems in bulk
8. Review staged problems
9. Approve and insert into Postgres directly

### Automated Pipeline

```bash
python automate_pipeline.py [options]
```

Full automation: fetches topics from main backend, generates scripts for all topic/subtopic combinations, mass-generates problems, uploads directly.

| Flag | Description |
|---|---|
| `--skip-generation` | Use existing scripts, skip LLM generation |
| `--problems-per-script N` | Override default 100 problems per script |
| `--topics "topic1,topic2"` | Only process specific topics |
| `--dry-run` | Show what would be done |
| `--clear-before` | Clear staged problems before starting |
| `--overwrite` | Overwrite existing scripts |
| `--timeout N` | LLM timeout in seconds (default: 300) |
| `--log-file PATH` | Write detailed logs to file |

Concurrency: 4 simultaneous LLM requests. Retries with exponential backoff (max 3 retries).

## Standalone Scripts

### `import_db.py` - Import SQLite to PostgreSQL

```bash
python import_db.py problems-v1.db
python import_db.py problems-v1.db --url postgres://locus:pass@localhost:5433/locus
python import_db.py problems-v1.db --dry-run
```

Batch inserts (2000 rows), skips duplicates by `(question_latex, answer_key)`.

### `publish_db.py` - Publish to GitHub Releases

```bash
python publish_db.py exports/*.sql
python publish_db.py exports/*.sql --tag problems-v2
python publish_db.py exports/*.sql --dry-run
```

Combines SQL files into a SQLite database, deduplicates, publishes to GitHub Releases on `FizzWizZleDazzle/locus-scripts`.

### `topup.py` - Boost Thin Subtopics

```bash
python topup.py /tmp/locus_check.db --target 1500 --output exports/topup.sql
```

Finds subtopics below the target count, locates their scripts, runs them with a 2x safety buffer. 8 parallel workers.

## Problem Format

Each generated problem must include:

| Field | Description |
|---|---|
| `question_latex` | LaTeX-formatted question |
| `answer_key` | SymEngine-compatible expression |
| `difficulty` | ELO-style rating (100-5000) |
| `main_topic` | Must match a topic in the database |
| `subtopic` | Must match a subtopic under the topic |
| `grading_mode` | `equivalent`, `factor`, or `expand` |
| `answer_type` | `expression`, `numeric`, `set`, `tuple`, `list`, `interval`, `inequality`, `equation`, `boolean`, `word`, `matrix`, `multi_part` |
| `calculator_allowed` | `none`, `scientific`, `graphing`, `cas` |
| `solution_latex` | Step-by-step solution in LaTeX (can be empty) |
| `question_image` | Compressed SVG (s1: prefix) or empty |
| `time_limit_seconds` | Optional, 1-3600 |
