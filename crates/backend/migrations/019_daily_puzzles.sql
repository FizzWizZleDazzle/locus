-- Daily puzzle feature: puzzle table, attempts, and streak tracking

CREATE TABLE daily_puzzles (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    problem_id      UUID NOT NULL REFERENCES problems(id) ON DELETE RESTRICT,
    puzzle_date     DATE UNIQUE,  -- NULL = unscheduled (in pool)
    title           VARCHAR(200) NOT NULL DEFAULT '',
    hints           JSONB NOT NULL DEFAULT '[]',
    editorial_latex TEXT NOT NULL DEFAULT '',
    source          VARCHAR(200) NOT NULL DEFAULT '',
    status          VARCHAR(20) NOT NULL DEFAULT 'draft'
                    CHECK (status IN ('draft', 'scheduled', 'archived')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_daily_puzzles_date ON daily_puzzles(puzzle_date) WHERE puzzle_date IS NOT NULL;
CREATE INDEX idx_daily_puzzles_status ON daily_puzzles(status);

CREATE TABLE daily_puzzle_attempts (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    daily_puzzle_id UUID NOT NULL REFERENCES daily_puzzles(id) ON DELETE CASCADE,
    user_input      TEXT NOT NULL,
    is_correct      BOOLEAN NOT NULL,
    hints_used      INT NOT NULL DEFAULT 0,
    time_taken_ms   INT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_dpa_user_puzzle ON daily_puzzle_attempts(user_id, daily_puzzle_id);
CREATE INDEX idx_dpa_puzzle ON daily_puzzle_attempts(daily_puzzle_id);

-- Daily puzzle streak tracking (separate from ranked streaks on users table)
ALTER TABLE users
  ADD COLUMN daily_puzzle_streak INT NOT NULL DEFAULT 0,
  ADD COLUMN daily_puzzle_last_solve DATE;
