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
| Modify problem generation | `crates/dsl/` + `crates/dsl-cli/` — YAML DSL parser, generator, AI scaffolding |

## Documentation Maintenance

When modifying code, **always update the corresponding doc**:

| Change area | Update |
|---|---|
| API routes, request/response types | `docs/API.md` |
| Database migrations, models, indexes | `docs/DATABASE.md` |
| Crate structure, modules, grading, components, pages | `docs/ARCHITECTURE.md` |
| Env vars, Docker, K8s, scripts | `docs/DEPLOYMENT.md` |

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
2. **Native SymEngine is built with `WITH_SYMENGINE_THREAD_SAFE=ON`** — atomic refcounts + atomic hash caching mean each thread can safely own its own `Expr`. `se_lock!` is a no-op. `Expr` is `Send` but not `Sync` — never share refs across threads.
3. **WASM allocator bridge is mandatory** — `frontend/src/main.rs` provides `malloc`/`free`/`calloc`/`realloc` that delegate to Rust's allocator; removing them causes "env" module import errors
4. **wasi-libc's dlmalloc is stripped** from `libc.a` to prevent dual-allocator conflicts

## Dev Quickstart

```bash
./dev.sh                    # Starts DB (5433), backend (3000), frontend (8080)
./dev.sh --services         # Starts services-backend (8090) + status page (8082)
```

Requires: `cargo`, `trunk`, `cargo-watch`, `docker`. Creates `.env` from `.env.example` automatically.

No coauthor/cosign on commits
