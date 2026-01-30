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

**Key Features:**
- UUID primary keys for security
- Per-topic ELO in separate table
- PostgreSQL functions for ELO management
- Indexes on frequently queried columns
- Migrations for schema versioning

**Tables:**
- `users` - User accounts
- `user_topic_elo` - Per-topic ratings (8 topics)
- `problems` - Problem bank with metadata
- `attempts` - User submission history

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

### Per-Topic Ratings

Each user has 8 separate ELO ratings (one per topic):
- Arithmetic
- Algebra 1
- Geometry
- Algebra 2
- Precalculus
- Calculus
- Multivariable Calculus
- Linear Algebra

### Rating Storage

**Table: `user_topic_elo`**
```sql
(user_id, topic) PRIMARY KEY
elo INTEGER DEFAULT 1500
updated_at TIMESTAMPTZ
```

**PostgreSQL Functions:**
```sql
get_user_elo(user_id UUID, topic TEXT) RETURNS INTEGER
  -- Returns ELO for topic, creates with 1500 if not exists

update_user_elo(user_id UUID, topic TEXT, new_elo INTEGER) RETURNS VOID
  -- Updates ELO for topic, creates if not exists
```

### ELO Calculation

```rust
// Expected score
E = 1 / (1 + 10^((opponent_elo - player_elo) / 400))

// New rating
new_elo = old_elo + K * (actual_score - expected_score)

// K-factor: 32 (standard for online games)
// actual_score: 1.0 (correct) or 0.0 (incorrect)
// opponent_elo: problem.difficulty
```

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

### Authentication

**JWT Tokens:**
- Signed with HS256
- Contains: user_id, username, exp
- Stored in localStorage (frontend)
- Sent in Authorization header
- Expires after 24 hours (configurable)

**Password Security:**
- Hashed with Argon2id
- Salt automatically generated
- Never stored or transmitted in plaintext

### Authorization

**Protected Routes:**
- `/api/submit` - Requires valid JWT
- `/api/user/me` - Requires valid JWT
- Ranked mode problems - Requires JWT to hide answer

**Public Routes:**
- `/api/auth/register`
- `/api/auth/login`
- `/api/problem` (practice mode)
- `/api/leaderboard`

### CORS

**Configuration:**
- Frontend dev server: http://localhost:8080
- Backend API: http://localhost:3000
- CORS enabled for cross-origin requests in development
- Production should use same origin or proper CORS config

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
