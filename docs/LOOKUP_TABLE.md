# Lookup Table

Quick reference for finding files, features, and implementations in the Locus codebase.

## Table of Contents

- [By Feature](#by-feature)
- [By File Type](#by-file-type)
- [By Component](#by-component)
- [By Topic](#by-topic)
- [Quick Reference](#quick-reference)

---

## By Feature

### Authentication & Authorization

| Feature | Location | Line |
|---------|----------|------|
| JWT token creation | `crates/backend/src/auth/jwt.rs` | 15 |
| JWT token verification | `crates/backend/src/auth/jwt.rs` | 35 |
| Auth middleware extractor | `crates/backend/src/auth/middleware.rs` | 12 |
| Password hashing (Argon2) | `crates/backend/src/api/auth.rs` | 25 |
| Password verification | `crates/backend/src/api/auth.rs` | 55 |
| Register endpoint | `crates/backend/src/api/auth.rs` | 15 |
| Login endpoint | `crates/backend/src/api/auth.rs` | 45 |
| Get current user endpoint | `crates/backend/src/api/auth.rs` | 75 |
| Frontend auth context | `crates/frontend/src/main.rs` | 30 |
| Frontend login page | `crates/frontend/src/pages/login.rs` | 1 |
| Frontend register page | `crates/frontend/src/pages/register.rs` | 1 |
| LocalStorage token management | `crates/frontend/src/api.rs` | 20 |

### Problem Management

| Feature | Location | Line |
|---------|----------|------|
| Get random problem | `crates/backend/src/models/problem.rs` | 40 |
| Problem retrieval endpoint | `crates/backend/src/api/problems.rs` | 18 |
| Problem database model | `crates/backend/src/models/problem.rs` | 10 |
| Problem table schema | `crates/backend/migrations/001_initial.sql` | 20 |
| Topic enum definition | `crates/common/src/lib.rs` | 15 |
| Subtopics definition | `crates/common/src/lib.rs` | 50 |
| Problem generation script | `content-gen/generate.py` | 1 |
| Seed problems migration | `crates/backend/migrations/004_seed_problems.sql` | 1 |

### ELO System

| Feature | Location | Line |
|---------|----------|------|
| ELO calculation engine | `crates/backend/src/elo.rs` | 5 |
| Expected score formula | `crates/backend/src/elo.rs` | 10 |
| ELO change calculation | `crates/backend/src/elo.rs` | 20 |
| Per-topic ELO table | `crates/backend/migrations/003_topic_elo_table.sql` | 10 |
| Get user ELO function (SQL) | `crates/backend/migrations/003_topic_elo_table.sql` | 44 |
| Update user ELO function (SQL) | `crates/backend/migrations/003_topic_elo_table.sql` | 58 |
| Get ELO for topic (Rust) | `crates/backend/src/models/user.rs` | 60 |
| Update ELO for topic (Rust) | `crates/backend/src/models/user.rs` | 75 |
| ELO tests | `crates/backend/src/elo.rs` | 35 |

### Grading System

| Feature | Location | Line |
|---------|----------|------|
| Server-side grading | `crates/backend/src/grader.rs` | 5 |
| Client-side grading | `crates/frontend/src/grader.rs` | 80 |
| Input preprocessing | `crates/frontend/src/grader.rs` | 23 |
| Answer normalization | `crates/backend/src/grader.rs` | 15 |
| Submit answer endpoint | `crates/backend/src/api/submit.rs` | 20 |
| Grading mode enum | `crates/common/src/lib.rs` | 120 |
| SymEngine bindings (future) | `crates/frontend/src/symengine.rs` | 1 |

### Practice Mode

| Feature | Location | Line |
|---------|----------|------|
| Practice page component | `crates/frontend/src/pages/practice.rs` | 1 |
| Topic selector component | `crates/frontend/src/components/topic_selector.rs` | 1 |
| Client-side grader | `crates/frontend/src/grader.rs` | 80 |
| Math input component | `crates/frontend/src/components/math_input.rs` | 1 |
| Problem card component | `crates/frontend/src/components/problem_card.rs` | 1 |

### Ranked Mode

| Feature | Location | Line |
|---------|----------|------|
| Ranked page component | `crates/frontend/src/pages/ranked.rs` | 1 |
| Submit answer API call | `crates/frontend/src/api.rs` | 80 |
| Submit endpoint handler | `crates/backend/src/api/submit.rs` | 20 |
| Attempt recording | `crates/backend/src/models/attempt.rs` | 30 |
| Attempts table schema | `crates/backend/migrations/001_initial.sql` | 40 |

### Leaderboard

| Feature | Location | Line |
|---------|----------|------|
| Leaderboard page | `crates/frontend/src/pages/leaderboard.rs` | 1 |
| Leaderboard endpoint | `crates/backend/src/api/leaderboard.rs` | 15 |
| Leaderboard SQL query | `crates/backend/src/models/user.rs` | 108 |
| Get leaderboard API call | `crates/frontend/src/api.rs` | 100 |

### Database

| Feature | Location | Line |
|---------|----------|------|
| Users table | `crates/backend/migrations/001_initial.sql` | 5 |
| User_topic_elo table | `crates/backend/migrations/003_topic_elo_table.sql` | 10 |
| Problems table | `crates/backend/migrations/001_initial.sql` | 20 |
| Attempts table | `crates/backend/migrations/001_initial.sql` | 40 |
| Database connection pool | `crates/backend/src/db.rs` | 5 |
| User model | `crates/backend/src/models/user.rs` | 10 |
| Problem model | `crates/backend/src/models/problem.rs` | 10 |
| Attempt model | `crates/backend/src/models/attempt.rs` | 10 |

---

## By File Type

### Rust Source Files

#### Backend

| File | Purpose | Key Functions |
|------|---------|---------------|
| `crates/backend/src/main.rs` | Server entry point | main(), create_app() |
| `crates/backend/src/config.rs` | Environment config | Config::from_env() |
| `crates/backend/src/db.rs` | Database pool | create_pool() |
| `crates/backend/src/elo.rs` | ELO calculations | calculate_expected_score(), calculate_elo_change() |
| `crates/backend/src/grader.rs` | Answer grading | grade_answer(), normalize() |
| `crates/backend/src/api/mod.rs` | Router & errors | create_router(), ApiError |
| `crates/backend/src/api/auth.rs` | Auth endpoints | register(), login(), get_me() |
| `crates/backend/src/api/problems.rs` | Problem endpoints | get_problem() |
| `crates/backend/src/api/submit.rs` | Submit endpoint | submit_answer() |
| `crates/backend/src/api/leaderboard.rs` | Leaderboard endpoint | get_leaderboard() |
| `crates/backend/src/auth/jwt.rs` | JWT operations | create_token(), verify_token() |
| `crates/backend/src/auth/middleware.rs` | Auth middleware | AuthUser extractor |
| `crates/backend/src/models/user.rs` | User database ops | create(), find_by_email(), get_elo_for_topic() |
| `crates/backend/src/models/problem.rs` | Problem database ops | get_random(), find_by_id() |
| `crates/backend/src/models/attempt.rs` | Attempt database ops | create(), get_user_attempts() |

#### Frontend

| File | Purpose | Key Functions |
|------|---------|---------------|
| `crates/frontend/src/main.rs` | App entry, router | App(), main() |
| `crates/frontend/src/api.rs` | HTTP client | register(), login(), get_problem(), submit_answer() |
| `crates/frontend/src/grader.rs` | Client grading | preprocess_input(), grade_answer() |
| `crates/frontend/src/symengine.rs` | SymEngine FFI | parse(), equals() (scaffolding) |
| `crates/frontend/src/pages/home.rs` | Home page | Home() |
| `crates/frontend/src/pages/practice.rs` | Practice page | Practice() |
| `crates/frontend/src/pages/ranked.rs` | Ranked page | Ranked() |
| `crates/frontend/src/pages/leaderboard.rs` | Leaderboard page | Leaderboard() |
| `crates/frontend/src/pages/login.rs` | Login page | Login() |
| `crates/frontend/src/pages/register.rs` | Register page | Register() |
| `crates/frontend/src/components/navbar.rs` | Navigation bar | Navbar() |
| `crates/frontend/src/components/math_input.rs` | Math input field | MathInput() |
| `crates/frontend/src/components/problem_card.rs` | Problem display | ProblemCard() |
| `crates/frontend/src/components/topic_selector.rs` | Topic selection | TopicSelector() |

#### Common

| File | Purpose | Key Types |
|------|---------|-----------|
| `crates/common/src/lib.rs` | Shared types | MainTopic, RegisterRequest, ProblemResponse, etc. |

### SQL Migration Files

| File | Purpose |
|------|---------|
| `crates/backend/migrations/001_initial.sql` | Initial schema (users, problems, attempts) |
| `crates/backend/migrations/003_topic_elo_table.sql` | Per-topic ELO refactor |
| `crates/backend/migrations/004_seed_problems.sql` | Seed 37 initial problems |

### Configuration Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Workspace configuration |
| `crates/backend/Cargo.toml` | Backend dependencies |
| `crates/frontend/Cargo.toml` | Frontend dependencies |
| `crates/common/Cargo.toml` | Common dependencies |
| `crates/frontend/Trunk.toml` | Trunk build config |
| `docker-compose.yml` | PostgreSQL container |
| `.env.example` | Environment template |
| `.gitignore` | Git ignore rules |

### Scripts

| File | Purpose |
|------|---------|
| `dev.sh` | Development startup script |
| `content-gen/generate.py` | Problem generation |

### Documentation

| File | Purpose |
|------|---------|
| `README.md` | Project overview |
| `DOCUMENTATION.md` | Documentation index |
| `docs/ARCHITECTURE.md` | System architecture |
| `docs/API.md` | API reference |
| `docs/DATABASE.md` | Database schema |
| `docs/FRONTEND.md` | Frontend guide |
| `docs/BACKEND.md` | Backend guide |
| `docs/DEVELOPMENT.md` | Dev setup guide |
| `docs/DEPLOYMENT.md` | Deployment guide |
| `docs/LOOKUP_TABLE.md` | This file |

---

## By Component

### API Endpoints

| Endpoint | Handler | Model | Line |
|----------|---------|-------|------|
| POST /api/auth/register | `crates/backend/src/api/auth.rs` | User | 15 |
| POST /api/auth/login | `crates/backend/src/api/auth.rs` | User | 45 |
| GET /api/user/me | `crates/backend/src/api/auth.rs` | User | 75 |
| GET /api/problem | `crates/backend/src/api/problems.rs` | Problem | 18 |
| POST /api/submit | `crates/backend/src/api/submit.rs` | Problem, Attempt | 20 |
| GET /api/leaderboard | `crates/backend/src/api/leaderboard.rs` | User | 15 |
| GET /api/health | `crates/backend/src/api/mod.rs` | N/A | 50 |

### Frontend Routes

| Route | Component | Auth Required |
|-------|-----------|---------------|
| / | `crates/frontend/src/pages/home.rs` | No |
| /practice | `crates/frontend/src/pages/practice.rs` | No |
| /ranked | `crates/frontend/src/pages/ranked.rs` | Yes |
| /leaderboard | `crates/frontend/src/pages/leaderboard.rs` | No |
| /login | `crates/frontend/src/pages/login.rs` | No |
| /register | `crates/frontend/src/pages/register.rs` | No |

### UI Components

| Component | File | Props |
|-----------|------|-------|
| App | `crates/frontend/src/main.rs` | None |
| Navbar | `crates/frontend/src/components/navbar.rs` | None (uses context) |
| MathInput | `crates/frontend/src/components/math_input.rs` | value, on_submit |
| ProblemCard | `crates/frontend/src/components/problem_card.rs` | problem, show_answer |
| TopicSelector | `crates/frontend/src/components/topic_selector.rs` | selected_topic, selected_subtopics |

---

## By Topic

### Math Topics

| Topic | Subtopics | Location |
|-------|-----------|----------|
| Arithmetic | basic_operations, fractions, decimals, percentages, order_of_operations | `crates/common/src/lib.rs:50` |
| Algebra1 | linear_equations, inequalities, graphing_lines, systems_of_equations, polynomials, factoring_quadratics, exponents, radicals | `crates/common/src/lib.rs:60` |
| Geometry | angles, triangles, quadrilaterals, circles, area_perimeter, volume_surface_area, pythagorean_theorem | `crates/common/src/lib.rs:70` |
| Algebra2 | quadratic_functions, complex_numbers, polynomial_functions, rational_functions, exponential_functions, logarithmic_functions | `crates/common/src/lib.rs:80` |
| Precalculus | trigonometry, polar_coordinates, sequences_series, limits, conic_sections | `crates/common/src/lib.rs:90` |
| Calculus | limits_continuity, derivatives, applications_of_derivatives, integrals, applications_of_integrals | `crates/common/src/lib.rs:100` |
| MultivariableCalculus | partial_derivatives, multiple_integrals, vector_calculus, line_integrals, surface_integrals | `crates/common/src/lib.rs:110` |
| LinearAlgebra | matrices, determinants, vector_spaces, eigenvalues_eigenvectors, linear_transformations | `crates/common/src/lib.rs:120` |

### Problem Types by Generator

| Type | Generator Function | Location |
|------|-------------------|----------|
| Algebra - Combine like terms | `generate_algebra_problems()` | `content-gen/generate.py:25` |
| Algebra - Expand binomials | `generate_algebra_problems()` | `content-gen/generate.py:30` |
| Algebra - Factor quadratics | `generate_algebra_problems()` | `content-gen/generate.py:35` |
| Calculus - Power rule | `generate_calculus_problems()` | `content-gen/generate.py:60` |
| Calculus - Derivatives | `generate_calculus_problems()` | `content-gen/generate.py:65` |
| Linear Algebra - Determinants | `generate_linear_algebra_problems()` | `content-gen/generate.py:100` |

---

## Quick Reference

### Common Tasks

| Task | Command/Location |
|------|------------------|
| Start dev environment | `./dev.sh` |
| Start backend only | `cargo run -p locus-backend` |
| Start frontend only | `cd crates/frontend && trunk serve` |
| Run tests | `cargo test` |
| Format code | `cargo fmt` |
| Check code | `cargo clippy` |
| Connect to database | `psql postgres://locus:locus_dev_password@localhost:5433/locus` |
| Generate problems | `cd content-gen && python generate.py --topic calculus --count 50` |
| Build for production | `cargo build --release -p locus-backend` |
| Build frontend for production | `cd crates/frontend && trunk build --release` |

### Important Constants

| Constant | Value | Location |
|----------|-------|----------|
| ELO K-factor | 32 | `crates/backend/src/elo.rs:5` |
| Default ELO | 1500 | `crates/backend/migrations/003_topic_elo_table.sql:14` |
| Default JWT expiry | 24 hours | `.env.example:10` |
| Backend port | 3000 | `.env.example:6` |
| Frontend port | 8080 | `crates/frontend/Trunk.toml:7` |
| Database port | 5433 | `docker-compose.yml:10` |
| Max database connections | 5 | `crates/backend/src/db.rs:8` |

### Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| DATABASE_URL | PostgreSQL connection | Yes | - |
| JWT_SECRET | JWT signing secret | Yes | - |
| HOST | Bind address | No | 0.0.0.0 |
| PORT | Server port | No | 3000 |
| JWT_EXPIRY_HOURS | Token lifetime | No | 24 |
| RUST_LOG | Log level | No | info |

### Database Tables

| Table | Primary Key | Foreign Keys | Indexes |
|-------|-------------|--------------|---------|
| users | id (UUID) | - | username, email |
| user_topic_elo | (user_id, topic) | user_id → users.id | (topic, elo DESC) |
| problems | id (UUID) | - | main_topic, subtopic |
| attempts | id (UUID) | user_id → users.id, problem_id → problems.id | user_id, created_at |

### API Response Codes

| Code | Meaning | Common Causes |
|------|---------|---------------|
| 200 | Success | Request completed successfully |
| 400 | Bad Request | Invalid input, validation failed |
| 401 | Unauthorized | Missing or invalid JWT token |
| 404 | Not Found | Resource doesn't exist |
| 409 | Conflict | Duplicate username/email |
| 500 | Internal Server Error | Database error, unexpected failure |

---

## Search Tips

### Finding Specific Code

**By feature name:**
```bash
grep -r "practice" crates/frontend/src/
```

**By function name:**
```bash
grep -r "calculate_elo" crates/backend/src/
```

**By database table:**
```bash
grep -r "user_topic_elo" crates/backend/
```

**By API endpoint:**
```bash
grep -r "/api/submit" crates/
```

### Finding Tests

```bash
# All tests
find . -name "*.rs" -exec grep -l "#\[test\]" {} \;

# Specific test module
grep -A 20 "mod tests" crates/backend/src/elo.rs
```

### Finding Dependencies

```bash
# Backend dependencies
cat crates/backend/Cargo.toml | grep "^[a-z]"

# Frontend dependencies
cat crates/frontend/Cargo.toml | grep "^[a-z]"
```

---

## Glossary

| Term | Definition | Location |
|------|------------|----------|
| ELO | Rating system for skill level | `crates/backend/src/elo.rs` |
| JWT | JSON Web Token for authentication | `crates/backend/src/auth/jwt.rs` |
| WASM | WebAssembly (frontend compilation target) | `crates/frontend/` |
| SQLx | Rust SQL toolkit | `crates/backend/src/db.rs` |
| Leptos | Reactive UI framework | `crates/frontend/src/main.rs` |
| Axum | Web framework | `crates/backend/src/main.rs` |
| Trunk | WASM build tool | `crates/frontend/Trunk.toml` |
| KaTeX | LaTeX math rendering | `crates/frontend/index.html` |
| SymEngine | Symbolic math engine | `symengine.js/` |
| Practice Mode | Unranked mode with instant feedback | `crates/frontend/src/pages/practice.rs` |
| Ranked Mode | Competitive mode with ELO changes | `crates/frontend/src/pages/ranked.rs` |
| Grading Mode | How answers are validated | `crates/common/src/lib.rs:120` |
| Topic | Main math category | `crates/common/src/lib.rs:15` |
| Subtopic | Specific skill within topic | `crates/common/src/lib.rs:50` |

---

## Index by Line Count

### Largest Files

| File | Lines | Purpose |
|------|-------|---------|
| `docs/BACKEND.md` | 800+ | Backend documentation |
| `docs/FRONTEND.md` | 700+ | Frontend documentation |
| `docs/DATABASE.md` | 600+ | Database documentation |
| `docs/DEPLOYMENT.md` | 600+ | Deployment guide |
| `docs/DEVELOPMENT.md` | 500+ | Development guide |
| `docs/API.md` | 500+ | API reference |
| `crates/common/src/lib.rs` | 300+ | Shared types |
| `crates/backend/migrations/004_seed_problems.sql` | 200+ | Seed data |

### Most Complex Modules

| Module | Files | Complexity |
|--------|-------|------------|
| Backend API | 5 files | High |
| Frontend Pages | 6 files | Medium |
| Database Models | 3 files | Medium |
| Auth System | 2 files | Medium |
| Components | 4 files | Low |

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2024-01 | Initial release with per-topic ELO |
| 003 | Migration | Topic ELO refactor |
| 004 | Migration | Seed 37 problems |

---

This lookup table is comprehensive and should help you navigate the codebase efficiently. For detailed information about any component, refer to the specific documentation file linked throughout this document.
