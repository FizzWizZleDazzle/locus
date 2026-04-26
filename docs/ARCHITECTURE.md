# Architecture

## Workspace

Rust workspace. The `common` crate is shared between frontend (WASM) and backend (native) via conditional compilation. The `services-backend` and `status` crates are standalone (no dependency on `common`).

```
Cargo.toml              Workspace root
crates/
  common/               Shared library (grading, SymEngine FFI, types)
  frontend/             Leptos 0.8 CSR app (wasm32-unknown-unknown)
  backend/              Axum REST API (native target)
  dsl/                  Problem DSL parser + generator
  dsl-cli/              DSL CLI tool (generate, validate, parse, ai)
  services-backend/     Services API (status page, Axum)
  status/               Status page frontend (Leptos WASM, Cloudflare Pages)
```

### Services Architecture

```
status.locusmath.org -> Cloudflare Pages (crates/status WASM)
                             |
                             v  API calls
                        crates/services-backend (K8s pod, port 8090)
                             |
                             v
                        PostgreSQL (shared with main backend)
                             + pings api.locusmath.org/api/health
```

Services backend runs its own migrations (`crates/services-backend/migrations/`) and manages the `status_checks` table. Auth reads the main `users` table directly (shared JWT_SECRET). The `forum_*` tables from migration 001 are orphaned — the forum was migrated to GitHub Discussions and no code currently reads them.

## Crate Map

### `common` (locus-common)

Shared between frontend and backend. Compiles to both WASM and native targets.

| File | Purpose |
|---|---|
| `src/badges.rs` | Badge definitions: `BadgeCategory`, `BadgeTier`, `EarnedBadge`, `BadgeDisplay`, `compute_badges()`, `compute_all_badges()` — 29 badges across 6 categories (Streak, Elo, Problems, TopicMastery, DailyPuzzle, Fun), computed dynamically from stats |
| `src/lib.rs` | Shared types: `MainTopic`, `AnswerType`, `GradingMode`, API request/response structs (incl. `PublicProfileResponse`), topic definitions |
| `src/constants.rs` | Game constants: `DEFAULT_ELO` (1500), `PROBLEM_BATCH_SIZE`, `WARMUP_SIZE` (5) |
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
| `src/svg_compress.rs` | Dictionary-based SVG compression (prefix `s1:`) — 24 token mappings for compact DB storage |
| `build.rs` | Conditional linking: WASM links from `symengine.js/dist/wasm-unknown/lib/`, native links `/usr/local/lib/libsymengine.a` + system gmp/stdc++ |

### `frontend` (locus-frontend)

Leptos 0.7 CSR app. Compiles to `wasm32-unknown-unknown`.

| File | Purpose |
|---|---|
| `src/main.rs` | App root: C allocator bridge (malloc/free/calloc/realloc), routing, AuthContext, ThemeContext |
| `src/api.rs` | Gloo HTTP client for all backend endpoints. Username in LocalStorage; JWT in httpOnly cookie. Auto-redirects to `/login` on 401. |
| `src/grader.rs` | Client-side grading: LaTeX preprocessing via `convert_latex_to_plain()`, calls `locus_common::grader::grade_answer()` |
| `src/env.rs` | Compile-time config: `api_base()` from `LOCUS_API_URL`, `frontend_base()` from `LOCUS_FRONTEND_URL` |
| `src/oauth.rs` | OAuth popup window management with postMessage callback |
| `src/problem_queue.rs` | Pre-fetches problems in batches, auto-refills at 5 remaining |
| `src/katex_bindings.rs` | KaTeX JS bindings for LaTeX rendering |
| `src/utils.rs` | Utility functions |
| `src/components/mod.rs` | Component re-exports |
| `src/components/activity_matrix.rs` | GitHub-style activity heatmap (365 days, SVG-rendered, CSS variable theming) — shared by stats and profile pages |
| `src/components/badge_grid.rs` | Badge grid: 6 cols desktop / 3 mobile, tier colors for earned, mystery "?" for locked. Badge images at `/badges/{badge_id}.png` |
| `src/components/math_field.rs` | MathQuill wrapper: creates MQ.MathField, edit/enter handlers, template pre-seeding, restriction support |
| `src/components/answer_input.rs` | Per-AnswerType dispatcher: templates (Set/Tuple/List/Equation), restrictions (Numeric), affordances (Interval bracket toggles, Inequality palette, Matrix +/-row/col with dynamic template, Boolean True/False toggle, MultiPart stacked fields) |
| `src/components/latex_renderer.rs` | KaTeX LaTeX renderer component |
| `src/components/navbar.rs` | Top navigation with logo, theme toggle, user menu |
| `src/components/sidebar.rs` | Side navigation (shown when logged in) |
| `src/components/problem_card.rs` | Problem display card (question, optional image, time limit indicator) |
| `src/components/problem_interface.rs` | Full problem UI with timer, input, hints, solution, whiteboard toggle |
| `src/components/timer.rs` | Countdown timer with visual feedback |
| `src/components/topic_selector.rs` | Topic/subtopic dropdown selector |
| `src/components/whiteboard.rs` | Fabric.js drawing canvas — pen, text, eraser tools with floating toolbar. Canvas stored on `window.__wb_canvas` |
| `src/components/draggable.rs` | Reusable draggable wrapper component |
| `src/formatters/` | `common.rs`, `equation.rs`, `inequality.rs`, `interval.rs`, `matrix.rs`, `multi_part.rs`, `set.rs`, `tests.rs` — format grader results for display |
| `src/pages/` | 16 pages (see Frontend Routing below) |

### `backend` (locus-backend)

Axum REST API. Compiles to native target.

| File | Purpose |
|---|---|
| `src/main.rs` | Server init: load config, connect DB, run migrations, build router, start Axum |
| `src/config.rs` | `Config` struct loaded from environment variables |
| `src/db.rs` | PostgreSQL connection pool (configurable via `MAX_DB_CONNECTIONS`, default 10 dev / 50 prod) |
| `src/grader.rs` | Server-side grading wrapper: calls `locus_common::grader::grade_answer()` |
| `src/email.rs` | `EmailService` using Resend: verification emails, password reset emails |
| `src/rate_limit.rs` | IP-based rate limiting via governor: auth (5/15min), login (10/15min), sensitive (5/15min), general (1000/min) |
| `src/topics.rs` | `TopicCache`: in-memory cache of enabled topics/subtopics, daily background refresh |
| `src/api/mod.rs` | `AppState` struct (includes all caches), router assembly |
| `src/api/auth.rs` | Auth endpoints: register, login, logout, set-password, change-password, change-username, delete-account, unlink-oauth, verify-email, resend-verification, forgot-password, validate-reset-token, reset-password |
| `src/api/problems.rs` | `GET /problems`: fetch random problems with topic/subtopic/ELO filters |
| `src/api/submit.rs` | `POST /submit`: grade answer (via `spawn_blocking`), update ELO and streaks in transaction, record attempt |
| `src/api/leaderboard.rs` | `GET /leaderboard`: top 100 users by topic ELO (cached 5 min per topic) |
| `src/api/stats.rs` | `GET /user/stats`, `GET /user/elo-history`: per-topic stats and 30-day chart data |
| `src/api/topics.rs` | `GET /topics`: enabled topics and subtopics from cache |
| `src/api/daily.rs` | Daily puzzle endpoints: today (cached), puzzle/{date}, submit, archive, activity |
| `src/api/profile.rs` | `GET /profile/{username}`: public profile with badges, stats, activity matrix |
| `src/api/oauth.rs` | OAuth flows: Google/GitHub login, callback, account linking |
| `src/auth/mod.rs` | Auth module re-exports |
| `src/auth/jwt.rs` | JWT creation/verification (HS256, 24-hour expiry) |
| `src/auth/middleware.rs` | `AuthUser` extractor: verifies JWT from cookie or Authorization header, returns user UUID |
| `src/models/mod.rs` | Model re-exports |
| `src/models/user.rs` | `User`, `OAuthAccount`, `LeaderboardRow` with all DB queries |
| `src/models/problem.rs` | `Problem` with random selection, batch fetch, difficulty matching |
| `src/models/attempt.rs` | `Attempt` recording and aggregation |
| `src/models/email_verification.rs` | `EmailVerificationToken`: generation, validation, rate limiting |
| `src/models/password_reset.rs` | `PasswordResetToken`: generation, validation, rate limiting |
| `src/models/daily_puzzle.rs` | `DailyPuzzle`, `DailyPuzzleAttempt`: daily puzzle + attempt models, activity matrix, streak tracking |
| `src/bin/grade_check.rs` | Grade-check CLI binary: reads JSONL from stdin, self-grades answer_keys via `grade_answer()` |

## Frontend Routing

| Path | Page | Auth |
|---|---|---|
| `/` | Home | None |
| `/practice` | Practice (unranked), Enter key advances | None |
| `/ranked` | Ranked (ELO-tracked), optional warmup, Enter key advances | Required |
| `/daily` | Today's daily puzzle | Optional |
| `/daily/archive` | Paginated puzzle archive | Optional |
| `/daily/puzzle/:date` | Past puzzle detail (answer + editorial) | Optional |
| `/leaderboard` | Global ELO rankings | None |
| `/profile/:username` | Public profile (badges, stats, activity) | None |
| `/stats` | Personal stats dashboard | Required |
| `/settings` | Account management | Required |
| `/login` | Login | None |
| `/register` | Sign up | None |
| `/verify-email` | Email verification callback | None |
| `/forgot-password` | Password reset request | None |
| `/reset-password` | Password reset form | None |
| `/privacy-policy` | Privacy policy | None |
| `/terms-of-service` | Terms of service | None |

## Problem Generation

Problems are authored as YAML files under `problems/` and generated via the Rust `dsl-cli` binary.

```
problems/                     YAML source files (one per topic/subtopic)
crates/dsl/                   DSL parser + SymEngine-backed evaluator
crates/dsl-cli/               CLI: generate, parse, validate, ai
```

Typical workflow:

```bash
cargo run --bin dsl-cli -- generate problems/calculus/derivative_rules.yaml -n 100
cargo run --bin dsl-cli -- validate problems/ --runs 3
```

`dsl-cli` emits JSONL to stdout; `scripts/import_jsonl.py` bulk-loads into PostgreSQL via `COPY`.

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
2. **Variable guard**: Both expressions must have the same set of free symbols (prevents false positives from different variable names like `X` vs `x`)
3. **Numerical**: Evaluate both at test points `[0.7, 1.3, 2.1, -0.4, 0.31]` with distinct per-variable offsets, `|diff| < 1e-6` tolerance

### Input Pipeline

```
MathQuill -> field.latex() -> convert_latex_to_plain() -> Plain text -> Grader
```

## Badge System

29 badges across 6 categories, computed dynamically from user stats (no DB tables). Badge images served from `/badges/{badge_id}.png`.

| Category | Badges | Thresholds |
|---|---|---|
| Streak | `streak_3`, `streak_7`, `streak_30`, `streak_100` | 3/7/30/100 day global streak |
| Elo | `elo_1600`, `elo_1800`, `elo_2000`, `elo_2500` | Peak ELO in any topic |
| Problems | `solved_50`, `solved_250`, `solved_1000`, `solved_5000` | Total correct attempts |
| TopicMastery | `topics_3`, `topics_5`, `topics_8` | Topics with 50+ solves |
| DailyPuzzle | `daily_3`, `daily_7`, `daily_30` | Daily puzzle streak |
| Fun | 11 novelty badges | Various criteria (e.g., `first_blood`, `perfectionist`, `sharpshooter`) |

**Key functions:**
- `compute_badges()` — returns only earned badges (excludes Fun category)
- `compute_all_badges()` — returns all badges with `earned: true/false` (used by profile/stats pages to show locked badges)

Fun badges are defined but **hidden from the public API** — filtered out in `compute_badges()`.

## Validation Rules

Shared between frontend and backend via `common/src/validation.rs`:

| Field | Rules |
|---|---|
| Password | 8+ chars, must contain: uppercase, lowercase, digit, special char |
| Email | Regex: `^[^\s@]+@[^\s@]+\.[^\s@]+$` |
| Username | 3-50 chars, `[a-zA-Z0-9_-]` only |

## SVG Compression

Dictionary-based compression in `common/src/svg_compress.rs` for compact SVG storage in the database.

- **Prefix**: `s1:` marks compressed SVGs (raw `<svg...` passes through unchanged)
- **24 token mappings**: longest-first replacement for safe decompression
- Examples: `~X` → `xmlns="http://www.w3.org/2000/svg"`, `~P` → `<path d="`
- `compress_svg()` is the inverse of `decompress_svg()` — both iterate the
  same `SVG_DICT`; `compress_svg` is used by the DSL diagram renderer to
  produce dictionary-friendly output, `decompress_svg` runs in the frontend
  (`crates/frontend/src/components/problem_card.rs:30`)
- Used by factory-generated question images (`question_image` column)

## Diagram Pipeline

`crates/dsl/src/diagram/` renders the optional `diagram:` block on a
`Variant` to a compressed SVG that lands in `ProblemOutput.question_image`.

- **Spec**: `docs/DSL_SPEC.md` §11 defines the YAML schema. `spec.rs` mirrors
  it as a typed `DiagramSpec` enum (replaces `serde_yaml::Value` in
  `crates/dsl/src/spec.rs`); the `Num` wrapper deserializes from either YAML
  number or string so callers can write `radius: 5` or `radius: r`.
- **Render**: `diagram::render(spec, &vars)` dispatches by type to one of
  `number_line`, `coordinate_plane`, `triangle`, `circle`, `polygon`,
  `function_graph`, `force_diagram`, `field`. Each module emits SVG fragments
  through `diagram::svg` builder helpers (`<line>`, `<text>`, `<circle>`,
  `<polyline>`, `<polygon>`, `<path>`) that match the `SVG_DICT` shortcuts.
- **Compress**: the wrapped `<svg>` string is passed through
  `locus_common::svg_compress::compress_svg` before storage.
- **Eval**: `diagram::eval::eval_num(expr, vars)` resolves any field to
  `f64` — accepts literals, variable references, or full expressions
  (parses via SymEngine, substitutes via `Expr::subs_float`).
- **Hook points**: `crates/dsl/src/lib.rs::generate_fast` and
  `crates/dsl/src/gpu/enumerator.rs` populate `question_image` from
  `variant.diagram` if present.
- **Circuit (§11.9)**: parsed but not rendered — pdflatex/circuitikz
  pipeline is future work; the renderer returns a clear error if invoked.
- **No external deps**: the renderer is pure Rust + SymEngine. No Typst,
  no LaTeX subprocess, no bundled packages.

## Caching Strategies

Three caching layers in `AppState`:

| Cache | Type | TTL | Invalidation |
|---|---|---|---|
| `TopicCache` | `Arc<RwLock<Vec<TopicResponse>>>` | Refreshed daily | Background task on startup |
| `LeaderboardCache` | `Arc<RwLock<HashMap<String, (Instant, Vec)>>>` | 5 minutes per topic | Time-based expiry check |
| `DailyPuzzleCache` | `Arc<RwLock<Option<(NaiveDate, DailyPuzzle, Problem)>>>` | Until date changes | Date mismatch check on read |

## Theme System

Dark mode toggle stored in `localStorage` (`theme` key). HTML root element gets `dark` class. CSS custom properties control all theme-aware colors:

- `--matrix-empty`, `--matrix-missed`, `--matrix-late`, `--matrix-same-day` for activity matrix
- Standard color variables for backgrounds, text, borders, etc.

## ELO System

Standard ELO with time bonus. K-factor = 32. Per-topic ratings stored in `user_topic_elo`.

- `expected_score(a, b) = 1 / (1 + 10^((b-a)/400))`
- Time multiplier: faster solving = up to 1.5x bonus (per-problem time limits override difficulty tiers)
- Default ELO: 1500
- Problem selection: weighted by proximity to user ELO + randomness

## Authentication

### Email/Password
1. Register with email + password (Argon2 hashed via `spawn_blocking`), `accepted_tos` must be true
2. Verification email sent via Resend (1-hour token, atomic verify via CTE)
3. Login returns JWT (HS256, 24-hour expiry, password verify via `spawn_blocking`) as httpOnly cookie
4. Password reset via email (30-minute token, atomic reset via CTE)

### OAuth (Google, GitHub)
1. Frontend opens popup to `/auth/oauth/{provider}`
2. Server generates CSRF state JWT (10-minute expiry), redirects to provider
3. Provider callback creates or links account, sets httpOnly cookie
4. Response sent to opener via `postMessage`
5. Email conflict: must log in first and link in Settings

### Cookie-Based Auth
- Primary: `locus_token` httpOnly cookie (SameSite=Lax, Path=/api, 24h expiry, Secure in prod)
- Fallback: `Authorization: Bearer <token>` header
- Middleware checks cookie first, then header
- Frontend sends `credentials: include` for cross-origin cookie delivery

## Build System

### WASM Target (frontend)

`common/build.rs` links pre-compiled WASM libraries from `symengine.js/dist/wasm-unknown/lib/`:
- `libsymengine.a`, `libc++.a`, `libc++abi.a`, `libc.a` (with dlmalloc stripped)
- Compiles `wasi_stub.c` for WASI shims

Frontend `main.rs` provides C allocator bridge (`malloc`/`free`/`calloc`/`realloc`) to prevent dual-allocator conflicts.

### Native Target (backend)

Links `/usr/local/lib/libsymengine.a` (static) + system `libgmp` and `libstdc++` (dynamic).

All FFI calls serialized through global `Mutex` since native SymEngine is not thread-safe.

### External JS Dependencies

Loaded via CDN in `frontend/index.html`:
- **jQuery** — required by MathQuill
- **MathQuill 0.10.1** — interactive math input (CSS + JS)
- **KaTeX** — LaTeX rendering
- **Fabric.js** — whiteboard canvas (lazy-loaded by whiteboard component)
