# API Reference

Base path: `/api`

## Authentication

Three auth mechanisms (in priority order for user endpoints):

| Type | Mechanism | Used by |
|---|---|---|
| httpOnly cookie | `locus_token` cookie (set by server) | User-facing endpoints (browser) |
| Bearer JWT | `Authorization: Bearer <token>` header | API clients / migration fallback |
| API Key | `x-api-key: <key>` header | Factory (`POST /internal/problems`) |

JWT claims: `sub` (user UUID), `username`, `exp` (24h), `iat`.

Cookie attributes: `HttpOnly; SameSite=Lax; Path=/api; Max-Age=86400`. `Secure` flag added in production.

The auth middleware checks for the cookie first, then falls back to the Authorization header. Browser requests must include `credentials: include` for cookies to be sent cross-origin.

## Rate Limiting

IP-based via governor. Unlimited in debug builds.

| Limiter | Limit | Applies to | Env var |
|---|---|---|---|
| Auth | 5 / 15 min | register, verify-email, resend-verification, forgot-password | `RATE_LIMIT_AUTH_PER_15MIN` |
| Login | 10 / 15 min | login | `RATE_LIMIT_LOGIN_PER_15MIN` |
| General | 1000 / min | all other endpoints | `RATE_LIMIT_GENERAL_PER_MIN` |

## Error Format

All errors return:

```json
{ "message": "Error description" }
```

Status codes: `400` validation, `401` auth, `404` not found, `500` server error.

---

## Endpoints

### Health

#### `GET /health`

No auth, no rate limiting. Returns 200 if the server is running.

---

### Auth

#### `POST /auth/register`

Create a new account. Sends verification email.

**Auth**: None | **Rate limit**: Auth

```json
// Request
{ "username": "alice", "email": "alice@example.com", "password": "P@ssw0rd!", "accepted_tos": true }

// Response 200
{ "success": true, "message": "...", "email": "alice@example.com" }
```

Validation: username 3-20 chars `[a-zA-Z0-9_]`, password 8+ chars with uppercase + lowercase + digit, unique username and email, `accepted_tos` must be true.

#### `POST /auth/login`

**Auth**: None | **Rate limit**: Login

```json
// Request
{ "email": "alice@example.com", "password": "P@ssw0rd!" }

// Response 200 (with Set-Cookie header)
{ "user": { UserProfile } }
```

Requires verified email. Sets `locus_token` httpOnly cookie (24-hour expiry). Token is NOT returned in the response body.

#### `POST /auth/logout`

Clears the auth cookie.

**Auth**: None

```json
// Response 200 (with Set-Cookie: Max-Age=0)
{ "success": true, "message": "Logged out successfully" }
```

#### `GET /user/me`

**Auth**: Bearer

```json
// Response 200
{
  "id": "uuid",
  "username": "alice",
  "email": "alice@example.com",
  "email_verified": true,
  "elo_ratings": { "calculus": 1650, "algebra1": 1500 },
  "has_password": true,
  "oauth_providers": ["google"],
  "created_at": "2024-01-01T00:00:00Z",
  "current_streak": 5
}
```

#### `POST /auth/set-password`

Set password for OAuth-only users.

**Auth**: Bearer

```json
// Request
{ "password": "P@ssw0rd!" }

// Response 200
{ UserProfile }
```

#### `POST /auth/change-password`

**Auth**: Bearer

```json
// Request
{ "old_password": "P@ssw0rd!", "new_password": "N3wP@ss!" }

// Response 200
{ "success": true, "message": "..." }
```

#### `POST /auth/change-username`

**Auth**: Bearer

```json
// Request
{ "new_username": "bob" }

// Response 200
{ UserProfile }
```

#### `POST /auth/delete-account`

Cascading delete of all user data. Clears auth cookie.

**Auth**: Cookie or Bearer

```json
// Request
{ "password": "P@ssw0rd!" }  // required if user has password

// Response 200 (with Set-Cookie: Max-Age=0)
{ "success": true, "message": "..." }
```

#### `POST /auth/unlink-oauth`

Remove an OAuth provider. Requires at least one other auth method (password or another provider).

**Auth**: Bearer

```json
// Request
{ "provider": "google" }

// Response 200
{ UserProfile }
```

#### `POST /auth/verify-email`

**Auth**: None | **Rate limit**: Auth

```json
// Request
{ "token": "64-char-hex-token" }

// Response 200
{ "success": true, "message": "..." }
```

Token expires after 1 hour.

#### `POST /auth/resend-verification`

**Auth**: None | **Rate limit**: Auth

```json
// Request
{ "email": "alice@example.com" }

// Response 200
{ "success": true, "message": "..." }
```

Rate limited to 1 email per minute per user. Silently succeeds for non-existent emails.

#### `POST /auth/forgot-password`

**Auth**: None | **Rate limit**: Auth

```json
// Request
{ "email": "alice@example.com" }

// Response 200
{ "success": true, "message": "..." }
```

Always returns success (email enumeration prevention). Token expires after 30 minutes.

#### `POST /auth/validate-reset-token`

**Auth**: None

```json
// Request
{ "token": "64-char-hex-token" }

// Response 200
{ "valid": true, "message": null }
```

#### `POST /auth/reset-password`

**Auth**: None | **Rate limit**: Auth

```json
// Request
{ "token": "64-char-hex-token", "new_password": "N3wP@ss!" }

// Response 200
{ "success": true, "message": "..." }
```

---

### OAuth

#### `GET /auth/oauth/{provider}`

Redirects to Google or GitHub OAuth consent screen. Generates CSRF state JWT (10-minute expiry).

**Auth**: None | **Path params**: `provider` = `google` | `github`

Returns HTML that redirects the browser.

#### `GET /auth/oauth/{provider}/callback`

OAuth provider callback. Creates or links account. Sets `locus_token` httpOnly cookie on success.

**Auth**: None | **Query params**: `code`, `state`, `error`

Returns HTML with `postMessage` to opener window:
- Success: `{ type: "oauth_success", data: { user } }` (token is in Set-Cookie header, not in postMessage)
- Error: `{ type: "oauth_error", error: "..." }`

Email conflict: blocks login if email already exists under a different auth method.

#### `GET /auth/oauth/link/{provider}`

Link an OAuth provider to an existing account.

**Auth**: Cookie (httpOnly `locus_token`) | **Path params**: `provider` = `google` | `github`

Redirects to OAuth consent screen with user ID embedded in CSRF state. Cookie is sent automatically by the browser on popup navigation.

---

### Problems

#### `GET /problems`

Fetch random problems.

**Auth**: Required for ranked mode, optional for practice

| Query param | Type | Default | Description |
|---|---|---|---|
| `practice` | bool | false | Include answer key and solution |
| `main_topic` | string | - | Filter by topic |
| `subtopics` | string | - | Comma-separated subtopic filter |
| `elo` | i32 | - | Override target difficulty (0-3000) |
| `count` | u32 | 1 | Number of problems (max 10) |

```json
// Response 200
[{
  "id": "uuid",
  "question_latex": "\\frac{d}{dx} x^2",
  "difficulty": 1400,
  "main_topic": "calculus",
  "subtopic": "derivatives",
  "grading_mode": "equivalent",
  "answer_type": "expression",
  "calculator_allowed": "none",
  "answer_key": "2*x",           // practice mode only
  "solution_latex": "...",        // practice mode only
  "question_image": "",
  "time_limit_seconds": null
}]
```

Ranked mode uses user's per-topic ELO for difficulty matching (default 1500).

#### `GET /problem` (deprecated)

Same as `GET /problems` but returns a single problem. Returns `Deprecation: true` header.

---

### Submit

#### `POST /submit`

Submit an answer for grading (ranked mode).

**Auth**: Bearer

```json
// Request
{ "problem_id": "uuid", "user_input": "2*x", "time_taken_ms": 15000 }

// Response 200
{
  "is_correct": true,
  "elo_before": 1500,
  "elo_after": 1520,
  "elo_change": 20,
  "topic_streak": 3
}
```

Server-side grading. Updates ELO, streaks (topic + global daily), and records attempt.

---

### Topics

#### `GET /topics`

**Auth**: None

```json
// Response 200
[{
  "id": "calculus",
  "name": "Calculus",
  "enabled": true,
  "subtopics": [
    { "id": "derivatives", "name": "Derivatives" }
  ]
}]
```

Served from in-memory cache.

---

### Leaderboard

#### `GET /leaderboard`

**Auth**: None

| Query param | Type | Default | Description |
|---|---|---|---|
| `topic` | string | calculus | Topic to rank |

```json
// Response 200
{
  "entries": [{ "rank": 1, "username": "alice", "elo": 2100 }],
  "topic": "calculus"
}
```

Returns top 100 users.

---

### Stats

#### `GET /user/stats`

**Auth**: Bearer

```json
// Response 200
{
  "total_attempts": 500,
  "correct_attempts": 350,
  "current_streak": 5,
  "topics": [{
    "topic": "calculus",
    "total": 200,
    "correct": 140,
    "elo": 1650,
    "peak_elo": 1700,
    "topic_streak": 8,
    "peak_topic_streak": 15
  }]
}
```

#### `GET /user/elo-history`

**Auth**: Bearer

| Query param | Type | Required | Description |
|---|---|---|---|
| `topic` | string | yes | Topic to get history for |

```json
// Response 200
{
  "topic": "calculus",
  "history": [{ "day": "2024-01-15", "elo": 1520 }]
}
```

Returns last 30 days. One entry per day (last attempt's ELO).

---

### Factory (Internal)

#### `POST /internal/problems`

Create a new problem.

**Auth**: API Key (`x-api-key` header)

```json
// Request
{
  "question_latex": "\\frac{d}{dx} x^3",
  "answer_key": "3*x**2",
  "difficulty": 1400,
  "main_topic": "calculus",
  "subtopic": "derivative_rules",
  "grading_mode": "equivalent",
  "answer_type": "expression",
  "calculator_allowed": "none",
  "solution_latex": "...",
  "question_image": "",
  "time_limit_seconds": null
}

// Response 201
{ "id": "uuid", "message": "Problem created" }
```

Validates: non-empty question/answer, difficulty 0-3000, valid topic/subtopic from cache, valid grading_mode/answer_type/calculator_allowed, time_limit 1-3600 if provided.
