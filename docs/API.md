# API Reference

Complete documentation for all Locus REST API endpoints.

## Base URL

**Development:** `http://localhost:3000/api`
**Production:** `https://yourdomain.com/api`

## Authentication

Most endpoints require a JWT token obtained from login.

**Header:**
```
Authorization: Bearer {token}
```

**Token Expiry:** 24 hours (configurable via `JWT_EXPIRY_HOURS`)

---

## Endpoints

### Health Check

#### GET /api/health

Health check endpoint to verify server is running.

**Auth Required:** No

**Request:**
```bash
curl http://localhost:3000/api/health
```

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

---

## Authentication Endpoints

### Register

#### POST /api/auth/register

Create a new user account.

**Auth Required:** No

**Request Body:**
```json
{
  "username": "john_doe",
  "email": "john@example.com",
  "password": "SecurePassword123!"
}
```

**Validation:**
- `username`: 3-50 characters, alphanumeric + underscore
- `email`: Valid email format
- `password`: Minimum 8 characters (no other restrictions)

**Response (200 OK):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "username": "john_doe"
}
```

**Errors:**
- `400` - Invalid input (validation failed)
- `409` - Username or email already exists

**Example:**
```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "SecurePassword123!"
  }'
```

---

### Login

#### POST /api/auth/login

Authenticate and receive a JWT token.

**Auth Required:** No

**Request Body:**
```json
{
  "email": "john@example.com",
  "password": "SecurePassword123!"
}
```

**Response (200 OK):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "username": "john_doe"
}
```

**Errors:**
- `400` - Invalid input
- `401` - Invalid credentials

**Example:**
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "SecurePassword123!"
  }'
```

---

### Get Current User

#### GET /api/user/me

Get the profile of the currently authenticated user.

**Auth Required:** Yes

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "john_doe",
  "email": "john@example.com",
  "created_at": "2024-01-15T10:30:00Z",
  "elo_arithmetic": 1520,
  "elo_algebra1": 1485,
  "elo_geometry": 1500,
  "elo_algebra2": 1500,
  "elo_precalculus": 1500,
  "elo_calculus": 1560,
  "elo_multivariable_calculus": 1500,
  "elo_linear_algebra": 1500
}
```

**Errors:**
- `401` - Unauthorized (missing or invalid token)

**Example:**
```bash
curl http://localhost:3000/api/user/me \
  -H "Authorization: Bearer {token}"
```

---

## Problem Endpoints

### Get Problem

#### GET /api/problem

Get a random problem, optionally filtered by topic and subtopics.

**Auth Required:** Optional
- **Without auth (Practice mode):** Returns problem with answer
- **With auth (Ranked mode):** Returns problem without answer

**Query Parameters:**
- `main_topic` (optional) - Filter by topic: `Arithmetic`, `Algebra1`, `Geometry`, `Algebra2`, `Precalculus`, `Calculus`, `MultivariableCalculus`, `LinearAlgebra`
- `subtopics` (optional) - Comma-separated list of subtopics

**Response (200 OK - Practice Mode):**
```json
{
  "id": "660e8400-e29b-41d4-a716-446655440000",
  "question_latex": "Factor the expression: $x^2 - 5x + 6$",
  "answer_key": "(x-2)(x-3)",
  "difficulty": 1400,
  "main_topic": "Algebra1",
  "subtopic": "factoring_quadratics",
  "grading_mode": "Factor"
}
```

**Response (200 OK - Ranked Mode):**
```json
{
  "id": "660e8400-e29b-41d4-a716-446655440000",
  "question_latex": "Factor the expression: $x^2 - 5x + 6$",
  "answer_key": null,
  "difficulty": 1400,
  "main_topic": "Algebra1",
  "subtopic": "factoring_quadratics",
  "grading_mode": "Factor"
}
```

**Errors:**
- `404` - No problems found matching criteria

**Examples:**

Get any problem (practice mode):
```bash
curl http://localhost:3000/api/problem
```

Get Calculus problem (practice mode):
```bash
curl "http://localhost:3000/api/problem?main_topic=Calculus"
```

Get Algebra1 problem with specific subtopics (practice mode):
```bash
curl "http://localhost:3000/api/problem?main_topic=Algebra1&subtopics=factoring_quadratics,polynomials"
```

Get problem for ranked mode:
```bash
curl http://localhost:3000/api/problem \
  -H "Authorization: Bearer {token}"
```

---

## Submission Endpoints

### Submit Answer

#### POST /api/submit

Submit an answer to a problem in ranked mode. Calculates ELO change and records attempt.

**Auth Required:** Yes

**Request Body:**
```json
{
  "problem_id": "660e8400-e29b-41d4-a716-446655440000",
  "user_input": "(x-2)(x-3)",
  "time_taken_ms": 45000
}
```

**Fields:**
- `problem_id` - UUID of the problem being answered
- `user_input` - User's answer (string)
- `time_taken_ms` - Time taken in milliseconds (optional, defaults to 0)

**Response (200 OK):**
```json
{
  "is_correct": true,
  "correct_answer": "(x-2)(x-3)",
  "elo_change": 15,
  "new_elo": 1515,
  "problem_difficulty": 1400
}
```

**Fields:**
- `is_correct` - Whether the answer was correct
- `correct_answer` - The correct answer (always shown after submission)
- `elo_change` - Change in ELO rating (positive or negative)
- `new_elo` - User's new ELO for this topic
- `problem_difficulty` - The difficulty rating of the problem

**Errors:**
- `401` - Unauthorized (missing or invalid token)
- `404` - Problem not found
- `500` - Server error (database failure, etc.)

**Example:**
```bash
curl -X POST http://localhost:3000/api/submit \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{
    "problem_id": "660e8400-e29b-41d4-a716-446655440000",
    "user_input": "(x-2)(x-3)",
    "time_taken_ms": 45000
  }'
```

---

## Leaderboard Endpoints

### Get Leaderboard

#### GET /api/leaderboard

Get the top 100 users by ELO rating for a specific topic.

**Auth Required:** No

**Query Parameters:**
- `topic` (required) - One of: `Arithmetic`, `Algebra1`, `Geometry`, `Algebra2`, `Precalculus`, `Calculus`, `MultivariableCalculus`, `LinearAlgebra`

**Response (200 OK):**
```json
[
  {
    "rank": 1,
    "username": "math_wizard",
    "elo": 1850
  },
  {
    "rank": 2,
    "username": "calc_master",
    "elo": 1820
  },
  {
    "rank": 3,
    "username": "algebra_ace",
    "elo": 1795
  }
]
```

**Errors:**
- `400` - Invalid topic parameter

**Example:**
```bash
curl "http://localhost:3000/api/leaderboard?topic=Calculus"
```

---

## Data Models

### MainTopic Enum

```rust
pub enum MainTopic {
    Arithmetic,
    Algebra1,
    Geometry,
    Algebra2,
    Precalculus,
    Calculus,
    MultivariableCalculus,
    LinearAlgebra,
}
```

### Subtopics by Topic

**Arithmetic:**
- `basic_operations`
- `fractions`
- `decimals`
- `percentages`
- `order_of_operations`

**Algebra1:**
- `linear_equations`
- `inequalities`
- `graphing_lines`
- `systems_of_equations`
- `polynomials`
- `factoring_quadratics`
- `exponents`
- `radicals`

**Geometry:**
- `angles`
- `triangles`
- `quadrilaterals`
- `circles`
- `area_perimeter`
- `volume_surface_area`
- `pythagorean_theorem`

**Algebra2:**
- `quadratic_functions`
- `complex_numbers`
- `polynomial_functions`
- `rational_functions`
- `exponential_functions`
- `logarithmic_functions`

**Precalculus:**
- `trigonometry`
- `polar_coordinates`
- `sequences_series`
- `limits`
- `conic_sections`

**Calculus:**
- `limits_continuity`
- `derivatives`
- `applications_of_derivatives`
- `integrals`
- `applications_of_integrals`

**MultivariableCalculus:**
- `partial_derivatives`
- `multiple_integrals`
- `vector_calculus`
- `line_integrals`
- `surface_integrals`

**LinearAlgebra:**
- `matrices`
- `determinants`
- `vector_spaces`
- `eigenvalues_eigenvectors`
- `linear_transformations`

### GradingMode Enum

```rust
pub enum GradingMode {
    Equivalent,  // Mathematical equivalence (e.g., 2x + 4 = 2(x + 2))
    Factor,      // Must be in factored form
}
```

---

## Error Responses

All errors follow this format:

```json
{
  "error": "Error message description"
}
```

**Common HTTP Status Codes:**
- `200` - Success
- `400` - Bad Request (validation failed)
- `401` - Unauthorized (missing or invalid JWT)
- `404` - Not Found (resource doesn't exist)
- `409` - Conflict (duplicate username/email)
- `500` - Internal Server Error

---

## Rate Limiting

**Current:** No rate limiting implemented

**Future Considerations:**
- 100 requests per minute per IP
- 1000 requests per hour per user
- Higher limits for authenticated users

---

## Versioning

**Current Version:** v1 (implicit in `/api` prefix)

**Future Versioning:**
- `/api/v2/...` for breaking changes
- Maintain v1 for backward compatibility

---

## CORS Configuration

**Development:**
- Allowed origins: `http://localhost:8080`
- Allowed methods: GET, POST, PUT, DELETE
- Allowed headers: Authorization, Content-Type

**Production:**
- Configure based on deployment domain
- Use environment variables for allowed origins

---

## WebSocket Support

**Current:** Not implemented

**Future Plans:**
- Real-time leaderboard updates
- Live competitions
- Problem racing mode

---

## Response Times

**Expected Response Times:**
- Auth endpoints: < 200ms
- Problem retrieval: < 50ms
- Submit answer: < 100ms
- Leaderboard: < 100ms

**Database Query Performance:**
- Indexed queries on main_topic, subtopic
- Connection pooling for concurrent requests
