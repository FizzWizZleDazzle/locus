-- Migration 016: Add streaks and peak ELO tracking

-- Global daily streak on users table
ALTER TABLE users
  ADD COLUMN current_streak INT NOT NULL DEFAULT 0,
  ADD COLUMN last_active_date DATE;

-- Per-topic streak and peak ELO on user_topic_elo table
ALTER TABLE user_topic_elo
  ADD COLUMN topic_streak INT NOT NULL DEFAULT 0,
  ADD COLUMN peak_topic_streak INT NOT NULL DEFAULT 0,
  ADD COLUMN peak_elo INT NOT NULL DEFAULT 1500;

-- Backfill peak_elo from current elo values
UPDATE user_topic_elo SET peak_elo = elo;

-- Index to speed up ELO history queries (user, topic, time-ordered)
CREATE INDEX ON attempts(user_id, main_topic, created_at DESC);
