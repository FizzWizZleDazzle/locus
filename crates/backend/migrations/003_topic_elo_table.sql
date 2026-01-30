-- Create a separate table for per-topic ELO ratings (more flexible than 8 columns)
CREATE TABLE IF NOT EXISTS user_topic_elo (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    topic VARCHAR(50) NOT NULL,
    elo INTEGER NOT NULL DEFAULT 1500,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, topic)
);

-- Index for leaderboard queries
CREATE INDEX IF NOT EXISTS idx_user_topic_elo_topic_elo ON user_topic_elo(topic, elo DESC);

-- Remove old elo column from users (if it exists)
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'users' AND column_name = 'elo'
    ) THEN
        ALTER TABLE users DROP COLUMN elo;
    END IF;
END $$;

-- Update problems table to have main_topic and subtopic
DO $$
BEGIN
    -- Drop old topic column if exists
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'problems' AND column_name = 'topic'
    ) THEN
        ALTER TABLE problems DROP COLUMN topic;
    END IF;

    -- Add main_topic if doesn't exist
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'problems' AND column_name = 'main_topic'
    ) THEN
        ALTER TABLE problems ADD COLUMN main_topic VARCHAR(50) NOT NULL DEFAULT 'algebra1';
    END IF;

    -- Add subtopic if doesn't exist
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'problems' AND column_name = 'subtopic'
    ) THEN
        ALTER TABLE problems ADD COLUMN subtopic VARCHAR(50) NOT NULL DEFAULT 'linear_equations';
    END IF;
END $$;

-- Update attempts table to store which topic was used
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'attempts' AND column_name = 'main_topic'
    ) THEN
        ALTER TABLE attempts ADD COLUMN main_topic VARCHAR(50);
    END IF;
END $$;

-- Create indexes on main_topic and subtopic for faster queries
CREATE INDEX IF NOT EXISTS idx_problems_main_topic ON problems(main_topic);
CREATE INDEX IF NOT EXISTS idx_problems_subtopic ON problems(subtopic);
CREATE INDEX IF NOT EXISTS idx_problems_main_subtopic ON problems(main_topic, subtopic);

-- Function to get or create user's ELO for a topic
CREATE OR REPLACE FUNCTION get_user_elo(p_user_id UUID, p_topic VARCHAR(50))
RETURNS INTEGER AS $$
DECLARE
    v_elo INTEGER;
BEGIN
    SELECT elo INTO v_elo
    FROM user_topic_elo
    WHERE user_id = p_user_id AND topic = p_topic;

    IF v_elo IS NULL THEN
        INSERT INTO user_topic_elo (user_id, topic, elo)
        VALUES (p_user_id, p_topic, 1500)
        ON CONFLICT (user_id, topic) DO NOTHING;
        RETURN 1500;
    END IF;

    RETURN v_elo;
END;
$$ LANGUAGE plpgsql;

-- Function to update user's ELO for a topic
CREATE OR REPLACE FUNCTION update_user_elo(p_user_id UUID, p_topic VARCHAR(50), p_new_elo INTEGER)
RETURNS VOID AS $$
BEGIN
    INSERT INTO user_topic_elo (user_id, topic, elo, updated_at)
    VALUES (p_user_id, p_topic, p_new_elo, NOW())
    ON CONFLICT (user_id, topic)
    DO UPDATE SET elo = p_new_elo, updated_at = NOW();
END;
$$ LANGUAGE plpgsql;
