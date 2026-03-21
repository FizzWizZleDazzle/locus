CREATE TABLE status_checks (
  id SERIAL PRIMARY KEY,
  status_code INTEGER,
  response_time_ms INTEGER,
  is_healthy BOOLEAN NOT NULL,
  checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_status_checks_time ON status_checks(checked_at DESC);
