-- Add email verification columns to users table
ALTER TABLE users
ADD COLUMN email_verified BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN email_verified_at TIMESTAMPTZ;

-- Mark existing OAuth users as verified (all current users)
UPDATE users SET email_verified = TRUE, email_verified_at = NOW();

-- Create email verification tokens table
CREATE TABLE email_verification_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token VARCHAR(64) NOT NULL UNIQUE,  -- Hex-encoded 32-byte random token
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,    -- 1 hour from creation
    used_at TIMESTAMPTZ                 -- NULL = not used yet
);

CREATE INDEX idx_verification_tokens_token ON email_verification_tokens(token);
CREATE INDEX idx_verification_tokens_user_id ON email_verification_tokens(user_id);

-- Create email verification sends table (for rate limiting)
CREATE TABLE email_verification_sends (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    sent_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_verification_sends_user_time ON email_verification_sends(user_id, sent_at DESC);
