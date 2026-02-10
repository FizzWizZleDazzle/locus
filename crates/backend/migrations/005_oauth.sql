-- Make password_hash optional for OAuth-only users
ALTER TABLE users ALTER COLUMN password_hash DROP NOT NULL;

-- OAuth provider accounts linked to users
CREATE TABLE oauth_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(20) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    provider_email VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, provider_user_id)
);
CREATE INDEX idx_oauth_accounts_user_id ON oauth_accounts(user_id);
