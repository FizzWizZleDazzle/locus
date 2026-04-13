-- Bookmarks
CREATE TABLE problem_bookmarks (
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    problem_id UUID NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, problem_id)
);
CREATE INDEX idx_bookmarks_user_created ON problem_bookmarks(user_id, created_at DESC);

-- Spaced repetition queue
CREATE TABLE review_queue (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id        UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    problem_id     UUID NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
    next_review    DATE NOT NULL DEFAULT CURRENT_DATE,
    interval_days  INT NOT NULL DEFAULT 1,
    ease_factor    REAL NOT NULL DEFAULT 2.5,
    review_count   INT NOT NULL DEFAULT 0,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, problem_id)
);
CREATE INDEX idx_review_queue_user_due ON review_queue(user_id, next_review);
