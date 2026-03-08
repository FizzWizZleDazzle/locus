# Locus

Competitive math learning platform: Rust backend (Axum) + frontend (Leptos WASM) with SymEngine CAS grading.

## READ DOCS FIRST (MANDATORY)

**Before modifying ANY code, read the relevant documentation below.** The docs are concise and give you everything you need — crate maps, type definitions, API contracts, DB schema, and deployment config. Skipping them leads to duplicate work and broken assumptions.

| When you need to... | Read this first |
|---|---|
| Add/modify an API endpoint | [`docs/API.md`](docs/API.md) — all endpoints, auth, request/response types, rate limits |
| Change database schema or queries | [`docs/DATABASE.md`](docs/DATABASE.md) — all tables, columns, indexes, migrations, PG functions |
| Understand crate structure, grading, ELO, auth | [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — module map, grading dispatch, build system, caching, routing |
| Change env vars, Docker, K8s, deployment | [`docs/DEPLOYMENT.md`](docs/DEPLOYMENT.md) — all env vars, Docker files, Helm, Cloudflare, scripts |
| Modify problem generation pipeline | [`factory/README.md`](factory/README.md) — LLM config, Julia/Python scripts, validation, automation |

## Documentation Maintenance

When modifying code, **always update the corresponding doc**:

| Change area | Update |
|---|---|
| API routes, request/response types | `docs/API.md` |
| Database migrations, models, indexes | `docs/DATABASE.md` |
| Crate structure, modules, grading, components, pages | `docs/ARCHITECTURE.md` |
| Env vars, Docker, K8s, scripts | `docs/DEPLOYMENT.md` |
| Factory pipeline, scripts, LLM config | `factory/README.md` |

## Key Conventions

- **Rust workspace**: `common` (shared types + grading + SymEngine FFI), `frontend` (Leptos WASM), `backend` (Axum)
- **Grading runs on both sides**: client-side (practice) and server-side (ranked) via shared `common` crate
- **Auth**: httpOnly cookie (`locus_token`) preferred; Bearer JWT as fallback. No API keys.
- **Caching**: TopicCache (in-memory, daily refresh), LeaderboardCache (per-topic, 5min TTL), DailyPuzzleCache (per-day)
- **Frontend input**: MathQuill for math input → `convert_latex_to_plain()` → grader
- **Badges**: computed dynamically from stats (no DB tables). Fun badges exist but are hidden from public API.
- **Tests**: `cargo test -p locus-common` for grader tests. `grade-check` binary for factory round-trip validation.

## SymEngine FFI Safety (CRITICAL)

These rules prevent segfaults and undefined behavior. Violating them crashes the process.

1. **Always guard `number_is_zero()` with `is_a_Number()` first** — segfaults on non-Number types in native builds
2. **Native SymEngine is NOT thread-safe** — all FFI calls are serialized through a global `Mutex` (`SYMENGINE_LOCK`); on WASM this is a no-op
3. **`Expr::clone()` must use a single lock acquisition** — do not call `to_string()` then `parse()` separately (deadlock risk)
4. **WASM allocator bridge is mandatory** — `frontend/src/main.rs` provides `malloc`/`free`/`calloc`/`realloc` that delegate to Rust's allocator; removing them causes "env" module import errors
5. **wasi-libc's dlmalloc is stripped** from `libc.a` to prevent dual-allocator conflicts

## Dev Quickstart

```bash
./dev.sh                    # Starts DB (5433), backend (3000), frontend (8080)
cd factory && ./start.sh    # Starts factory backend (9090) + UI (9091)
```

Requires: `cargo`, `trunk`, `cargo-watch`, `docker`. Creates `.env` from `.env.example` automatically.

No coauthor/cosign on commits
