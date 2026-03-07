-- Performance indexes for common query patterns

-- Composite index for ELO history queries (stats.rs) that filter on user_id + main_topic
CREATE INDEX IF NOT EXISTS idx_attempts_user_topic_created
  ON attempts(user_id, main_topic, created_at DESC);

-- Composite index for daily puzzle date+status queries
CREATE INDEX IF NOT EXISTS idx_daily_puzzles_date_status
  ON daily_puzzles(puzzle_date, status) WHERE puzzle_date IS NOT NULL;
