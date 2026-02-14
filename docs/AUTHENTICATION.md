# Authentication & OAuth Guide

Complete guide to authentication flows in the Locus platform.

## Overview

Locus supports two authentication methods:
1. **Email/Password** - Traditional registration with email verification
2. **OAuth** - Google and GitHub single sign-on with account linking

All authenticated requests use JWT tokens (HS256, 24-hour expiry).

## Authentication Flows

### Flow 1: Email/Password Registration

```
User → Frontend: Enter email, password, username
  ↓
Frontend → Backend: POST /api/auth/register
  ↓
Backend: Validate input (email format, password strength, unique username)
  ↓
Backend: Hash password with Argon2id
  ↓
Backend → Database: INSERT INTO users (email_verified=false)
  ↓
Backend: Generate email verification token (1h expiry)
  ↓
Backend → Database: INSERT INTO email_verifications
  ↓
Backend → SMTP: Send verification email with token link
  ↓
Backend → Frontend: 200 OK {message: "Check your email"}
  ↓
User: Click verification link in email
  ↓
Frontend → Backend: GET /api/auth/verify-email/{token}
  ↓
Backend → Database: SELECT * FROM email_verifications WHERE token=?
  ↓
Backend: Check if token expired (created_at + 1h > NOW)
  ↓
Backend → Database: UPDATE users SET email_verified=true
  ↓
Backend → Database: DELETE FROM email_verifications WHERE token=?
  ↓
Backend → Frontend: Redirect to login page
  ↓
User: Login with email/password (Flow 2)
```

**Key Points:**
- Password must be 8+ characters
- Email verification required before login
- Verification tokens expire after 1 hour
- Tokens are single-use (deleted after verification)

### Flow 2: Email/Password Login

```
User → Frontend: Enter email, password
  ↓
Frontend → Backend: POST /api/auth/login
  ↓
Backend → Database: SELECT * FROM users WHERE email=?
  ↓
Backend: Verify password with Argon2id
  ↓
Backend: Check if email_verified=true
  ↓
Backend: Generate JWT token (24h expiry)
  JWT payload: {user_id, username, exp}
  ↓
Backend → Frontend: 200 OK {token, username}
  ↓
Frontend: Store token in localStorage
  ↓
Frontend: Set Authorization header for all API requests:
  Authorization: Bearer {token}
```

**JWT Structure:**
```json
{
  "user_id": "uuid-here",
  "username": "alice",
  "exp": 1640995200
}
```

**Error Cases:**
- Email not found → 401 Unauthorized
- Wrong password → 401 Unauthorized
- Email not verified → 403 Forbidden "Please verify your email"
- Account locked → 403 Forbidden

### Flow 3: OAuth Login (New User)

```
User → Frontend: Click "Login with Google/GitHub"
  ↓
Frontend → Backend: GET /api/auth/oauth/{provider}/authorize
  ↓
Backend → Frontend: Redirect to provider OAuth consent page
  provider: https://accounts.google.com/o/oauth2/auth?...
  ↓
User: Approve OAuth consent
  ↓
Provider → Backend: GET /api/auth/oauth/{provider}/callback?code=...
  ↓
Backend → Provider: Exchange code for access_token
  POST https://oauth2.googleapis.com/token
  ↓
Backend → Provider: Fetch user profile
  GET https://www.googleapis.com/oauth2/v1/userinfo
  Response: {id, email, name}
  ↓
Backend → Database: SELECT * FROM oauth_accounts WHERE provider=? AND provider_user_id=?
  Result: None (new user)
  ↓
Backend: Generate unique username from email or name
  Example: "alice.smith" or "alice.smith.2" if taken
  ↓
Backend → Database: BEGIN TRANSACTION
  ↓
Backend → Database: INSERT INTO users (email, username, email_verified=true)
  Note: OAuth emails are pre-verified
  ↓
Backend → Database: INSERT INTO oauth_accounts (user_id, provider, provider_user_id)
  ↓
Backend → Database: INSERT INTO user_topic_elo (user_id, topic) for all 8 topics
  Default ELO: 1500
  ↓
Backend → Database: COMMIT
  ↓
Backend: Generate JWT token
  ↓
Backend → Frontend: Redirect to /auth/callback?token={jwt}&username={username}
  ↓
Frontend: Extract token from URL, store in localStorage
  ↓
Frontend: Redirect to home page (logged in)
```

**Providers:**
- **Google**: Uses Google People API
- **GitHub**: Uses GitHub Users API

### Flow 4: OAuth Login (Existing User - Link Account)

```
User → Frontend: Click "Login with GitHub" (already has Google account)
  ↓
Frontend → Backend: GET /api/auth/oauth/github/authorize
  ↓
Backend → Frontend: Redirect to GitHub OAuth
  ↓
User: Approve OAuth consent
  ↓
Provider → Backend: GET /api/auth/oauth/github/callback?code=...
  ↓
Backend → Provider: Exchange code, fetch profile
  Response: {id, email: "alice@example.com"}
  ↓
Backend → Database: SELECT * FROM oauth_accounts WHERE provider='github' AND provider_user_id=?
  Result: None (new OAuth provider for this user)
  ↓
Backend → Database: SELECT * FROM users WHERE email='alice@example.com'
  Result: Found (user exists via Google OAuth)
  ↓
Backend → Database: INSERT INTO oauth_accounts (user_id, provider='github', provider_user_id)
  Links GitHub to existing user account
  ↓
Backend: Generate JWT token for existing user
  ↓
Backend → Frontend: Redirect with token
  ↓
Frontend: User is now logged in, can use either Google or GitHub
```

**Account Linking:**
- If email matches existing user, link OAuth provider to that account
- User can have multiple OAuth providers (Google + GitHub)
- Cannot link if different user already linked to that provider

**Error Cases:**
- OAuth provider already linked to different account → 400 Bad Request
- Email mismatch (rare) → Manual account merge required

### Flow 5: Password Reset

```
User → Frontend: Click "Forgot Password"
  ↓
Frontend → Backend: POST /api/auth/request-password-reset
  Body: {email: "alice@example.com"}
  ↓
Backend → Database: SELECT * FROM users WHERE email=?
  ↓
Backend: Generate reset token (1h expiry)
  ↓
Backend → Database: INSERT INTO password_resets (user_id, token, expires_at)
  ↓
Backend → SMTP: Send reset email with token link
  ↓
Backend → Frontend: 200 OK {message: "Check your email"}
  ↓
User: Click reset link in email
  ↓
Frontend → Backend: GET /api/auth/reset-password/{token}
  ↓
Backend → Database: SELECT * FROM password_resets WHERE token=?
  ↓
Backend: Check if token expired
  ↓
Backend → Frontend: Display password reset form
  ↓
User: Enter new password
  ↓
Frontend → Backend: POST /api/auth/reset-password
  Body: {token, new_password}
  ↓
Backend: Validate password strength
  ↓
Backend: Hash new password with Argon2id
  ↓
Backend → Database: BEGIN TRANSACTION
  ↓
Backend → Database: UPDATE users SET password_hash=? WHERE id=?
  ↓
Backend → Database: DELETE FROM password_resets WHERE token=?
  ↓
Backend → Database: COMMIT
  ↓
Backend → Frontend: 200 OK {message: "Password reset successful"}
  ↓
Frontend: Redirect to login page
```

**Security:**
- Reset tokens expire after 1 hour
- Tokens are single-use (deleted after reset)
- Password strength requirements enforced
- Rate limited to 5 requests per 15 minutes

## Email Verification

### Token Lifecycle

```
Token Generation:
- Random 32-byte hex string (64 characters)
- Stored in email_verifications table
- Associated with user_id and email
- Created timestamp for expiry check

Token Validation:
1. Check token exists in database
2. Verify created_at + 1h > NOW
3. Match email to user account
4. Delete token after successful verification

Expiry Handling:
- Tokens older than 1h are considered invalid
- User can request new verification email
- Old tokens automatically cleaned up (future: cron job)
```

**Resend Verification Email:**
```
POST /api/auth/resend-verification
Body: {email: "alice@example.com"}

Logic:
1. Check if user exists
2. Check if already verified (skip if true)
3. Delete old verification tokens for this user
4. Generate new token
5. Send new email
```

## OAuth Providers

### Google OAuth Configuration

**Setup:**
1. Create project in Google Cloud Console
2. Enable Google+ API
3. Create OAuth 2.0 credentials
4. Set authorized redirect URI: `http://localhost:3000/api/auth/oauth/google/callback`

**Environment Variables:**
```bash
GOOGLE_CLIENT_ID=your-client-id.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=your-client-secret
GOOGLE_REDIRECT_URI=http://localhost:3000/api/auth/oauth/google/callback
```

**Scopes Requested:**
- `openid` - Standard OpenID Connect
- `email` - User's email address
- `profile` - User's basic profile info (name, picture)

**Provider User ID:**
- Google's unique user identifier (`sub` claim)
- Stable across all Google services
- Used to link Google account to Locus user

### GitHub OAuth Configuration

**Setup:**
1. Go to GitHub Settings → Developer settings → OAuth Apps
2. Create new OAuth app
3. Set Authorization callback URL: `http://localhost:3000/api/auth/oauth/github/callback`

**Environment Variables:**
```bash
GITHUB_CLIENT_ID=your-client-id
GITHUB_CLIENT_SECRET=your-client-secret
GITHUB_REDIRECT_URI=http://localhost:3000/api/auth/oauth/github/callback
```

**Scopes Requested:**
- `user:email` - Read user's email addresses

**Provider User ID:**
- GitHub's unique user ID (numeric)
- Stable, never changes

## JWT Tokens

### Structure

**Header:**
```json
{
  "alg": "HS256",
  "typ": "JWT"
}
```

**Payload:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "alice",
  "exp": 1640995200,
  "iat": 1640908800
}
```

**Signature:**
```
HMACSHA256(
  base64UrlEncode(header) + "." + base64UrlEncode(payload),
  secret
)
```

### Token Management

**Storage (Frontend):**
```javascript
// Store token after login
localStorage.setItem('token', jwt_token);

// Include in API requests
headers: {
  'Authorization': `Bearer ${localStorage.getItem('token')}`
}

// Logout
localStorage.removeItem('token');
```

**Validation (Backend):**
```rust
// Middleware checks:
1. Token present in Authorization header
2. Token format valid (Bearer {token})
3. Signature valid (verify with JWT_SECRET)
4. Token not expired (exp > NOW)
5. Extract user_id from payload

// If valid: attach user_id to request context
// If invalid: return 401 Unauthorized
```

**Expiry:**
- Default: 24 hours (86400 seconds)
- Configurable via `JWT_EXPIRY_SECONDS` env var
- No refresh tokens (user must re-login after expiry)

**Future Enhancement:**
- Implement refresh tokens for seamless re-authentication
- Short-lived access tokens (15min) + long-lived refresh tokens (7 days)

## Security Considerations

### Password Requirements

**Minimum Requirements:**
- Length: 8+ characters
- No maximum length (Argon2id handles any length)

**Recommended:**
- 12+ characters
- Mix of uppercase, lowercase, numbers, symbols
- Avoid common passwords (future: check against breach database)

**Storage:**
```rust
// Hash with Argon2id
let config = argon2::Config::default();
let salt = SaltString::generate(&mut OsRng);
let password_hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?;

// Verify
let matches = argon2::verify_encoded(&password_hash, password.as_bytes())?;
```

### Rate Limiting

**Endpoints:**
```rust
// Registration: 5 requests per 15 minutes
POST /api/auth/register
  → Limit: 5 req/15min per IP

// Login: 10 requests per 15 minutes
POST /api/auth/login
  → Limit: 10 req/15min per IP

// Password reset request: 5 requests per 15 minutes
POST /api/auth/request-password-reset
  → Limit: 5 req/15min per IP

// Email verification resend: 3 requests per 15 minutes
POST /api/auth/resend-verification
  → Limit: 3 req/15min per IP
```

**Implementation:**
```rust
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

let register_governor = GovernorConfigBuilder::default()
    .per_second(1)
    .burst_size(5)
    .finish()
    .unwrap();

Router::new()
    .route("/api/auth/register", post(register_handler))
    .layer(GovernorLayer { config: register_governor })
```

### Token Security

**JWT Secret:**
- Minimum 32 characters
- Randomly generated, stored in environment variable
- Never commit to version control
- Rotate periodically in production

**Token Transmission:**
- HTTPS only in production (prevents token interception)
- Authorization header (more secure than query params)
- HttpOnly cookies (future enhancement)

**Token Revocation:**
- Currently not supported (stateless JWT)
- Future: Implement token blacklist in Redis
- Workaround: Change JWT_SECRET to invalidate all tokens

### OAuth Security

**State Parameter:**
```rust
// Generate random state before redirect
let state = generate_random_string(32);
session.insert("oauth_state", state.clone())?;

// Verify state in callback
let received_state = query_params.get("state")?;
let stored_state = session.get("oauth_state")?;
if received_state != stored_state {
    return Err("CSRF attack detected");
}
```

**PKCE (Proof Key for Code Exchange):**
- Future enhancement for additional security
- Prevents authorization code interception attacks

## API Reference

### Register

```http
POST /api/auth/register
Content-Type: application/json

{
  "username": "alice",
  "email": "alice@example.com",
  "password": "SecurePass123!"
}

Response 200:
{
  "message": "Registration successful. Please check your email to verify your account."
}

Response 400:
{
  "error": "Email already registered"
}
{
  "error": "Username already taken"
}
{
  "error": "Password must be at least 8 characters"
}
```

### Verify Email

```http
GET /api/auth/verify-email/{token}

Response 200:
Redirect to /login?verified=true

Response 400:
{
  "error": "Invalid or expired verification token"
}
```

### Login

```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "alice@example.com",
  "password": "SecurePass123!"
}

Response 200:
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...", // gitleaks:allow
  "username": "alice"
}

Response 401:
{
  "error": "Invalid email or password"
}

Response 403:
{
  "error": "Please verify your email before logging in"
}
```

### OAuth Authorize

```http
GET /api/auth/oauth/{provider}/authorize
  provider: "google" | "github"

Response 302:
Redirect to provider OAuth consent page
```

### OAuth Callback

```http
GET /api/auth/oauth/{provider}/callback?code={code}&state={state}

Response 302:
Redirect to /auth/callback?token={jwt}&username={username}

Response 400:
{
  "error": "OAuth provider already linked to a different account"
}
```

### Request Password Reset

```http
POST /api/auth/request-password-reset
Content-Type: application/json

{
  "email": "alice@example.com"
}

Response 200:
{
  "message": "If that email exists, a password reset link has been sent."
}
```

### Reset Password

```http
POST /api/auth/reset-password
Content-Type: application/json

{
  "token": "reset-token-here",
  "new_password": "NewSecurePass123!"
}

Response 200:
{
  "message": "Password reset successful"
}

Response 400:
{
  "error": "Invalid or expired reset token"
}
{
  "error": "Password must be at least 8 characters"
}
```

### Resend Verification Email

```http
POST /api/auth/resend-verification
Content-Type: application/json

{
  "email": "alice@example.com"
}

Response 200:
{
  "message": "Verification email sent"
}

Response 400:
{
  "error": "Email already verified"
}
```

## Troubleshooting

### "Invalid token" errors

**Symptoms:**
- 401 Unauthorized on API requests
- User logged out unexpectedly

**Causes:**
1. Token expired (24h expiry)
2. JWT_SECRET changed on server
3. Token malformed (localStorage corruption)

**Solutions:**
- User must re-login
- Check token in localStorage: `localStorage.getItem('token')`
- Verify token at jwt.io (check signature, expiry)

### Email verification not working

**Symptoms:**
- Verification link returns error
- Email not received

**Causes:**
1. Token expired (1h expiry)
2. Email in spam folder
3. SMTP configuration incorrect

**Solutions:**
- Request new verification email
- Check spam folder
- Verify SMTP env vars: `SMTP_HOST`, `SMTP_USERNAME`, `SMTP_PASSWORD`
- Check backend logs for SMTP errors

### OAuth "Account already linked" error

**Symptoms:**
- Cannot link second OAuth provider
- Error: "OAuth provider already linked to a different account"

**Cause:**
- Another user already linked this Google/GitHub account

**Solution:**
- User must login with original OAuth provider
- Contact support to unlink account (manual database operation)

### Rate limit exceeded

**Symptoms:**
- 429 Too Many Requests
- "Rate limit exceeded" error

**Cause:**
- Too many login/register attempts from same IP

**Solution:**
- Wait 15 minutes for rate limit to reset
- Verify you're not in an infinite retry loop
- Check if multiple users share same IP (NAT, VPN)

### Password reset link not working

**Symptoms:**
- Reset link returns "Invalid token"
- Link worked once, but not twice

**Causes:**
1. Token expired (1h)
2. Token already used (single-use)
3. Multiple reset requests (old tokens invalidated)

**Solutions:**
- Request new reset link
- Use link within 1 hour
- Don't click link multiple times

## Database Schema Reference

### users table

```sql
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  username VARCHAR(50) UNIQUE NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR(255),  -- NULL for OAuth-only users
  email_verified BOOLEAN DEFAULT false,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### oauth_accounts table

```sql
CREATE TABLE oauth_accounts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  provider VARCHAR(50) NOT NULL,  -- 'google' | 'github'
  provider_user_id VARCHAR(255) NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  UNIQUE(provider, provider_user_id)
);

CREATE INDEX idx_oauth_user_id ON oauth_accounts(user_id);
```

### email_verifications table

```sql
CREATE TABLE email_verifications (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  email VARCHAR(255) NOT NULL,
  token VARCHAR(64) UNIQUE NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_email_verifications_token ON email_verifications(token);
```

### password_resets table

```sql
CREATE TABLE password_resets (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token VARCHAR(64) UNIQUE NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_password_resets_token ON password_resets(token);
```

## Further Reading

- [JWT Best Practices](https://tools.ietf.org/html/rfc8725)
- [OAuth 2.0 Security](https://tools.ietf.org/html/rfc6749)
- [Argon2 Password Hashing](https://github.com/P-H-C/phc-winner-argon2)
- [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html)
