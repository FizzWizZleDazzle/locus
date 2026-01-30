# Database Schema

Complete documentation of the PostgreSQL database schema for Locus.

## Overview

**Database:** PostgreSQL 16
**Connection:** Port 5433 (mapped from container's 5432)
**User:** locus
**Password:** locus_dev_password (dev only)
**Database Name:** locus

---

## Tables

### users

Stores user account information.

**Schema:**
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Indexes:**
- Primary key on `id`
- Unique index on `username`
- Unique index on `email`

**Fields:**
| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Unique identifier for user |
| username | VARCHAR(50) | Display name (unique) |
| email | VARCHAR(255) | Email address (unique) |
| password_hash | VARCHAR(255) | Argon2id password hash |
| created_at | TIMESTAMPTZ | Account creation timestamp |

**Notes:**
- `password_hash` contains Argon2id hash (never store plaintext!)
- Old `elo` column removed in migration 003 (refactored to per-topic)

**Sample Row:**
```sql
id: 550e8400-e29b-41d4-a716-446655440000
username: "john_doe"
email: "john@example.com"
password_hash: "$argon2id$v=19$m=19456,t=2,p=1$..."
created_at: 2024-01-15 10:30:00+00
```

---

### user_topic_elo

Stores per-topic ELO ratings for each user.

**Schema:**
```sql
CREATE TABLE user_topic_elo (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    topic VARCHAR(50) NOT NULL,
    elo INTEGER NOT NULL DEFAULT 1500,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, topic)
);

CREATE INDEX idx_user_topic_elo_topic_elo ON user_topic_elo(topic, elo DESC);
```

**Indexes:**
- Composite primary key on `(user_id, topic)`
- Index on `(topic, elo DESC)` for leaderboard queries

**Fields:**
| Field | Type | Description |
|-------|------|-------------|
| user_id | UUID | Foreign key to users table |
| topic | VARCHAR(50) | Topic name (e.g., "Calculus") |
| elo | INTEGER | ELO rating for this topic |
| updated_at | TIMESTAMPTZ | Last rating update time |

**Valid Topics:**
- `Arithmetic`
- `Algebra1`
- `Geometry`
- `Algebra2`
- `Precalculus`
- `Calculus`
- `MultivariableCalculus`
- `LinearAlgebra`

**Default Behavior:**
- New users start with no rows (lazy initialization)
- Functions create entries with ELO 1500 on first access
- `ON DELETE CASCADE` removes all ratings when user is deleted

**Sample Rows:**
```sql
user_id: 550e8400-e29b-41d4-a716-446655440000, topic: "Calculus", elo: 1560, updated_at: 2024-01-20 14:22:00+00
user_id: 550e8400-e29b-41d4-a716-446655440000, topic: "Algebra1", elo: 1485, updated_at: 2024-01-18 09:15:00+00
```

---

### problems

Stores the problem bank with metadata.

**Schema:**
```sql
CREATE TABLE problems (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question_latex TEXT NOT NULL,
    answer_key TEXT NOT NULL,
    difficulty INTEGER NOT NULL,
    main_topic VARCHAR(50),
    subtopic VARCHAR(50),
    grading_mode VARCHAR(20) NOT NULL DEFAULT 'Equivalent'
);

CREATE INDEX idx_problems_main_topic ON problems(main_topic);
CREATE INDEX idx_problems_subtopic ON problems(subtopic);
```

**Indexes:**
- Primary key on `id`
- Index on `main_topic` for topic filtering
- Index on `subtopic` for subtopic filtering

**Fields:**
| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Unique problem identifier |
| question_latex | TEXT | LaTeX formatted question |
| answer_key | TEXT | Correct answer (normalized) |
| difficulty | INTEGER | ELO difficulty rating (800-2400) |
| main_topic | VARCHAR(50) | Main topic category |
| subtopic | VARCHAR(50) | Specific subtopic |
| grading_mode | VARCHAR(20) | How to grade the answer |

**Grading Modes:**
- `Equivalent` - Mathematical equivalence (default)
- `Factor` - Must be in factored form

**Sample Row:**
```sql
id: 660e8400-e29b-41d4-a716-446655440000
question_latex: "Factor the expression: $x^2 - 5x + 6$"
answer_key: "(x-2)(x-3)"
difficulty: 1400
main_topic: "Algebra1"
subtopic: "factoring_quadratics"
grading_mode: "Factor"
```

**Question LaTeX Examples:**
```latex
"Simplify: $3x + 2x - 4x + 7$"
"Find $\\frac{d}{dx}(x^3 + 2x^2)$"
"Calculate $\\det\\begin{pmatrix}2 & 3 \\\\ 1 & 4\\end{pmatrix}$"
```

---

### attempts

Records user submission history.

**Schema:**
```sql
CREATE TABLE attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    problem_id UUID NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
    user_input TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL,
    elo_before INTEGER NOT NULL,
    elo_after INTEGER NOT NULL,
    time_taken_ms INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    main_topic VARCHAR(50)
);

CREATE INDEX idx_attempts_user_id ON attempts(user_id);
CREATE INDEX idx_attempts_created_at ON attempts(created_at DESC);
```

**Indexes:**
- Primary key on `id`
- Index on `user_id` for user history queries
- Index on `created_at DESC` for recent attempts

**Fields:**
| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Unique attempt identifier |
| user_id | UUID | Foreign key to users |
| problem_id | UUID | Foreign key to problems |
| user_input | TEXT | User's submitted answer |
| is_correct | BOOLEAN | Whether answer was correct |
| elo_before | INTEGER | ELO rating before attempt |
| elo_after | INTEGER | ELO rating after attempt |
| time_taken_ms | INTEGER | Time taken in milliseconds |
| created_at | TIMESTAMPTZ | Submission timestamp |
| main_topic | VARCHAR(50) | Topic for this attempt |

**Sample Row:**
```sql
id: 770e8400-e29b-41d4-a716-446655440000
user_id: 550e8400-e29b-41d4-a716-446655440000
problem_id: 660e8400-e29b-41d4-a716-446655440000
user_input: "(x-2)(x-3)"
is_correct: true
elo_before: 1500
elo_after: 1515
time_taken_ms: 45000
created_at: 2024-01-20 14:22:00+00
main_topic: "Algebra1"
```

**Usage:**
- Track user performance history
- Calculate statistics (accuracy, average time)
- Audit trail for ELO changes
- Support for replay/review features

---

## PostgreSQL Functions

### get_user_elo

Returns the ELO rating for a user and topic. Creates entry with 1500 if not exists.

**Signature:**
```sql
CREATE OR REPLACE FUNCTION get_user_elo(
    p_user_id UUID,
    p_topic TEXT
) RETURNS INTEGER
```

**Logic:**
```sql
INSERT INTO user_topic_elo (user_id, topic, elo)
VALUES (p_user_id, p_topic, 1500)
ON CONFLICT (user_id, topic) DO NOTHING;

SELECT elo FROM user_topic_elo
WHERE user_id = p_user_id AND topic = p_topic;
```

**Usage:**
```sql
SELECT get_user_elo('550e8400-e29b-41d4-a716-446655440000', 'Calculus');
-- Returns: 1500 (if first time) or existing ELO
```

**File:** `crates/backend/migrations/003_topic_elo_table.sql:44`

---

### update_user_elo

Updates the ELO rating for a user and topic. Creates entry if not exists.

**Signature:**
```sql
CREATE OR REPLACE FUNCTION update_user_elo(
    p_user_id UUID,
    p_topic TEXT,
    p_new_elo INTEGER
) RETURNS VOID
```

**Logic:**
```sql
INSERT INTO user_topic_elo (user_id, topic, elo, updated_at)
VALUES (p_user_id, p_topic, p_new_elo, NOW())
ON CONFLICT (user_id, topic)
DO UPDATE SET elo = p_new_elo, updated_at = NOW();
```

**Usage:**
```sql
SELECT update_user_elo('550e8400-e29b-41d4-a716-446655440000', 'Calculus', 1560);
```

**File:** `crates/backend/migrations/003_topic_elo_table.sql:58`

---

## Migrations

Migrations are located in `crates/backend/migrations/` and run automatically on server startup.

### Migration History

| Version | File | Description |
|---------|------|-------------|
| 001 | 001_initial.sql | Initial schema (users, problems, attempts) |
| 003 | 003_topic_elo_table.sql | Per-topic ELO refactor |
| 004 | 004_seed_problems.sql | Seed 37 initial problems |

### Migration 001: Initial Schema

**File:** `crates/backend/migrations/001_initial.sql`

**Created:**
- `users` table with single `elo` column (later deprecated)
- `problems` table with single `topic` column (later deprecated)
- `attempts` table
- Basic indexes

### Migration 003: Topic ELO Refactor

**File:** `crates/backend/migrations/003_topic_elo_table.sql`

**Changes:**
- Created `user_topic_elo` table
- Added `main_topic` and `subtopic` columns to `problems`
- Created `get_user_elo()` function
- Created `update_user_elo()` function
- Added indexes for performance

**Migration Path:**
1. Create new table `user_topic_elo`
2. Alter `problems` table (add columns)
3. Create helper functions
4. Add indexes

### Migration 004: Seed Data

**File:** `crates/backend/migrations/004_seed_problems.sql`

**Data:**
- 13 Algebra1 problems
- 15 Calculus problems
- 5 LinearAlgebra problems
- 4 Algebra2 problems
- Total: 37 problems

---

## Query Patterns

### Get Random Problem by Topic and Subtopics

```sql
SELECT * FROM problems
WHERE main_topic = $1
  AND subtopic = ANY($2)
ORDER BY RANDOM()
LIMIT 1;
```

**File:** `crates/backend/src/models/problem.rs:68`

### Get User Leaderboard for Topic

```sql
SELECT
    username,
    elo
FROM user_topic_elo
JOIN users ON user_topic_elo.user_id = users.id
WHERE topic = $1
ORDER BY elo DESC
LIMIT 100;
```

**File:** `crates/backend/src/models/user.rs:108`

### Get User's All ELO Ratings

```sql
SELECT topic, elo
FROM user_topic_elo
WHERE user_id = $1;
```

**File:** `crates/backend/src/models/user.rs:90`

### Record Attempt

```sql
INSERT INTO attempts (
    user_id, problem_id, user_input, is_correct,
    elo_before, elo_after, time_taken_ms, main_topic
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
```

**File:** `crates/backend/src/models/attempt.rs:31`

---

## Database Configuration

### Development (docker-compose.yml)

```yaml
POSTGRES_USER: locus
POSTGRES_PASSWORD: locus_dev_password
POSTGRES_DB: locus
Port: 5433 (host) → 5432 (container)
```

### Connection String

```
DATABASE_URL=postgres://locus:locus_dev_password@localhost:5433/locus
```

### Connection Pool Settings

**File:** `crates/backend/src/db.rs`

```rust
PgPoolOptions::new()
    .max_connections(5)
    .connect(&config.database_url)
    .await
```

---

## Performance Considerations

### Indexes

**Current Indexes:**
- `users(username)` - UNIQUE
- `users(email)` - UNIQUE
- `user_topic_elo(topic, elo DESC)` - Leaderboard queries
- `problems(main_topic)` - Topic filtering
- `problems(subtopic)` - Subtopic filtering
- `attempts(user_id)` - User history
- `attempts(created_at DESC)` - Recent attempts

**Future Considerations:**
- Index on `attempts(user_id, created_at)` for paginated history
- Partial index on `attempts(is_correct)` for accuracy stats
- Index on `problems(difficulty)` for difficulty-based selection

### Query Optimization

**Random Selection:**
- Current: `ORDER BY RANDOM()` (simple but slower for large datasets)
- Future: Pre-select IDs, then random from subset

**Leaderboard:**
- Indexed on `(topic, elo DESC)` for fast sorting
- Limited to 100 rows (no pagination needed currently)

---

## Backup and Recovery

### Development

```bash
# Backup
docker exec locus-db pg_dump -U locus locus > backup.sql

# Restore
docker exec -i locus-db psql -U locus locus < backup.sql
```

### Production

- Use managed PostgreSQL service (AWS RDS, GCP Cloud SQL)
- Enable automated backups
- Set retention period (7-30 days)
- Test restore procedure regularly

---

## Security

### Password Storage

- **Never** store passwords in plaintext
- Use Argon2id with automatic salt generation
- Password hashes are 255 characters max

### SQL Injection Prevention

- All queries use SQLx parameterized queries
- No string concatenation for SQL
- Compile-time checked queries with `query!` macro

### UUID Primary Keys

- UUIDs instead of sequential integers
- Harder to enumerate users/problems
- No information leakage about table size

---

## Testing

### Database Tests

Run database tests:
```bash
cargo test --package locus-backend
```

**Test Database:**
- Use separate test database
- Reset schema before each test
- Mock database for unit tests

---

## Schema Evolution

### Adding New Topics

1. Add to `MainTopic` enum in `crates/common/src/lib.rs`
2. Add subtopics to `subtopics()` method
3. No database migration needed (user_topic_elo is dynamic)
4. Add seed problems in new migration

### Adding New Problem Fields

1. Add column to `problems` table in new migration
2. Update `Problem` struct in `crates/backend/src/models/problem.rs`
3. Update `ProblemResponse` in `crates/common/src/lib.rs`
4. Update frontend components

---

## Data Integrity

### Foreign Key Constraints

- `user_topic_elo.user_id` → `users.id` (CASCADE DELETE)
- `attempts.user_id` → `users.id` (CASCADE DELETE)
- `attempts.problem_id` → `problems.id` (CASCADE DELETE)

**Behavior:**
- Deleting a user removes all their ELO ratings and attempts
- Deleting a problem removes all attempts for that problem

### Unique Constraints

- `users.username` - No duplicate usernames
- `users.email` - No duplicate emails
- `(user_id, topic)` - One ELO per user per topic

---

## Future Enhancements

- **Partitioning:** Partition `attempts` by date for large datasets
- **Caching:** Redis cache for leaderboards
- **Read Replicas:** Scale read queries
- **Full-Text Search:** Search problems by content
- **Soft Deletes:** Archive instead of deleting data
