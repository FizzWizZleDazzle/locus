# Contributing to Locus

## Ways to Contribute

### 1. Add Problem YAMLs (easiest)

Write new math problems using the LocusDSL. No Rust knowledge needed.

```bash
# 1. Create a YAML file in problems/{topic}/
# 2. Validate it
cargo run --bin dsl-cli -- validate problems/your_topic/your_file.yaml

# 3. Generate test problems
cargo run --bin dsl-cli -- generate problems/your_topic/your_file.yaml -n 10

# 4. Submit a PR
```

See [`docs/DSL_SPEC.md`](docs/DSL_SPEC.md) for the full DSL reference.

**Quick example:**
```yaml
topic: algebra1/two_step_equations
difficulty: easy
calculator: none

variables:
  a: nonzero(-8, 8)
  b: integer(-10, 10)
  answer: integer(-10, 10)
  rhs: a * answer + b

constraints:
  - a != 1
  - a != -1

question: "Solve for x: {equation(a*x + b, rhs)}"

answer: answer

solution:
  - "Start with {a}x + {b} = {rhs}"
  - "Subtract {b}: {a}x = {rhs} - {b}"
  - "Divide by {a}: x = {answer}"
```

### 2. Improve the DSL Parser

The parser lives in `crates/dsl/`. Key areas:
- `functions.rs` — add new math operations
- `display.rs` — add new display functions for LaTeX formatting
- `sampler.rs` — add new random variable types
- `diagram/` — SVG diagram generation (Typst + circuitikz)

### 3. Fix Bugs / Improve Grading

- `crates/common/src/grader/` — grading logic for all answer types
- `crates/common/src/katex_validate.rs` — KaTeX rendering validation
- `crates/frontend/` — Leptos WASM frontend

### 4. Add Physics C Support

We're expanding into AP Physics C (Mechanics + E&M). See `docs/DSL_SPEC.md` section 12-13.

## Development Setup

```bash
# Prerequisites: Rust, Docker, cargo-watch, trunk
git clone https://github.com/FizzWizZleDazzle/locus.git
cd locus
./dev.sh  # Starts DB (5433), backend (3000), frontend (8080)
```

## Running Tests

```bash
cargo test -p locus-common      # Grader + validator tests
cargo test -p locus-dsl          # DSL parser tests
cargo run --bin dsl-cli -- validate problems/  # All problem files
```

## Branching Workflow

The repo has two long-lived protected branches:

- **`stable`** — what's deployed to production. Only PRs from `main` or
  `hotfix/<slug>` can target it. CI must be green; admin pushes are blocked.
  This is the lane for shipping a security fix without dragging unfinished
  `main` work along.
- **`main`** — integration branch. All feature work lands here via PR. May
  contain features that are still being smoke-tested on staging. Direct
  pushes (including by admins) are blocked.

### Routine feature work → main

Use a personal namespace for branches:

```bash
git checkout -b <your-github-username>/<short-feature-slug>
git push -u origin HEAD
gh pr create --base main --fill
```

Examples: `fizzwizzledazzle/gpu-enumerator`, `alice/algebra2-yamls`. One
topic per branch — keep them short-lived and rebase or merge `main` into
them frequently.

### Promoting main → stable

Once main is verified on staging, open a PR from `main` to `stable`:

```bash
gh pr create --base stable --head main --title "Promote main to stable"
```

The Stable Gate workflow only allows `main` and `hotfix/*` as the head
branch — any other source is rejected automatically.

### Hotfixes (security or prod-breaking)

Branch off `stable`, fix, then PR into both `stable` and `main`:

```bash
git checkout -b hotfix/<slug> origin/stable
# … fix …
git push -u origin HEAD
gh pr create --base stable --fill   # ships to prod
gh pr create --base main --fill     # forward-port the fix
```

Hotfix PRs to stable bypass the "wait for main to be ready" cycle — the
fix lands directly. The forward-port PR keeps main from regressing.

## PR Guidelines

- One topic per PR
- All problem YAMLs must pass `dsl-cli validate`
- Run `cargo clippy` before submitting
- Include test output for new problems
- Resolve CI failures before requesting merge — admins won't bypass the gate
