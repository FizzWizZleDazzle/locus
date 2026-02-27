# Locus

Competitive math learning platform: Rust backend (Axum) + frontend (Leptos WASM) with SymEngine CAS grading.

## Documentation Maintenance

When modifying code, update the corresponding doc:

| Change area | Update |
|---|---|
| API routes, request/response types | `docs/API.md` |
| Database migrations, models | `docs/DATABASE.md` |
| Crate structure, modules, grading logic | `docs/ARCHITECTURE.md` |
| Env vars, Docker, K8s, scripts | `docs/DEPLOYMENT.md` |
| Factory pipeline, scripts, LLM config | `factory/README.md` |

## SymEngine FFI Safety (CRITICAL)

These rules prevent segfaults and undefined behavior. Violating them crashes the process.

1. **Always guard `number_is_zero()` with `is_a_Number()` first** - segfaults on non-Number types in native builds
2. **Native SymEngine is NOT thread-safe** - all FFI calls are serialized through a global `Mutex` (`SYMENGINE_LOCK`); on WASM this is a no-op
3. **`Expr::clone()` must use a single lock acquisition** - do not call `to_string()` then `parse()` separately (deadlock risk)
4. **WASM allocator bridge is mandatory** - `frontend/src/main.rs` provides `malloc`/`free`/`calloc`/`realloc` that delegate to Rust's allocator; removing them causes "env" module import errors
5. **wasi-libc's dlmalloc is stripped** from `libc.a` to prevent dual-allocator conflicts

## Dev Quickstart

```bash
./dev.sh                    # Starts DB (5433), backend (3000), frontend (8080)
cd factory && ./start.sh    # Starts factory backend (9090) + UI (9091)
```

Requires: `cargo`, `trunk`, `cargo-watch`, `docker`. Creates `.env` from `.env.example` automatically.

## Documentation Index

Prioritize reading documentation before diving into code. Each doc is concise and focused on a specific area. When making changes, update the relevant doc to keep it accurate.

| Document | Purpose |
|---|---|
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Crate map, module inventory, grading system, ELO, auth flows, build system |
| [`docs/API.md`](docs/API.md) | All HTTP endpoints with methods, auth, request/response types, rate limiting |
| [`docs/DATABASE.md`](docs/DATABASE.md) | All tables, columns, migrations, PostgreSQL functions, constraints |
| [`docs/DEPLOYMENT.md`](docs/DEPLOYMENT.md) | Environment variables, Docker, dev containers, Kubernetes, Cloudflare, scripts |
| [`factory/README.md`](factory/README.md) | Problem generation pipeline, LLM config, automation scripts |

No coauthor/cosign on commits