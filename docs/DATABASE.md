# Database Schema

PostgreSQL 16. Migrations managed by SQLx in `crates/backend/migrations/`.

## Tables

### `users`

| Column | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `username` | VARCHAR(50) | UNIQUE, NOT NULL |
| `email` | VARCHAR(255) | UNIQUE, NOT NULL |
| `password_hash` | VARCHAR(255) | nullable (OAuth-only users) |
| `email_verified` | BOOLEAN | default FALSE |
| `email_verified_at` | TIMESTAMPTZ | nullable |
| `current_streak` | INT | default 0 (global daily streak) |
| `last_active_date` | DATE | nullable |
| `created_at` | TIMESTAMPTZ | default NOW() |

### `user_topic_elo`

Per-topic ELO ratings with streak and peak tracking.

| Column | Type | Constraints |
|---|---|---|
| `user_id` | UUID | PK (composite), FK -> users ON DELETE CASCADE |
| `topic` | VARCHAR(50) | PK (composite) |
| `elo` | INT | default 1500 |
| `peak_elo` | INT | default 1500 |
| `topic_streak` | INT | default 0 |
| `peak_topic_streak` | INT | default 0 |
| `updated_at` | TIMESTAMPTZ | default NOW() |

### `problems`

| Column | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `question_latex` | TEXT | NOT NULL |
| `answer_key` | TEXT | NOT NULL |
| `difficulty` | INT | NOT NULL |
| `main_topic` | VARCHAR(50) | NOT NULL |
| `subtopic` | VARCHAR(50) | NOT NULL |
| `grading_mode` | VARCHAR(20) | default 'equivalent' |
| `answer_type` | VARCHAR(20) | NOT NULL, default 'expression' |
| `calculator_allowed` | VARCHAR(20) | NOT NULL, default 'none' |
| `solution_latex` | TEXT | NOT NULL, default '' |
| `question_image` | TEXT | NOT NULL, default '' |
| `time_limit_seconds` | INT | nullable |

**CHECK constraints**:
- `calculator_allowed` IN (`none`, `scientific`, `graphing`, `cas`)
- `answer_type` IN (`expression`, `numeric`, `set`, `tuple`, `list`, `interval`, `inequality`, `equation`, `boolean`, `word`, `matrix`, `multi_part`)

### `attempts`

| Column | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `user_id` | UUID | FK -> users ON DELETE CASCADE |
| `problem_id` | UUID | FK -> problems ON DELETE CASCADE |
| `user_input` | TEXT | NOT NULL |
| `is_correct` | BOOLEAN | NOT NULL |
| `elo_before` | INT | NOT NULL |
| `elo_after` | INT | NOT NULL |
| `time_taken_ms` | INT | nullable |
| `main_topic` | VARCHAR(50) | nullable |
| `created_at` | TIMESTAMPTZ | default NOW() |

### `oauth_accounts`

| Column | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `user_id` | UUID | FK -> users ON DELETE CASCADE |
| `provider` | VARCHAR(20) | NOT NULL (google, github) |
| `provider_user_id` | VARCHAR(255) | NOT NULL |
| `provider_email` | VARCHAR(255) | nullable |
| `created_at` | TIMESTAMPTZ | default NOW() |

**UNIQUE**: (`provider`, `provider_user_id`)

### `topics`

| Column | Type | Constraints |
|---|---|---|
| `id` | VARCHAR(50) | PK |
| `display_name` | VARCHAR(100) | NOT NULL |
| `sort_order` | INT | NOT NULL |
| `enabled` | BOOLEAN | default TRUE |

9 topics: arithmetic, algebra1, geometry, algebra2, precalculus, calculus, differential_equations, multivariable_calculus, linear_algebra.

### `subtopics`

| Column | Type | Constraints |
|---|---|---|
| `topic_id` | VARCHAR(50) | PK (composite), FK -> topics ON DELETE CASCADE |
| `id` | VARCHAR(50) | PK (composite) |
| `display_name` | VARCHAR(100) | NOT NULL |
| `sort_order` | INT | NOT NULL |
| `enabled` | BOOLEAN | default TRUE |

89 subtopics total across all topics (see migration 014).

### `email_verification_tokens`

| Column | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `user_id` | UUID | FK -> users ON DELETE CASCADE |
| `token` | VARCHAR(64) | UNIQUE, NOT NULL (hex-encoded 32 bytes) |
| `created_at` | TIMESTAMPTZ | default NOW() |
| `expires_at` | TIMESTAMPTZ | NOT NULL (1 hour) |
| `used_at` | TIMESTAMPTZ | nullable |

### `email_verification_sends`

Rate limiting table: 1 email per minute per user.

| Column | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `user_id` | UUID | FK -> users ON DELETE CASCADE |
| `sent_at` | TIMESTAMPTZ | default NOW() |

### `password_reset_tokens`

| Column | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `user_id` | UUID | FK -> users ON DELETE CASCADE |
| `token` | VARCHAR(64) | UNIQUE, NOT NULL (hex-encoded 32 bytes) |
| `created_at` | TIMESTAMPTZ | default NOW() |
| `expires_at` | TIMESTAMPTZ | NOT NULL (30 minutes) |
| `used_at` | TIMESTAMPTZ | nullable |

### `password_reset_sends`

Rate limiting table: 1 email per minute per user.

| Column | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `user_id` | UUID | FK -> users ON DELETE CASCADE |
| `sent_at` | TIMESTAMPTZ | default NOW() |

### `daily_puzzles`

Wraps a `problems` row with daily puzzle metadata (hints, editorial, scheduling).

| Column | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `problem_id` | UUID | FK -> problems ON DELETE RESTRICT, NOT NULL |
| `puzzle_date` | DATE | UNIQUE, nullable (NULL = unscheduled/in pool) |
| `title` | VARCHAR(200) | NOT NULL, default '' |
| `hints` | JSONB | NOT NULL, default '[]' (array of LaTeX hint strings) |
| `editorial_latex` | TEXT | NOT NULL, default '' |
| `source` | VARCHAR(200) | NOT NULL, default '' |
| `status` | VARCHAR(20) | NOT NULL, default 'draft', CHECK IN ('draft', 'scheduled', 'archived') |
| `created_at` | TIMESTAMPTZ | default NOW() |

### `daily_puzzle_attempts`

| Column | Type | Constraints |
|---|---|---|
| `id` | UUID | PK, default `gen_random_uuid()` |
| `user_id` | UUID | FK -> users ON DELETE CASCADE, NOT NULL |
| `daily_puzzle_id` | UUID | FK -> daily_puzzles ON DELETE CASCADE, NOT NULL |
| `user_input` | TEXT | NOT NULL |
| `is_correct` | BOOLEAN | NOT NULL |
| `hints_used` | INT | NOT NULL, default 0 |
| `time_taken_ms` | INT | nullable |
| `created_at` | TIMESTAMPTZ | default NOW() |

**Additional columns on `users` table** (added by migration 019):

| Column | Type | Constraints |
|---|---|---|
| `daily_puzzle_streak` | INT | NOT NULL, default 0 |
| `daily_puzzle_last_solve` | DATE | nullable |

## Relationships

```
users ──┬── user_topic_elo
        ├── oauth_accounts
        ├── attempts
        ├── daily_puzzle_attempts
        ├── email_verification_tokens
        ├── email_verification_sends
        ├── password_reset_tokens
        └── password_reset_sends

problems ──┬── attempts
           └── daily_puzzles

daily_puzzles ── daily_puzzle_attempts

topics ──── subtopics
```

All user FKs cascade on delete.

**Note**: `accepted_tos` is validated at the API level during registration but NOT stored in the database.

## PostgreSQL Functions

### `get_user_elo(p_user_id UUID, p_topic VARCHAR) -> INTEGER`

Returns the user's ELO for a topic. Creates a default 1500 entry if none exists.

### `update_user_elo(p_user_id UUID, p_topic VARCHAR, p_new_elo INTEGER) -> VOID`

Upserts user ELO for a topic. Updates `updated_at` timestamp.

## Indexes

```sql
-- users
idx_users_elo                       ON users(elo DESC)  -- legacy, may not exist

-- problems
idx_problems_difficulty             ON problems(difficulty)
idx_problems_main_topic             ON problems(main_topic)
idx_problems_subtopic               ON problems(subtopic)
idx_problems_main_subtopic          ON problems(main_topic, subtopic)
idx_problems_calculator_allowed     ON problems(calculator_allowed)
idx_problems_answer_type            ON problems(answer_type)

-- attempts
idx_attempts_user_id                ON attempts(user_id, created_at DESC)
idx_attempts_user_topic_time        ON attempts(user_id, main_topic, created_at DESC)

-- user_topic_elo
idx_user_topic_elo_topic_elo        ON user_topic_elo(topic, elo DESC)

-- oauth_accounts
idx_oauth_accounts_user_id          ON oauth_accounts(user_id)

-- email_verification_tokens
idx_verification_tokens_token       ON email_verification_tokens(token)
idx_verification_tokens_user_id     ON email_verification_tokens(user_id)

-- email_verification_sends
idx_verification_sends_user_time    ON email_verification_sends(user_id, sent_at DESC)

-- password_reset_tokens
idx_password_reset_tokens_token     ON password_reset_tokens(token)
idx_password_reset_tokens_user_id   ON password_reset_tokens(user_id)

-- password_reset_sends
idx_password_reset_sends_user_time  ON password_reset_sends(user_id, sent_at DESC)

-- daily_puzzles
idx_daily_puzzles_date              ON daily_puzzles(puzzle_date) WHERE puzzle_date IS NOT NULL
idx_daily_puzzles_status            ON daily_puzzles(status)

-- daily_puzzle_attempts
idx_dpa_user_puzzle                 ON daily_puzzle_attempts(user_id, daily_puzzle_id)
idx_dpa_puzzle                      ON daily_puzzle_attempts(daily_puzzle_id)

-- performance indexes (migration 020)
idx_attempts_user_topic_created     ON attempts(user_id, main_topic, created_at DESC)
idx_daily_puzzles_date_status       ON daily_puzzles(puzzle_date, status) WHERE puzzle_date IS NOT NULL
```

## Migration History

| # | Name | Description |
|---|---|---|
| 001 | initial | users, problems, attempts tables |
| 003 | topic_elo_table | user_topic_elo table, move ELO from users, add main_topic/subtopic to problems |
| 004 | seed_problems | Sample problems for algebra1, calculus, linear_algebra, algebra2 |
| 005 | oauth | oauth_accounts table, make password_hash nullable |
| 006 | dynamic_topics | topics + subtopics tables with seed data (8 topics) |
| 007 | calculator_allowed | calculator_allowed column on problems |
| 008 | email_verification | email_verified on users, email_verification_tokens + sends tables |
| 009 | password_reset | password_reset_tokens + sends tables |
| 010 | answer_type | answer_type column on problems |
| 011 | solution_latex | solution_latex column on problems |
| 012 | question_image | question_image column on problems |
| 013 | time_limit | time_limit_seconds column on problems |
| 014 | granular_subtopics | Replace subtopics with 89 granular ones, add differential_equations topic |
| 015 | delete_orphaned | Delete problems with orphaned subtopics |
| 016 | streaks_and_peak_elo | current_streak + last_active_date on users, topic_streak + peak columns on user_topic_elo |
| 017 | normalize_email | Lowercase all emails |
| 018 | fix_answer_key_formats | Fix set Python list notation, delete broken list-of-Matrix problems |
| 019 | daily_puzzles | daily_puzzles + daily_puzzle_attempts tables, daily_puzzle_streak + daily_puzzle_last_solve on users |
| 020 | performance_indexes | Composite indexes for ELO history queries and daily puzzle lookups |
