-- Initial schema for Locus

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    elo INTEGER NOT NULL DEFAULT 1500,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for leaderboard queries
CREATE INDEX idx_users_elo ON users(elo DESC);

-- Problems table
CREATE TABLE problems (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    question_latex TEXT NOT NULL,
    answer_key TEXT NOT NULL,
    difficulty INTEGER NOT NULL,
    topic VARCHAR(50) NOT NULL,
    grading_mode VARCHAR(20) NOT NULL DEFAULT 'equivalent'
);

-- Index for problem selection
CREATE INDEX idx_problems_difficulty ON problems(difficulty);
CREATE INDEX idx_problems_topic ON problems(topic);

-- Attempts table
CREATE TABLE attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    problem_id UUID NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
    user_input TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL,
    elo_before INTEGER NOT NULL,
    elo_after INTEGER NOT NULL,
    time_taken_ms INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for user history
CREATE INDEX idx_attempts_user_id ON attempts(user_id, created_at DESC);
