# Architecture

## Workspace

Rust workspace with three crates. The `common` crate is shared between frontend (WASM) and backend (native) via conditional compilation.

```
Cargo.toml              Workspace root
crates/
  common/               Shared library (grading, SymEngine FFI, types)
  frontend/             Leptos 0.7 CSR app (wasm32-unknown-unknown)
  backend/              Axum REST API (native target)
```

## Crate Map

### `common` (locus-common)

Shared between frontend and backend. Compiles to both WASM and native targets.

| File | Purpose |
|---|---|
| `src/lib.rs` | Shared types: `MainTopic`, `AnswerType`, `GradingMode`, API request/response structs, topic definitions |
| `src/constants.rs` | Game constants: `DEFAULT_ELO` (1500), `PROBLEM_BATCH_SIZE` |
| `src/symengine.rs` | SymEngine FFI bindings: `Expr` type, `parse`, `expand`, `subs2`, `evalf`, `free_symbols`, `is_a_Number`, `number_is_zero`. Global `Mutex` on native; no-op on WASM |
| `src/grader/mod.rs` | `ExprEngine` trait, `grade_answer()` dispatcher (routes by `AnswerType`), `check_answer_expr()` two-stage equivalence |
| `src/grader/expression.rs` | Expression grading with `Factor`/`Expand` mode enforcement |
| `src/grader/numeric.rs` | Numeric comparison |
| `src/grader/set.rs` | Unordered element-wise comparison (strips `{}`) |
| `src/grader/ordered.rs` | Tuple/List ordered comparison (strips `()`/`[]`) |
| `src/grader/interval.rs` | Interval bound comparison. DB format: `open:1,closed:7` or JSON `{interval:{open:1,closed:7}}`. User format: `(1, 7]`. Supports unions with `\|` or `union:` prefix |
| `src/grader/inequality.rs` | Parses inequalities (`x > -4`, `-2 < x <= 5`), converts to interval, delegates to interval grader |
| `src/grader/equation.rs` | Splits on `=`, checks LHS-RHS difference equivalence and proportionality |
| `src/grader/boolean.rs` | True/false parsing (accepts `true`, `yes`, `t`, `1`, etc.) |
| `src/grader/word.rs` | Case-insensitive exact string match |
| `src/grader/matrix.rs` | 2D element-wise comparison (`[[a, b], [c, d]]`) |
| `src/grader/multipart.rs` | Splits on `\|\|\|`, grades each part independently with its own answer type |
| `src/grader/parse.rs` | Shared parsing: `split_top_level()`, `split_equation()` |
| `src/elo.rs` | ELO calculation: `K_FACTOR=32`, `expected_score()`, `time_multiplier()`, `calculate_new_elo()` |
| `src/latex.rs` | LaTeX to plain text converter: fractions, sqrt, trig, inverse trig, hyperbolic, Greek symbols, comparison operators, matrix environments, implicit multiplication |
| `src/validation.rs` | Username/password/email validation rules shared between frontend and backend |
| `src/svg_compress.rs` | Dictionary-based SVG compression (prefix `s1:`) |
| `build.rs` | Conditional linking: WASM links from `symengine.js/dist/wasm-unknown/lib/`, native links `/usr/local/lib/libsymengine.a` + system gmp/stdc++ |

### `frontend` (locus-frontend)

Leptos 0.7 CSR app. Compiles to `wasm32-unknown-unknown`.

| File | Purpose |
|---|---|
| `src/main.rs` | App root: C allocator bridge (malloc/free/calloc/realloc), routing, AuthContext, ThemeContext |
| `src/api.rs` | Gloo HTTP client for all backend endpoints. Token storage in LocalStorage |
| `src/grader.rs` | Client-side grading: LaTeX preprocessing via `convert_latex_to_plain()`, calls `locus_common::grader::grade_answer()` |
| `src/env.rs` | Compile-time config: `api_base()` from `LOCUS_API_URL`, `frontend_base()` from `LOCUS_FRONTEND_URL` |
| `src/oauth.rs` | OAuth popup window management with postMessage callback |
| `src/problem_queue.rs` | Pre-fetches problems in batches, auto-refills at 5 remaining |
| `src/katex_bindings.rs` | KaTeX JS bindings for LaTeX rendering |
| `src/utils.rs` | Utility functions |
| `src/components/mod.rs` | Component re-exports |
| `src/components/math_field.rs` | MathQuill wrapper: creates MQ.MathField, edit/enter handlers, template pre-seeding, restriction support |
| `src/components/answer_input.rs` | Per-AnswerType dispatcher: templates (Set/Tuple/List/Equation), restrictions (Numeric), affordances (Interval bracket toggles, Inequality palette, Matrix +/-row/col with dynamic template, Boolean True/False toggle, MultiPart stacked fields) |
| `src/components/latex_renderer.rs` | KaTeX LaTeX renderer component |
| `src/components/navbar.rs` | Top navigation bar |
| `src/components/sidebar.rs` | Side navigation |
| `src/components/problem_card.rs` | Problem display card |
| `src/components/problem_interface.rs` | Full problem UI with timer, input, grading feedback |
| `src/components/timer.rs` | Countdown timer |
| `src/components/topic_selector.rs` | Topic/subtopic filter UI |
| `src/formatters/` | `common.rs`, `equation.rs`, `inequality.rs`, `interval.rs`, `matrix.rs`, `multi_part.rs`, `set.rs`, `tests.rs` - Format grader results for display |
| `src/pages/` | `home`, `login`, `register`, `verify_email`, `forgot_password`, `reset_password`, `practice`, `ranked`, `leaderboard`, `stats`, `settings`, `privacy_policy`, `terms_of_service` |

### `backend` (locus-backend)

Axum REST API. Compiles to native target.

| File | Purpose |
|---|---|
| `src/main.rs` | Server init: load config, connect DB, run migrations, build router, start Axum |
| `src/config.rs` | `Config` struct loaded from environment variables |
| `src/db.rs` | PostgreSQL connection pool (max 10 connections) |
| `src/grader.rs` | Server-side grading wrapper: calls `locus_common::grader::grade_answer()` |
| `src/email.rs` | `EmailService` using Resend: verification emails, password reset emails |
| `src/rate_limit.rs` | IP-based rate limiting via governor: auth (5/15min), login (10/15min), general (1000/min) |
| `src/topics.rs` | `TopicCache`: in-memory cache of enabled topics/subtopics, periodic refresh |
| `src/api/mod.rs` | `AppState` struct, router assembly |
| `src/api/auth.rs` | Auth endpoints: register, login, set-password, change-password, change-username, delete-account, unlink-oauth, verify-email, resend-verification, forgot-password, validate-reset-token, reset-password |
| `src/api/problems.rs` | `GET /problems`: fetch random problems with topic/subtopic/ELO filters |
| `src/api/submit.rs` | `POST /submit`: grade answer, update ELO and streaks, record attempt |
| `src/api/leaderboard.rs` | `GET /leaderboard`: top 100 users by topic ELO |
| `src/api/stats.rs` | `GET /user/stats`, `GET /user/elo-history`: per-topic stats and 30-day chart data |
| `src/api/topics.rs` | `GET /topics`: enabled topics and subtopics from cache |
| `src/api/oauth.rs` | OAuth flows: Google/GitHub login, callback, account linking |
| `src/auth/mod.rs` | Auth module re-exports |
| `src/auth/jwt.rs` | JWT creation/verification (HS256, 24-hour expiry) |
| `src/auth/middleware.rs` | `AuthUser` extractor: verifies JWT, returns user UUID |
| `src/models/mod.rs` | Model re-exports |
| `src/models/user.rs` | `User`, `OAuthAccount`, `LeaderboardRow` with all DB queries |
| `src/models/problem.rs` | `Problem` with random selection, batch fetch, difficulty matching |
| `src/models/attempt.rs` | `Attempt` recording and aggregation |
| `src/models/email_verification.rs` | `EmailVerificationToken`: generation, validation, rate limiting |
| `src/models/password_reset.rs` | `PasswordResetToken`: generation, validation, rate limiting |

## Factory (Python)

Problem generation pipeline. See [`../factory/README.md`](../factory/README.md).

```
factory/
  backend/              FastAPI app (port 9090)
    main.py             Entry point
    config.py           LLM + database configuration
    models.py           Pydantic models
    routes/             config_routes, problem_routes, script_routes
    services/           LLM, script execution, DB access, validation
    scripts/src/        Generated Python problem scripts
  frontend/             TypeScript UI (port 9091)
  automate_pipeline.py  Full automation: topics -> scripts -> problems -> upload
  import_db.py          SQLite -> PostgreSQL importer
  publish_db.py         SQL -> SQLite -> GitHub Release
  topup.py              Re-run scripts for thin subtopics
```

## Grading System

### Dispatcher

`grade_answer()` in `common/src/grader/mod.rs` routes by `AnswerType`:

| AnswerType | Grader | Format |
|---|---|---|
| `expression` | `expression.rs` | SymEngine expression, respects `GradingMode` (Equivalent/Factor/Expand) |
| `numeric` | `numeric.rs` | Number comparison |
| `set` | `set.rs` | `{a, b, c}` - unordered, unique |
| `tuple` | `ordered.rs` | `(a, b)` - ordered |
| `list` | `ordered.rs` | `[a, b]` - ordered |
| `interval` | `interval.rs` | `(1, 7]`, unions with `U`. Also parses factory JSON format `{interval:{...}}` |
| `inequality` | `inequality.rs` | `x > -4`, `-2 < x <= 5` |
| `equation` | `equation.rs` | `LHS = RHS` |
| `boolean` | `boolean.rs` | true/false |
| `word` | `word.rs` | Case-insensitive string |
| `matrix` | `matrix.rs` | `[[a, b], [c, d]]` |
| `multi_part` | `multipart.rs` | Parts split by `\|\|\|`, each with own type |

### Two-Stage Expression Equivalence

1. **Symbolic**: `expand(user - key) == 0`
2. **Numerical**: Evaluate both at test points `[0.7, 1.3, 2.1, -0.4, 0.31]` with `|diff| < 1e-6` tolerance

### Input Pipeline

```
MathQuill -> field.latex() -> convert_latex_to_plain() -> Plain text -> Grader
```

## ELO System

Standard ELO with time bonus. K-factor = 32. Per-topic ratings stored in `user_topic_elo`.

- `expected_score(a, b) = 1 / (1 + 10^((b-a)/400))`
- Time multiplier: faster solving = up to 1.5x bonus (per-problem time limits override difficulty tiers)
- Default ELO: 1500
- Problem selection: weighted by proximity to user ELO + randomness

## Authentication

### Email/Password
1. Register with email + password (Argon2 hashed)
2. Verification email sent via Resend (1-hour token)
3. Login returns JWT (HS256, 24-hour expiry)
4. Password reset via email (30-minute token)

### OAuth (Google, GitHub)
1. Frontend opens popup to `/auth/oauth/{provider}`
2. Server generates CSRF state JWT (10-minute expiry), redirects to provider
3. Provider callback creates or links account
4. Response sent to opener via `postMessage`
5. Email conflict: must log in first and link in Settings

## Build System

### WASM Target (frontend)

`common/build.rs` links pre-compiled WASM libraries from `symengine.js/dist/wasm-unknown/lib/`:
- `libsymengine.a`, `libc++.a`, `libc++abi.a`, `libc.a` (with dlmalloc stripped)
- Compiles `wasi_stub.c` for WASI shims

Frontend `main.rs` provides C allocator bridge (`malloc`/`free`/`calloc`/`realloc`) to prevent dual-allocator conflicts.

### Native Target (backend)

Links `/usr/local/lib/libsymengine.a` (static) + system `libgmp` and `libstdc++` (dynamic).

All FFI calls serialized through global `Mutex` since native SymEngine is not thread-safe.
