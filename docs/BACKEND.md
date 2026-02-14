# Backend Documentation

Complete guide to the Locus backend built with Axum and PostgreSQL.

## Overview

**Framework:** Axum 0.8
**Runtime:** Tokio (async)
**Database:** SQLx with PostgreSQL
**Auth:** JWT with Argon2 password hashing
**Port:** 3000 (configurable)

---

## Project Structure

```
crates/backend/
├── Cargo.toml
├── migrations/               # SQL migration files
│   ├── 001_initial.sql
│   ├── 003_topic_elo_table.sql
│   └── 004_seed_problems.sql
└── src/
    ├── main.rs              # Server entry point
    ├── config.rs            # Environment configuration
    ├── db.rs                # Database connection pool
    ├── elo.rs               # ELO calculation engine
    ├── grader.rs            # Answer grading logic
    ├── api/                 # API route handlers
    │   ├── mod.rs           # Router, AppState, errors
    │   ├── auth.rs          # Register, login endpoints
    │   ├── problems.rs      # Problem retrieval
    │   ├── submit.rs        # Answer submission
    │   └── leaderboard.rs   # Leaderboard queries
    ├── auth/                # Authentication logic
    │   ├── mod.rs
    │   ├── jwt.rs           # JWT token creation/verification
    │   └── middleware.rs    # Auth middleware extractor
    └── models/              # Database models
        ├── mod.rs
        ├── user.rs          # User CRUD operations
        ├── problem.rs       # Problem CRUD operations
        └── attempt.rs       # Attempt recording
```

---

## Main Application

### main.rs

**File:** `crates/backend/src/main.rs`

**Responsibilities:**
- Server initialization
- Database connection
- Running migrations
- Middleware setup
- Route registration
- Graceful shutdown

**Entry Point:**
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let db = create_pool(&config.database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db)
        .await?;

    let app = create_app(db);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server running on http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
```

**File location:** `crates/backend/src/main.rs:15`

---

## Configuration

### config.rs

**File:** `crates/backend/src/config.rs`

**Environment Variables:**
```rust
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub host: String,
    pub port: u16,
}
```

**Loading:**
```rust
impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .expect("JWT_EXPIRY_HOURS must be a number"),
            host: env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
        }
    }
}
```

**Required Variables:**
- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret for signing JWT tokens

**Optional Variables:**
- `JWT_EXPIRY_HOURS` - Token expiry (default: 24)
- `HOST` - Bind address (default: 0.0.0.0)
- `PORT` - Server port (default: 3000)

---

## Database Layer

### db.rs

**File:** `crates/backend/src/db.rs`

**Connection Pool:**
```rust
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
```

**Pool Configuration:**
- Max connections: 5
- Lazy connection establishment
- Automatic connection recycling
- Health checks via ping

**Usage in Handlers:**
```rust
async fn handler(
    State(db): State<PgPool>,
) -> Result<Json<Response>, ApiError> {
    // Use db for queries
    sqlx::query!("SELECT * FROM users").fetch_all(&db).await?;
    Ok(Json(response))
}
```

---

## API Layer

### Router and AppState

**File:** `crates/backend/src/api/mod.rs`

**AppState:**
```rust
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
}
```

**Router:**
```rust
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/user/me", get(get_me))
        .route("/api/problem", get(get_problem))
        .route("/api/submit", post(submit_answer))
        .route("/api/leaderboard", get(get_leaderboard))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
```

**Middleware Stack:**
1. CORS (permissive in dev)
2. Tracing (request logging)
3. State injection

---

### Auth Routes

**File:** `crates/backend/src/api/auth.rs`

#### POST /api/auth/register

**Handler:**
```rust
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, ApiError>
```

**Logic:**
1. Validate input (username length, email format)
2. Check if username/email already exists
3. Hash password with Argon2
4. Create user in database
5. Generate JWT token
6. Return token and username

**File location:** `crates/backend/src/api/auth.rs:15`

#### POST /api/auth/login

**Handler:**
```rust
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError>
```

**Logic:**
1. Find user by email
2. Verify password with Argon2
3. Generate JWT token
4. Return token and username

**File location:** `crates/backend/src/api/auth.rs:45`

#### GET /api/user/me

**Handler:**
```rust
pub async fn get_me(
    State(db): State<PgPool>,
    auth: AuthUser,
) -> Result<Json<UserProfile>, ApiError>
```

**Logic:**
1. Extract user from JWT (via AuthUser middleware)
2. Query user by ID
3. Get all ELO ratings for all topics
4. Return complete profile

**File location:** `crates/backend/src/api/auth.rs:75`

---

### Problem Routes

**File:** `crates/backend/src/api/problems.rs`

#### GET /api/problem

**Handler:**
```rust
pub async fn get_problem(
    State(db): State<PgPool>,
    auth: Option<AuthUser>,
    Query(query): Query<ProblemQuery>,
) -> Result<Json<ProblemResponse>, ApiError>
```

**Logic:**
1. Parse optional topic and subtopics from query params
2. Get random problem matching filters
3. If authenticated (ranked mode): hide answer
4. If not authenticated (practice mode): include answer
5. Return problem

**Query Parameters:**
- `main_topic` (optional) - Filter by main topic
- `subtopics` (optional) - Comma-separated subtopics

**File location:** `crates/backend/src/api/problems.rs:18`

---

### Submit Routes

**File:** `crates/backend/src/api/submit.rs`

#### POST /api/submit

**Handler:**
```rust
pub async fn submit_answer(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<SubmitRequest>,
) -> Result<Json<SubmitResponse>, ApiError>
```

**Logic:**
1. Get problem from database
2. Grade user input against answer key
3. Get user's current ELO for problem's topic
4. Calculate ELO change based on result
5. Update user's ELO in database
6. Record attempt in database
7. Return result with ELO change

**ELO Calculation:**
```rust
let expected_score = calculate_expected_score(user_elo, problem.difficulty);
let actual_score = if is_correct { 1.0 } else { 0.0 };
let elo_change = calculate_elo_change(user_elo, expected_score, actual_score);
let new_elo = (user_elo as f64 + elo_change).round() as i32;
```

**File location:** `crates/backend/src/api/submit.rs:20`

---

### Leaderboard Routes

**File:** `crates/backend/src/api/leaderboard.rs`

#### GET /api/leaderboard

**Handler:**
```rust
pub async fn get_leaderboard(
    State(db): State<PgPool>,
    Query(query): Query<LeaderboardQuery>,
) -> Result<Json<Vec<LeaderboardEntry>>, ApiError>
```

**Logic:**
1. Parse topic from query parameter
2. Query top 100 users for that topic
3. Assign ranks (1-100)
4. Return leaderboard entries

**Query:**
```sql
SELECT username, elo
FROM user_topic_elo
JOIN users ON user_topic_elo.user_id = users.id
WHERE topic = $1
ORDER BY elo DESC
LIMIT 100
```

**File location:** `crates/backend/src/api/leaderboard.rs:15`

---

## Authentication

### JWT Tokens

**File:** `crates/backend/src/auth/jwt.rs`

**Claims:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // user_id
    pub username: String,
    pub exp: i64,         // expiration timestamp
}
```

**Token Creation:**
```rust
pub fn create_token(
    user_id: &str,
    username: &str,
    secret: &str,
    expiry_hours: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(expiry_hours))
        .unwrap()
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
```

**Token Verification:**
```rust
pub fn verify_token(
    token: &str,
    secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
```

**File location:** `crates/backend/src/auth/jwt.rs`

---

### Auth Middleware

**File:** `crates/backend/src/auth/middleware.rs`

**AuthUser Extractor:**
```rust
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub username: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts.headers.get("Authorization")
            .ok_or(ApiError::Unauthorized)?;

        // Parse "Bearer {token}"
        let token = auth_header.to_str()
            .map_err(|_| ApiError::Unauthorized)?
            .strip_prefix("Bearer ")
            .ok_or(ApiError::Unauthorized)?;

        // Verify token
        let config = parts.extensions.get::<Arc<Config>>()
            .ok_or(ApiError::InternalServerError)?;

        let claims = verify_token(token, &config.jwt_secret)
            .map_err(|_| ApiError::Unauthorized)?;

        Ok(AuthUser {
            user_id: Uuid::parse_str(&claims.sub)
                .map_err(|_| ApiError::Unauthorized)?,
            username: claims.username,
        })
    }
}
```

**Usage:**
```rust
async fn protected_handler(
    auth: AuthUser,  // Automatically extracts and validates
) -> Result<Json<Response>, ApiError> {
    // auth.user_id and auth.username available here
}
```

**File location:** `crates/backend/src/auth/middleware.rs:12`

---

## Password Hashing

### Argon2

**File:** `crates/backend/src/api/auth.rs`

**Hash Password:**
```rust
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, rand_core::OsRng};

let salt = SaltString::generate(&mut OsRng);
let argon2 = Argon2::default();
let password_hash = argon2.hash_password(password.as_bytes(), &salt)
    .map_err(|_| ApiError::InternalServerError)?
    .to_string();
```

**Verify Password:**
```rust
use argon2::{Argon2, PasswordVerifier};
use argon2::password_hash::PasswordHash;

let parsed_hash = PasswordHash::new(&user.password_hash)
    .map_err(|_| ApiError::Unauthorized)?;

argon2.verify_password(password.as_bytes(), &parsed_hash)
    .map_err(|_| ApiError::Unauthorized)?;
```

**Algorithm:** Argon2id (recommended variant)
**Salt:** Randomly generated per password
**Output:** PHC string format (includes algorithm, params, salt, hash)

---

## Email System

### SMTP Configuration

**File:** `crates/backend/src/config.rs`

The email system uses SMTP for sending verification and password reset emails.

**Environment Variables:**
```bash
SMTP_HOST=smtp.gmail.com          # SMTP server
SMTP_PORT=587                     # TLS port (587 or 465)
SMTP_USERNAME=your@email.com      # SMTP username
SMTP_PASSWORD=your-app-password   # SMTP password
FRONTEND_URL=http://localhost:8080 # For email links
```

**Supported Providers:**
- Gmail (smtp.gmail.com:587)
- SendGrid (smtp.sendgrid.net:587)
- Mailgun (smtp.mailgun.org:587)
- Custom SMTP servers

### Email Templates

**File:** `crates/backend/src/email.rs`

#### Email Verification

```rust
pub fn send_verification_email(
    to: &str,
    username: &str,
    token: &str
) -> Result<(), EmailError> {
    let verify_url = format!(
        "{}/verify-email?token={}",
        env::var("FRONTEND_URL")?,
        token
    );

    let subject = "Verify your Locus account";
    let body = format!(
        "Hi {},\n\n\
        Please verify your email by clicking:\n{}\n\n\
        This link expires in 1 hour.\n\n\
        If you didn't create this account, ignore this email.",
        username, verify_url
    );

    send_email(to, subject, &body)
}
```

**Flow:**
1. User registers → Backend generates verification token
2. Backend inserts token into `email_verifications` table
3. Backend sends email with verification link
4. User clicks link → Frontend calls `/api/auth/verify-email/{token}`
5. Backend validates token, marks email as verified, deletes token

#### Password Reset

```rust
pub fn send_password_reset_email(
    to: &str,
    username: &str,
    token: &str
) -> Result<(), EmailError> {
    let reset_url = format!(
        "{}/reset-password?token={}",
        env::var("FRONTEND_URL")?,
        token
    );

    let subject = "Reset your Locus password";
    let body = format!(
        "Hi {},\n\n\
        You requested a password reset. Click here to reset:\n{}\n\n\
        This link expires in 1 hour.\n\n\
        If you didn't request this, ignore this email.",
        username, reset_url
    );

    send_email(to, subject, &body)
}
```

**Flow:**
1. User requests reset → Backend generates reset token
2. Backend inserts token into `password_resets` table (1h expiry)
3. Backend sends email with reset link
4. User clicks link → Frontend shows password reset form
5. User submits new password → Backend validates token, updates password, deletes token

### Token Management

**Token Generation:**
```rust
use rand::Rng;

fn generate_token() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    hex::encode(bytes) // 64-character hex string
}
```

**Token Storage:**
```sql
-- Email verifications
CREATE TABLE email_verifications (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  email VARCHAR(255) NOT NULL,
  token VARCHAR(64) UNIQUE NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Password resets
CREATE TABLE password_resets (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token VARCHAR(64) UNIQUE NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Token Validation:**
```rust
// Check token exists and not expired (1 hour)
let verification = sqlx::query_as::<_, EmailVerification>(
    "SELECT * FROM email_verifications
     WHERE token = $1 AND created_at > NOW() - INTERVAL '1 hour'"
)
.bind(&token)
.fetch_one(&pool)
.await?;
```

### Error Handling

**Email Send Failures:**
- Log error but don't fail registration (graceful degradation)
- User can request resend via `/api/auth/resend-verification`
- Monitor email sending in production logs

**Token Expiry:**
- Tokens expire after 1 hour
- User sees clear error message
- Can request new token via resend endpoint

### Security Considerations

**Token Security:**
- 256-bit random tokens (64 hex chars)
- Single-use (deleted after verification/reset)
- Time-limited (1 hour expiry)
- HTTPS-only links in production

**Email Validation:**
- Check email format before sending
- Prevent email enumeration (same response for valid/invalid)
- Rate limit resend requests (3 per 15 minutes)

**SMTP Security:**
- Use TLS (port 587) or SSL (port 465)
- Store SMTP credentials in environment variables
- Never log passwords or tokens
- Use app-specific passwords for Gmail

### Testing Email System

**Local Development (No Real Emails):**
```rust
// Mock email sender for development
#[cfg(debug_assertions)]
fn send_email(to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
    println!("=== EMAIL ===");
    println!("To: {}", to);
    println!("Subject: {}", subject);
    println!("Body:\n{}", body);
    println!("=============");
    Ok(())
}
```

**Production (Real SMTP):**
```rust
#[cfg(not(debug_assertions))]
fn send_email(to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
    let smtp_host = env::var("SMTP_HOST")?;
    let smtp_port = env::var("SMTP_PORT")?.parse()?;
    let username = env::var("SMTP_USERNAME")?;
    let password = env::var("SMTP_PASSWORD")?;

    // Use lettre crate for actual SMTP
    // ... implementation
}
```

**Testing Checklist:**
- Verify email received within 1 minute
- Check link format correct
- Test token expiry (wait 1 hour)
- Test resend functionality
- Verify spam folder if not in inbox

---

## Rate Limiting

### tower-governor Implementation

**File:** `crates/backend/src/main.rs`

Rate limiting protects authentication endpoints from brute-force attacks and abuse.

**Configuration:**
```rust
use tower_governor::{
    governor::GovernorConfigBuilder,
    key_extractor::SmartIpKeyExtractor,
    GovernorLayer,
};

// Registration: 5 requests per 15 minutes
let register_config = GovernorConfigBuilder::default()
    .per_second(1)
    .burst_size(5)
    .key_extractor(SmartIpKeyExtractor)
    .finish()
    .unwrap();

// Login: 10 requests per 15 minutes
let login_config = GovernorConfigBuilder::default()
    .per_second(1)
    .burst_size(10)
    .key_extractor(SmartIpKeyExtractor)
    .finish()
    .unwrap();

// Password reset: 5 requests per 15 minutes
let reset_config = GovernorConfigBuilder::default()
    .per_second(1)
    .burst_size(5)
    .key_extractor(SmartIpKeyExtractor)
    .finish()
    .unwrap();
```

### Per-Endpoint Limits

**Auth Endpoints:**
```rust
Router::new()
    .route("/api/auth/register", post(register_handler))
    .layer(GovernorLayer { config: Arc::clone(&register_config) })

    .route("/api/auth/login", post(login_handler))
    .layer(GovernorLayer { config: Arc::clone(&login_config) })

    .route("/api/auth/request-password-reset", post(request_reset_handler))
    .layer(GovernorLayer { config: Arc::clone(&reset_config) })

    .route("/api/auth/resend-verification", post(resend_handler))
    .layer(GovernorLayer { config: Arc::clone(&resend_config) })
```

**Rate Limit Table:**

| Endpoint | Limit | Window | Purpose |
|----------|-------|--------|---------|
| `/api/auth/register` | 5 req | 15 min | Prevent spam accounts |
| `/api/auth/login` | 10 req | 15 min | Prevent brute-force |
| `/api/auth/request-password-reset` | 5 req | 15 min | Prevent email bombing |
| `/api/auth/resend-verification` | 3 req | 15 min | Prevent email spam |
| `/api/submit` | 60 req | 1 min | Prevent answer farming |
| `/api/problem` | 100 req | 1 min | Prevent scraping |

### Key Extraction Strategy

**SmartIpKeyExtractor:**
- Extracts client IP from request
- Checks `X-Forwarded-For` header (proxy support)
- Falls back to socket address
- Handles IPv4 and IPv6

**Considerations:**
- Users behind NAT share same IP (may hit limits together)
- VPN users may change IPs (bypass limits)
- Future: Consider user-based rate limiting for authenticated endpoints

### Response Format

**Rate Limited Response:**
```http
HTTP/1.1 429 Too Many Requests
Retry-After: 900
X-RateLimit-Limit: 5
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1640995200

{
  "error": "Too many requests. Please try again in 15 minutes."
}
```

**Headers:**
- `Retry-After`: Seconds until limit resets
- `X-RateLimit-Limit`: Total requests allowed
- `X-RateLimit-Remaining`: Requests remaining
- `X-RateLimit-Reset`: Unix timestamp when limit resets

### Monitoring Rate Limits

**Logging:**
```rust
// Log rate limit hits
tracing::warn!(
    ip = %ip_addr,
    endpoint = %req.uri(),
    "Rate limit exceeded"
);
```

**Metrics (Future):**
- Count of 429 responses per endpoint
- Distribution of requests per IP
- Alert on unusual spike in rate limit hits

### Bypassing Limits (Admin/Testing)

**Whitelist IPs (Future):**
```rust
const WHITELIST_IPS: &[&str] = &[
    "127.0.0.1",  // Localhost
    "10.0.0.1",   // Internal admin
];

fn is_whitelisted(ip: &IpAddr) -> bool {
    WHITELIST_IPS.contains(&ip.to_string().as_str())
}
```

**Disable in Development:**
```rust
#[cfg(debug_assertions)]
let rate_limit_enabled = false;

#[cfg(not(debug_assertions))]
let rate_limit_enabled = true;

if rate_limit_enabled {
    router.layer(GovernorLayer { config });
}
```

### Tuning Rate Limits

**Increase Limits:**
- High-traffic production environment
- Legitimate users hitting limits
- After implementing user-based limiting

**Decrease Limits:**
- Under attack or abuse
- Resource constraints
- Suspicious activity patterns

**Per-User Limits (Future):**
```rust
// Authenticated users get higher limits
let user_config = GovernorConfigBuilder::default()
    .per_second(2)
    .burst_size(20)
    .key_extractor(UserIdKeyExtractor)  // Use user_id instead of IP
    .finish()
    .unwrap();
```

### Testing Rate Limiting

**Manual Testing:**
```bash
# Exceed login limit (10 requests)
for i in {1..15}; do
  curl -X POST http://localhost:3000/api/auth/login \
    -H "Content-Type: application/json" \
    -d '{"email":"test@example.com","password":"wrong"}' \
    -w "\nStatus: %{http_code}\n"
done

# Should see 429 after 10 requests
```

**Automated Testing:**
```rust
#[tokio::test]
async fn test_rate_limiting() {
    let app = create_test_app().await;

    // Send 11 requests (limit is 10)
    for i in 0..11 {
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/auth/login")
                    .body(Body::from(r#"{"email":"test@test.com","password":"test"}"#))
                    .unwrap()
            )
            .await
            .unwrap();

        if i < 10 {
            assert_ne!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        } else {
            assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        }
    }
}
```

---

## ELO System

### elo.rs

**File:** `crates/backend/src/elo.rs`

**Constants:**
```rust
const K_FACTOR: f64 = 32.0;
```

**Expected Score Calculation:**
```rust
pub fn calculate_expected_score(player_elo: i32, opponent_elo: i32) -> f64 {
    1.0 / (1.0 + 10.0_f64.powf((opponent_elo - player_elo) as f64 / 400.0))
}
```

**Formula:**
```
E = 1 / (1 + 10^((opponent - player) / 400))
```

**ELO Change Calculation:**
```rust
pub fn calculate_elo_change(
    player_elo: i32,
    expected_score: f64,
    actual_score: f64,
) -> f64 {
    K_FACTOR * (actual_score - expected_score)
}
```

**Formula:**
```
Δ = K × (S - E)

K = 32 (constant)
S = actual score (1.0 for win, 0.0 for loss)
E = expected score
```

**Example:**
```rust
// User (1500) vs Problem (1400)
let expected = calculate_expected_score(1500, 1400);  // 0.64
let elo_change = calculate_elo_change(1500, 0.64, 1.0);  // +11.5
let new_elo = 1500 + 11.5 = 1511

// If user got it wrong
let elo_change = calculate_elo_change(1500, 0.64, 0.0);  // -20.5
let new_elo = 1500 - 20.5 = 1479
```

**Tests:**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_expected_score_equal() {
        assert!((calculate_expected_score(1500, 1500) - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_elo_gain_on_win() {
        let change = calculate_elo_change(1500, 0.5, 1.0);
        assert!(change > 0.0);
    }
}
```

**File location:** `crates/backend/src/elo.rs`

---

## Grading System

### grader.rs

**File:** `crates/backend/src/grader.rs`

**MVP Implementation:**
```rust
pub fn grade_answer(user_input: &str, answer_key: &str) -> bool {
    let normalized_input = normalize(user_input);
    let normalized_answer = normalize(answer_key);

    normalized_input == normalized_answer
}

fn normalize(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}
```

**Current Behavior:**
- Removes all whitespace
- Converts to lowercase
- Exact string match

**Limitations:**
- Doesn't recognize mathematical equivalence
- Order matters: `x+1` ≠ `1+x`
- Factored forms must match exactly

**Future Enhancement:**
```rust
// With SymEngine WASM
pub fn grade_answer_symbolic(user_input: &str, answer_key: &str) -> bool {
    let user_expr = symengine::parse(user_input)?;
    let answer_expr = symengine::parse(answer_key)?;

    symengine::equals(user_expr, answer_expr)
}
```

**Tests:**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_exact_match() {
        assert!(grade_answer("x+1", "x+1"));
    }

    #[test]
    fn test_whitespace_insensitive() {
        assert!(grade_answer("x + 1", "x+1"));
    }

    #[test]
    fn test_case_insensitive() {
        assert!(grade_answer("X+1", "x+1"));
    }
}
```

**File location:** `crates/backend/src/grader.rs`

---

## Database Models

### User Model

**File:** `crates/backend/src/models/user.rs`

**Struct:**
```rust
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}
```

**Methods:**

#### create
```rust
pub async fn create(
    db: &PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error>
```

#### find_by_email
```rust
pub async fn find_by_email(
    db: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error>
```

#### find_by_id
```rust
pub async fn find_by_id(
    db: &PgPool,
    id: Uuid,
) -> Result<Option<User>, sqlx::Error>
```

#### get_elo_for_topic
```rust
pub async fn get_elo_for_topic(
    db: &PgPool,
    user_id: Uuid,
    topic: &str,
) -> Result<i32, sqlx::Error>
```

Uses PostgreSQL function `get_user_elo()`.

#### update_elo_for_topic
```rust
pub async fn update_elo_for_topic(
    db: &PgPool,
    user_id: Uuid,
    topic: &str,
    new_elo: i32,
) -> Result<(), sqlx::Error>
```

Uses PostgreSQL function `update_user_elo()`.

#### get_all_elos
```rust
pub async fn get_all_elos(
    db: &PgPool,
    user_id: Uuid,
) -> Result<HashMap<String, i32>, sqlx::Error>
```

Returns map of topic → ELO for all topics user has ratings in.

#### leaderboard
```rust
pub async fn leaderboard(
    db: &PgPool,
    topic: &str,
) -> Result<Vec<(String, i32)>, sqlx::Error>
```

Returns top 100 users for a topic as (username, elo) tuples.

---

### Problem Model

**File:** `crates/backend/src/models/problem.rs`

**Struct:**
```rust
pub struct Problem {
    pub id: Uuid,
    pub question_latex: String,
    pub answer_key: String,
    pub difficulty: i32,
    pub main_topic: Option<String>,
    pub subtopic: Option<String>,
    pub grading_mode: String,
}
```

**Methods:**

#### get_random
```rust
pub async fn get_random(
    db: &PgPool,
    main_topic: Option<&str>,
    subtopics: &[String],
) -> Result<Problem, sqlx::Error>
```

Gets random problem matching filters.

**Query Logic:**
- If no filters: random from all problems
- If topic only: random from that topic
- If topic + subtopics: random from topic WHERE subtopic IN (list)

#### find_by_id
```rust
pub async fn find_by_id(
    db: &PgPool,
    id: Uuid,
) -> Result<Option<Problem>, sqlx::Error>
```

#### create
```rust
pub async fn create(
    db: &PgPool,
    question_latex: &str,
    answer_key: &str,
    difficulty: i32,
    main_topic: Option<&str>,
    subtopic: Option<&str>,
    grading_mode: &str,
) -> Result<Problem, sqlx::Error>
```

---

### Attempt Model

**File:** `crates/backend/src/models/attempt.rs`

**Struct:**
```rust
pub struct Attempt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub problem_id: Uuid,
    pub user_input: String,
    pub is_correct: bool,
    pub elo_before: i32,
    pub elo_after: i32,
    pub time_taken_ms: i32,
    pub created_at: DateTime<Utc>,
    pub main_topic: Option<String>,
}
```

**Methods:**

#### create
```rust
pub async fn create(
    db: &PgPool,
    user_id: Uuid,
    problem_id: Uuid,
    user_input: &str,
    is_correct: bool,
    elo_before: i32,
    elo_after: i32,
    time_taken_ms: i32,
    main_topic: Option<&str>,
) -> Result<Attempt, sqlx::Error>
```

#### get_user_attempts
```rust
pub async fn get_user_attempts(
    db: &PgPool,
    user_id: Uuid,
    limit: i64,
) -> Result<Vec<Attempt>, sqlx::Error>
```

#### get_user_stats
```rust
pub async fn get_user_stats(
    db: &PgPool,
    user_id: Uuid,
) -> Result<UserStats, sqlx::Error>

pub struct UserStats {
    pub total_attempts: i64,
    pub correct_attempts: i64,
    pub accuracy: f64,
}
```

---

## Error Handling

### ApiError

**File:** `crates/backend/src/api/mod.rs`

**Enum:**
```rust
#[derive(Debug)]
pub enum ApiError {
    DatabaseError(sqlx::Error),
    Unauthorized,
    NotFound,
    BadRequest(String),
    InternalServerError,
}
```

**HTTP Mapping:**
```rust
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized".to_string(),
            ),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                "Not found".to_string(),
            ),
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                msg,
            ),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        (status, Json(json!({"error": message}))).into_response()
    }
}
```

---

## Logging and Tracing

### Configuration

**File:** `crates/backend/src/main.rs`

```rust
use tracing_subscriber;

tracing_subscriber::fmt()
    .with_env_filter(
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"))
    )
    .init();
```

**Log Levels:**
- ERROR - Critical errors
- WARN - Warnings
- INFO - General info (default)
- DEBUG - Debug info
- TRACE - Detailed traces

**Environment Variable:**
```bash
RUST_LOG=debug cargo run -p locus-backend
```

### Request Tracing

```rust
use tower_http::trace::TraceLayer;

Router::new()
    .layer(TraceLayer::new_for_http())
```

Logs:
- Request method and path
- Response status code
- Request duration

---

## Testing

### Unit Tests

```bash
cargo test -p locus-backend
```

**Test Modules:**
- `crates/backend/src/elo.rs` - ELO calculations
- `crates/backend/src/grader.rs` - Answer grading
- `crates/backend/src/auth/jwt.rs` - JWT operations

### Integration Tests

Future: Test full request/response cycle with test database.

---

## Performance Optimization

### Database Connection Pool

- Reuses connections
- Max 5 concurrent connections
- Lazy connection establishment

### Query Optimization

- Indexed queries on frequently accessed columns
- Prepared statements via SQLx
- Batch operations where possible

### Caching

Future:
- Redis for leaderboards
- In-memory cache for problems
- Session storage

---

## Security Best Practices

**Implemented:**
- JWT for stateless auth
- Argon2id for password hashing
- Parameterized SQL queries
- CORS configuration
- UUID primary keys
- Rate limiting (tower-governor)
- Email verification
- Password reset tokens (1h expiry)

**Future:**
- Enhanced input validation
- CSRF protection
- HTTPS enforcement
- Security headers (CSP, HSTS)
- SQL injection prevention audit
- XSS protection audit

---

## Deployment

### Building for Production

```bash
cargo build --release -p locus-backend
```

**Binary:** `target/release/locus-backend`

### Running in Production

```bash
DATABASE_URL=postgres://... \
JWT_SECRET=very-long-random-secret \
./target/release/locus-backend
```

### Docker Deployment

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release -p locus-backend

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5
COPY --from=builder /app/target/release/locus-backend /usr/local/bin/
CMD ["locus-backend"]
```

---

## Future Enhancements

- **WebSocket Support** - Real-time features
- **Redis Caching** - Performance optimization
- **Admin Panel** - Problem management
- **Analytics** - User behavior tracking
- **Push Notifications** - Real-time alerts
- **User-based Rate Limiting** - More granular control
- **Email Templates** - HTML emails with styling
- **2FA/MFA** - Multi-factor authentication
