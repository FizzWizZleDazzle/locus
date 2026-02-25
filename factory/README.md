# Factory

LLM-powered problem generation pipeline. Generates Python scripts that produce math problems, validates them with SymPy, and uploads to the main Locus backend.

## Architecture

```
factory/
  backend/                    FastAPI app (port 9090)
    main.py                   Entry point, route registration
    config.py                 LLM + Locus backend configuration
    models.py                 Pydantic request/response models
    routes/
      config_routes.py        LLM and backend config endpoints
      script_routes.py        Script generation, saving, testing, execution
      problem_routes.py       Problem staging, approval, export
    services/                 LLM client, script runner, problem staging
    scripts/src/              Generated problem scripts (Python)
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

`start.sh` creates a Python venv, installs dependencies, compiles TypeScript, starts the FastAPI backend (uvicorn with hot reload) and an HTTP server for the frontend.

### Environment Variables

| Variable | Description |
|---|---|
| `LLM_ENDPOINT` | `https://api.openai.com/v1/chat/completions` or `https://api.anthropic.com/v1/messages` |
| `LLM_API_KEY` | API key for the LLM provider |
| `LLM_MODEL` | Model name (e.g. `gpt-4`, `claude-sonnet-4-5-20250929`) |
| `LOCUS_BACKEND_URL` | Main backend URL (default: `http://localhost:3000`) |
| `LOCUS_API_KEY` | Must match `API_KEY_SECRET` in the main backend |

## Workflow

### Interactive (Web UI)

1. Open `http://localhost:9091`
2. Configure LLM provider and model
3. Select topic and subtopic
4. Generate a script (LLM writes a Python function that produces problems)
5. Test the script (runs it, validates output with SymPy)
6. Save to `scripts/src/`
7. Run to generate problems in bulk
8. Review staged problems
9. Approve and upload to main backend via API

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
| `difficulty` | ELO-style rating (1000-2000) |
| `main_topic` | Must match a topic in the database |
| `subtopic` | Must match a subtopic under the topic |
| `grading_mode` | `equivalent`, `factor`, or `expand` |
| `answer_type` | `expression`, `numeric`, `set`, `tuple`, `list`, `interval`, `inequality`, `equation`, `boolean`, `word`, `matrix`, `multi_part` |
| `calculator_allowed` | `none`, `scientific`, `graphing`, `cas` |
| `solution_latex` | Step-by-step solution in LaTeX (can be empty) |
| `question_image` | Compressed SVG (s1: prefix) or empty |
| `time_limit_seconds` | Optional, 1-3600 |
