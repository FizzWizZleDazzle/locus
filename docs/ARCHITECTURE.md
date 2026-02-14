# Architecture Overview

This document describes the high-level architecture of the Locus platform.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Browser                             │
│  ┌───────────────────────────────────────────────────────┐ │
│  │          Leptos Frontend (WASM)                       │ │
│  │                                                        │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────┐       │ │
│  │  │  Pages   │  │Components│  │ Client-side  │       │ │
│  │  │          │  │          │  │   Grader     │       │ │
│  │  └──────────┘  └──────────┘  └──────────────┘       │ │
│  │                                                        │ │
│  │  ┌──────────────────────────────────────────┐        │ │
│  │  │        API Client (gloo-net)              │        │ │
│  │  │    JWT in localStorage                    │        │ │
│  │  └──────────────────────────────────────────┘        │ │
│  └───────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ HTTP/JSON
                            │
┌─────────────────────────────────────────────────────────────┐
│                    Axum Backend (Rust)                      │
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │ Auth Routes  │  │Problem Routes│  │Submit Routes │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│                                                             │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │ JWT Middleware│  │   Grader    │  │ ELO Engine   │    │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│                                                             │
│  ┌────────────────────────────────────────────────────┐   │
│  │              SQLx Database Layer                   │   │
│  └────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ SQL/TCP
                            │
┌─────────────────────────────────────────────────────────────┐
│                  PostgreSQL 16 Database                     │
│                                                             │
│  ┌──────────┐  ┌──────────────┐  ┌──────────┐            │
│  │  users   │  │user_topic_elo│  │ problems │            │
│  └──────────┘  └──────────────┘  └──────────┘            │
│                                                             │
│  ┌──────────┐  ┌────────────────────────────────┐        │
│  │ attempts │  │ PostgreSQL Functions           │        │
│  └──────────┘  │ - get_user_elo()               │        │
│                 │ - update_user_elo()            │        │
│                 └────────────────────────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

## Component Breakdown

### Frontend (Leptos WASM)

**Responsibilities:**
- User interface rendering
- Client-side routing
- Practice mode grading (instant feedback)
- Math input preprocessing
- Auth token management
- KaTeX rendering for LaTeX

**Key Files:**
- `crates/frontend/src/main.rs` - App entry point, router setup
- `crates/frontend/src/api.rs` - HTTP client, localStorage auth
- `crates/frontend/src/grader.rs` - Client-side answer validation
- `crates/frontend/src/pages/` - Route handlers
- `crates/frontend/src/components/` - Reusable UI components

**Technologies:**
- Leptos (CSR mode) - Reactive UI framework
- gloo-net - HTTP client
- gloo-storage - localStorage access
- KaTeX - Math rendering
- Tailwind CSS - Styling

### Backend (Axum)

**Responsibilities:**
- REST API endpoints
- JWT authentication
- Server-side answer grading
- ELO calculation and updates
- Database operations
- Problem selection with filters

**Key Files:**
- `crates/backend/src/main.rs` - Server setup, middleware
- `crates/backend/src/api/` - Route handlers
- `crates/backend/src/auth/` - JWT and middleware
- `crates/backend/src/models/` - Database models
- `crates/backend/src/elo.rs` - ELO algorithm
- `crates/backend/src/grader.rs` - Answer validation

**Technologies:**
- Axum - Web framework
- Tokio - Async runtime
- SQLx - Database access
- jsonwebtoken - JWT handling
- Argon2 - Password hashing

### Database (PostgreSQL)

**Responsibilities:**
- User account storage
- Per-topic ELO ratings
- Problem bank storage
- Attempt history tracking
- Leaderboard queries
- OAuth account linking
- Email verification

PostgreSQL 16 with 7 core tables: users, user_topic_elo, problems, attempts, oauth_accounts, email_verifications, password_resets.

See [Database Schema](DATABASE.md) for complete table definitions, indexes, and migrations.

### Common Crate

**Responsibilities:**
- Shared type definitions
- API request/response models
- Topic and subtopic enums
- Error types
- Serialization/deserialization

**Key Files:**
- `crates/common/src/lib.rs` - All shared types

## Data Flow

### Practice Mode Flow

```
User → Frontend
  ↓
Select Topic/Subtopics
  ↓
Frontend → Backend: GET /api/problem?main_topic=X&subtopics=Y,Z
  ↓
Backend → Database: SELECT random problem
  ↓
Database → Backend: Problem with answer
  ↓
Backend → Frontend: Problem + answer_key
  ↓
User submits answer
  ↓
Frontend grader validates (client-side)
  ↓
Instant feedback (no DB write)
```

### Ranked Mode Flow

```
User → Frontend (must be logged in)
  ↓
Select Topic/Subtopics
  ↓
Frontend → Backend: GET /api/problem?main_topic=X&subtopics=Y,Z (with JWT)
  ↓
Backend → Database: SELECT random problem
  ↓
Database → Backend: Problem WITHOUT answer
  ↓
Backend → Frontend: Problem (no answer)
  ↓
User submits answer
  ↓
Frontend → Backend: POST /api/submit {problem_id, user_input}
  ↓
Backend grader validates
  ↓
Backend calculates ELO change
  ↓
Backend → Database: INSERT attempt, UPDATE user_topic_elo
  ↓
Database → Backend: Confirmation
  ↓
Backend → Frontend: {is_correct, elo_change, new_elo}
  ↓
Frontend displays result
```

### Authentication Flow

```
User → Frontend: Register/Login form
  ↓
Frontend → Backend: POST /api/auth/register or /api/auth/login
  ↓
Backend validates credentials
  ↓
Backend → Database: Create user or verify password
  ↓
Backend generates JWT token
  ↓
Backend → Frontend: {token, username}
  ↓
Frontend stores in localStorage
  ↓
All subsequent requests include: Authorization: Bearer {token}
```

## ELO System Architecture

Each user maintains separate ELO ratings for 8 math topics (Arithmetic, Algebra 1, Geometry, Algebra 2, Precalculus, Calculus, Multivariable Calculus, Linear Algebra).

- **Calculation:** See [ELO Implementation](BACKEND.md#elo-system) for formulas and algorithm
- **Storage:** See [user_topic_elo table](DATABASE.md#user_topic_elo) for schema and PostgreSQL functions
- **K-factor:** 32 (per topic)
- **Time bonus:** 1.0x-1.5x multiplier for fast solves (no penalties)
- **opponent_elo:** Uses problem.difficulty for calculation

## Grading Architecture

### MVP Approach (Current)

**Frontend:**
- String normalization (trim, lowercase)
- Implicit multiplication insertion (2x → 2*x)
- Preview shows preprocessed input

**Backend:**
- String normalization (trim, lowercase)
- Exact match with answer_key

### Future Enhancement

**SymEngine WASM Integration:**
- Parse user input to symbolic expression
- Parse answer key to symbolic expression
- Check mathematical equivalence
- Support for:
  - Algebraic simplification
  - Expression expansion
  - Factorization verification
  - Derivative computation

## Security Architecture

- **Authentication:** JWT tokens (HS256, 24h expiry) + Argon2id password hashing
  - See [Authentication Guide](AUTHENTICATION.md) for complete flows
  - See [JWT Implementation](BACKEND.md#authentication) for code details

- **OAuth:** Google and GitHub providers with account linking
  - See [OAuth Flows](AUTHENTICATION.md#oauth-flows) for diagrams

- **Email Verification:** Required for new accounts (1h token expiry)
  - See [Email Verification](AUTHENTICATION.md#email-verification) for flow

- **Password Reset:** Secure token-based reset with 1h expiry
  - See [Password Reset](AUTHENTICATION.md#password-reset) for flow

- **Rate Limiting:** Per-endpoint limits via tower-governor
  - Auth: 5 req/15min (register), 10 req/15min (login)
  - See [Rate Limiting](BACKEND.md#rate-limiting) for configuration

- **CORS:** Configured per environment (dev: localhost:8080, prod: same-origin)

## Scalability Considerations

### Current Architecture

- Single server deployment
- Connection pooling for database
- Stateless API (JWT-based auth)

### Future Enhancements

- Horizontal scaling: Multiple backend instances
- Load balancer: Distribute traffic
- Caching: Redis for leaderboards
- CDN: Static assets and WASM bundle
- Read replicas: Database read scaling

## Deployment Architecture

### Development

```
localhost:8080 (Trunk dev server)
  ↓ Proxies API requests to
localhost:3000 (Axum backend)
  ↓
localhost:5433 (Docker PostgreSQL)
```

### Production (Recommended)

```
HTTPS Load Balancer
  ↓
Backend instances (Axum)
  ↓
PostgreSQL (managed service or replica set)

Static files (WASM, HTML, CSS) → CDN
```

## Module Dependencies

```
┌─────────────────────────────────────────────┐
│                  common                     │
│  (Types, API models, enums)                 │
└─────────────────────────────────────────────┘
         ↑                    ↑
         │                    │
    ┌────────┐           ┌─────────┐
    │frontend│           │ backend │
    └────────┘           └─────────┘
         │                    │
         └────────────────────┘
              (Network)
```

No circular dependencies - common is pure data types.

## Error Handling

### Frontend
- API errors displayed to user
- Network errors trigger retry logic
- Invalid input prevented at UI level

### Backend
- Custom `ApiError` type
- Mapped to HTTP status codes
- Detailed error messages in development
- Generic messages in production

### Database
- SQLx compile-time checked queries
- Connection pool error handling
- Transaction rollback on failure

## Monitoring and Observability

### Logging

**Backend:**
- `tracing` crate for structured logging
- Log levels: ERROR, WARN, INFO, DEBUG, TRACE
- Environment-based log filtering

**Frontend:**
- console.log for development
- Error boundary for panic handling

### Health Checks

**Endpoint:** `GET /api/health`
- Returns 200 OK with basic info
- Can be extended to check database connectivity

## Testing Strategy

### Unit Tests
- ELO calculation (`crates/backend/src/elo.rs`)
- Grading logic (`crates/backend/src/grader.rs`)
- Input preprocessing (`crates/frontend/src/grader.rs`)

### Integration Tests
- API endpoint tests
- Database operations
- Auth flow

### Future Testing
- E2E tests with browser automation
- Load testing for scalability
- Security testing (SQL injection, XSS)
