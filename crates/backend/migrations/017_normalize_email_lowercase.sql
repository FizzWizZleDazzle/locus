-- Normalize existing emails to lowercase
UPDATE users SET email = LOWER(email) WHERE email != LOWER(email);

-- Replace the plain UNIQUE constraint with a case-insensitive unique index.
-- This prevents duplicates at the DB level even if application code misses normalization.
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_email_key;
DROP INDEX IF EXISTS users_email_key;
CREATE UNIQUE INDEX users_email_lower_unique ON users (LOWER(email));
