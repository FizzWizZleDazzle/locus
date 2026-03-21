# Locus

A competitive math learning platform where users solve problems, earn ELO ratings across topics, and climb leaderboards. Problems are graded symbolically using a computer algebra system (SymEngine) that runs both client-side (WASM) and server-side (native).

## Features

- **Symbolic grading** - SymEngine CAS checks mathematical equivalence, not just string matching
- **12 answer types** - Expressions, equations, inequalities, intervals, sets, tuples, matrices, and more
- **Per-topic ELO ratings** - Track skill across 9 math topics from arithmetic to linear algebra
- **Practice and ranked modes** - Untimed practice with solutions, or ranked with ELO stakes
- **Client-side pre-grading** - Instant feedback via WASM before server confirmation
- **MathLive input** - Rich math editor with MathJSON output
- **OAuth** - Google and GitHub login alongside email/password
- **Problem factory** - LLM-powered pipeline generates and validates thousands of problems

## Tech Stack

| Layer | Technology |
|---|---|
| Frontend | Leptos 0.7 (Rust WASM, CSR) |
| Backend | Axum (Rust) |
| Database | PostgreSQL 16 |
| CAS | SymEngine (WASM + native) |
| Math input | MathLive + KaTeX |
| Auth | JWT + Argon2 + OAuth (Google, GitHub) |
| Email | Resend |
| Factory | FastAPI (Python) + LLM |
| Deployment | Docker, Kubernetes (Helm), Cloudflare Pages |

## Quick Start

### Dev container (recommended)

Open in VS Code with the Dev Containers extension or GitHub Codespaces. Everything is pre-installed.

### Local development

```bash
# Prerequisites: cargo, trunk, cargo-watch, docker, SymEngine
./dev.sh
```

This starts PostgreSQL (port 5433), the backend (port 3000), and the frontend (port 8080).

### Production

```bash
make init       # Generate production secrets in .env
make all        # Build, push, deploy backend + frontend, load data
make status     # Verify deployment health
```

See [`docs/DEPLOYMENT.md`](docs/DEPLOYMENT.md) for full details.

## Project Structure

```
crates/
  common/       Shared types, SymEngine FFI, grading engine, ELO, validation
  frontend/     Leptos WASM app (MathLive input, client-side grading, OAuth)
  backend/      Axum REST API (auth, problems, submit, leaderboard, stats)
factory/        Python problem generation pipeline (FastAPI + LLM)
helm/           Kubernetes Helm chart
scripts/        Deployment and data loading scripts
.devcontainer/  VS Code dev container configuration
```

## Documentation

- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) - Crate map, grading system, build system
- [`docs/API.md`](docs/API.md) - HTTP endpoint reference
- [`docs/DATABASE.md`](docs/DATABASE.md) - Schema, migrations, functions
- [`docs/DEPLOYMENT.md`](docs/DEPLOYMENT.md) - Environment variables, Docker, Kubernetes
- [`factory/README.md`](factory/README.md) - Problem generation pipeline

## License

All rights reserved.
