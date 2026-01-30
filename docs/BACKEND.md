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

**Future:**
- Rate limiting
- Input validation
- CSRF protection
- HTTPS enforcement
- Security headers

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
- **Rate Limiting** - DDoS protection
- **Admin Panel** - Problem management
- **Analytics** - User behavior tracking
- **Notifications** - Email/push notifications
