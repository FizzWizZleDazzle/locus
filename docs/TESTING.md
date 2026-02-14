# Testing Guide

## Current State

WARNING: **No tests currently exist in this codebase.** This is our #1 priority gap.

This document outlines the testing strategy and provides examples to guide future test development.

## Testing Strategy

### 1. Unit Tests

#### Grading System (`crates/common/tests/grader_test.rs`)

**Priority:** P0 - Grading correctness is critical for platform credibility

**Test Coverage:**
- Two-stage equivalence validation (symbolic + numerical)
- Mode enforcement (Factor, Expand, Equivalent)
- Edge cases: trig identities, logarithms, infinity, NaN, complex expressions
- Test points coverage and numerical stability
- MathJSON conversion accuracy

**Example Tests:**
```rust
#[cfg(test)]
mod tests {
    use common::grader::{check_answer_expr, GradingMode};

    #[test]
    fn test_equivalent_expressions() {
        let answer = "x^2 - 1";

        // Same expression should pass
        assert!(check_answer_expr("x^2 - 1", answer, GradingMode::Equivalent).unwrap());

        // Factored form should pass in Equivalent mode
        assert!(check_answer_expr("(x+1)*(x-1)", answer, GradingMode::Equivalent).unwrap());

        // Reordered should pass
        assert!(check_answer_expr("x*x - 1", answer, GradingMode::Equivalent).unwrap());

        // Different signs should fail
        assert!(check_answer_expr("1 - x^2", answer, GradingMode::Equivalent).is_err());

        // Completely different should fail
        assert!(check_answer_expr("x + 1", answer, GradingMode::Equivalent).is_err());
    }

    #[test]
    fn test_factor_mode_enforcement() {
        let answer = "(x+1)*(x-1)";

        // Exact factored form should pass
        assert!(check_answer_expr("(x+1)*(x-1)", answer, GradingMode::Factor).unwrap());

        // Reordered factors should pass
        assert!(check_answer_expr("(x-1)*(x+1)", answer, GradingMode::Factor).unwrap());

        // Expanded form should fail
        assert!(check_answer_expr("x^2 - 1", answer, GradingMode::Factor).is_err());
    }

    #[test]
    fn test_expand_mode_enforcement() {
        let answer = "x^2 - 1";

        // Expanded form should pass
        assert!(check_answer_expr("x^2 - 1", answer, GradingMode::Expand).unwrap());

        // Factored form should fail
        assert!(check_answer_expr("(x+1)*(x-1)", answer, GradingMode::Expand).is_err());
    }

    #[test]
    fn test_trig_identities() {
        // sin^2(x) + cos^2(x) = 1
        assert!(check_answer_expr("sin(x)^2 + cos(x)^2", "1", GradingMode::Equivalent).unwrap());

        // tan(x) = sin(x)/cos(x)
        assert!(check_answer_expr("sin(x)/cos(x)", "tan(x)", GradingMode::Equivalent).unwrap());
    }

    #[test]
    fn test_logarithm_properties() {
        // log(a*b) = log(a) + log(b)
        assert!(check_answer_expr("log(x) + log(y)", "log(x*y)", GradingMode::Equivalent).unwrap());

        // log(a^b) = b*log(a)
        assert!(check_answer_expr("2*log(x)", "log(x^2)", GradingMode::Equivalent).unwrap());
    }

    #[test]
    fn test_edge_cases() {
        // Division by zero should be handled
        assert!(check_answer_expr("1/0", "infinity", GradingMode::Equivalent).is_err());

        // Undefined expressions
        assert!(check_answer_expr("0/0", "NaN", GradingMode::Equivalent).is_err());

        // Very large numbers
        assert!(check_answer_expr("10^100", "1e100", GradingMode::Equivalent).unwrap());
    }
}
```

#### ELO Calculation (`crates/backend/tests/elo_test.rs`)

**Priority:** P0 - ELO accuracy affects user experience and fairness

**Test Coverage:**
- Expected score calculation (E = 1/(1 + 10^((Rb-Ra)/400)))
- Rating updates (win/loss)
- Time multiplier validation (1.0x - 1.5x)
- K-factor application (K=32)
- Edge cases: new users, extreme ratings, equal ratings

**Example Tests:**
```rust
#[cfg(test)]
mod tests {
    use backend::elo::{calculate_expected_score, calculate_new_elo, get_time_multiplier};

    #[test]
    fn test_expected_score_equal_ratings() {
        let player_elo = 1500;
        let opponent_elo = 1500;

        let expected = calculate_expected_score(player_elo, opponent_elo);
        assert!((expected - 0.5).abs() < 0.001); // Should be 50%
    }

    #[test]
    fn test_expected_score_advantage() {
        let player_elo = 1700;
        let opponent_elo = 1500;

        let expected = calculate_expected_score(player_elo, opponent_elo);
        assert!(expected > 0.75); // Strong favorite should have >75% win probability
    }

    #[test]
    fn test_elo_increase_on_win() {
        let old_elo = 1500;
        let problem_difficulty = 1500;
        let is_correct = true;
        let time_multiplier = 1.0;

        let new_elo = calculate_new_elo(old_elo, problem_difficulty, is_correct, time_multiplier);
        assert!(new_elo > old_elo); // Should increase
        assert!(new_elo <= old_elo + 32); // Should not increase more than K-factor
    }

    #[test]
    fn test_elo_decrease_on_loss() {
        let old_elo = 1500;
        let problem_difficulty = 1500;
        let is_correct = false;
        let time_multiplier = 1.0;

        let new_elo = calculate_new_elo(old_elo, problem_difficulty, is_correct, time_multiplier);
        assert!(new_elo < old_elo); // Should decrease
        assert!(new_elo >= old_elo - 32); // Should not decrease more than K-factor
    }

    #[test]
    fn test_time_multiplier_fast_solve() {
        let solve_time = 10.0; // 10 seconds
        let expected_time = 60.0; // 60 seconds expected

        let multiplier = get_time_multiplier(solve_time, expected_time);
        assert!(multiplier > 1.0); // Fast solve should get bonus
        assert!(multiplier <= 1.5); // Max bonus is 1.5x
    }

    #[test]
    fn test_time_multiplier_slow_solve() {
        let solve_time = 120.0; // 120 seconds
        let expected_time = 60.0; // 60 seconds expected

        let multiplier = get_time_multiplier(solve_time, expected_time);
        assert_eq!(multiplier, 1.0); // No penalty for slow solves
    }

    #[test]
    fn test_elo_with_time_bonus() {
        let old_elo = 1500;
        let problem_difficulty = 1500;
        let is_correct = true;
        let time_multiplier = 1.5; // Fast solve

        let new_elo = calculate_new_elo(old_elo, problem_difficulty, is_correct, time_multiplier);
        let new_elo_normal = calculate_new_elo(old_elo, problem_difficulty, is_correct, 1.0);

        assert!(new_elo > new_elo_normal); // Bonus should increase gain
    }
}
```

#### SymEngine FFI (`crates/common/tests/symengine_test.rs`)

**Priority:** P0 - Safety critical (can cause segfaults)

**Test Coverage:**
- Type safety guards (is_a_Number before number_is_zero)
- Parse/expand/substitute operations
- Memory safety (no leaks)
- WASM vs native behavior parity
- Thread safety in native builds

**Example Tests:**
```rust
#[cfg(test)]
mod tests {
    use common::symengine::{Expr, is_a_Number, number_is_zero, expand, subs2};

    #[test]
    fn test_type_guard_safety() {
        // Symbol is NOT a number
        let symbol = Expr::parse("x").unwrap();
        assert!(!is_a_Number(&symbol.ptr));

        // Number IS a number
        let number = Expr::parse("42").unwrap();
        assert!(is_a_Number(&number.ptr));
    }

    #[test]
    fn test_number_is_zero_with_guard() {
        let zero = Expr::parse("0").unwrap();

        // Safe: check type first
        if is_a_Number(&zero.ptr) {
            assert!(number_is_zero(&zero.ptr) != 0);
        }

        let one = Expr::parse("1").unwrap();
        if is_a_Number(&one.ptr) {
            assert!(number_is_zero(&one.ptr) == 0);
        }
    }

    #[test]
    #[should_panic] // This test documents unsafe behavior
    fn test_number_is_zero_without_guard_panics() {
        let symbol = Expr::parse("x").unwrap();

        // UNSAFE: This will segfault in native builds
        let _ = number_is_zero(&symbol.ptr);
    }

    #[test]
    fn test_parse_and_to_string() {
        let expr = Expr::parse("x^2 + 2*x + 1").unwrap();
        let s = expr.to_string();
        assert!(s.contains("x"));
    }

    #[test]
    fn test_expand_operation() {
        let expr = Expr::parse("(x+1)*(x-1)").unwrap();
        let expanded = expand(&expr);
        let s = expanded.to_string();

        // Should expand to x^2 - 1
        assert!(s.contains("x**2") || s.contains("x^2"));
    }

    #[test]
    fn test_substitution() {
        let expr = Expr::parse("x + 1").unwrap();
        let x = Expr::parse("x").unwrap();
        let value = Expr::parse("5").unwrap();

        let result = subs2(&expr, &x, &value);
        let s = result.to_string();
        assert_eq!(s, "6");
    }

    #[test]
    fn test_clone_safety() {
        let expr1 = Expr::parse("x^2").unwrap();
        let expr2 = expr1.clone();

        assert_eq!(expr1.to_string(), expr2.to_string());
    }
}
```

### 2. Integration Tests

#### API Endpoints (`crates/backend/tests/api_test.rs`)

**Priority:** P0 - API correctness is critical

**Test Coverage:**
- Auth flow: register → verify email → login → get token
- Problem retrieval (practice vs ranked)
- Answer submission → ELO update
- Leaderboard queries
- OAuth flows (mocked provider)
- Rate limiting enforcement

**Setup:**
```rust
// Test harness setup
use axum::Router;
use sqlx::PgPool;
use tower::ServiceExt;

async fn setup_test_app() -> (Router, PgPool) {
    // Create test database connection
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await.unwrap();

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();

    // Create app with test config
    let app = backend::create_app(pool.clone()).await;

    (app, pool)
}

#[tokio::test]
async fn test_register_login_flow() {
    let (app, pool) = setup_test_app().await;

    // 1. Register new user
    let register_req = RegisterRequest {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "SecurePass123!".to_string(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/register")
                .header("content-type", "application/json")
                .body(serde_json::to_string(&register_req).unwrap().into())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 2. Verify email (get token from database)
    let token = sqlx::query_scalar::<_, String>(
        "SELECT token FROM email_verifications WHERE email = $1"
    )
    .bind("test@example.com")
    .fetch_one(&pool)
    .await
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/api/auth/verify-email/{}", token))
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 3. Login
    let login_req = LoginRequest {
        email: "test@example.com".to_string(),
        password: "SecurePass123!".to_string(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("content-type", "application/json")
                .body(serde_json::to_string(&login_req).unwrap().into())
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: LoginResponse = serde_json::from_slice(
        &hyper::body::to_bytes(response.into_body()).await.unwrap()
    ).unwrap();

    assert!(!body.token.is_empty());
}
```

#### Database Operations (`crates/backend/tests/db_test.rs`)

**Test Coverage:**
- User CRUD operations
- ELO updates per topic
- Attempt logging
- Email verification token lifecycle
- Password reset token expiry
- OAuth account linking

### 3. Frontend Tests (Future)

**Note:** Requires test harness setup for Leptos

Planned coverage:
- Component rendering
- Form validation
- API call handling
- Error states
- Navigation

## Test Environment Setup

### Database

```bash
# Create test database
docker run -d --name locus-test-db \
  -e POSTGRES_PASSWORD=test \
  -e POSTGRES_DB=locus_test \
  -p 5433:5432 \
  postgres:16

# Wait for startup
sleep 5

# Set environment variable
export TEST_DATABASE_URL=postgres://postgres:test@localhost:5433/locus_test
```

### Environment Variables

Create `.env.test`:
```bash
DATABASE_URL=postgres://postgres:test@localhost:5433/locus_test
JWT_SECRET=test_secret_minimum_32_characters_long_for_hs256
ENVIRONMENT=test
FACTORY_API_KEY=test-factory-key
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USERNAME=test@example.com
SMTP_PASSWORD=test-password
FRONTEND_URL=http://localhost:8080
```

### Test Fixtures

Create `crates/backend/tests/fixtures/`:

**users.sql:**
```sql
INSERT INTO users (id, username, email, password_hash, email_verified, created_at)
VALUES
  ('00000000-0000-0000-0000-000000000001', 'alice', 'alice@example.com', '$argon2id$...', true, NOW()),
  ('00000000-0000-0000-0000-000000000002', 'bob', 'bob@example.com', '$argon2id$...', true, NOW());
```

**problems.sql:**
```sql
INSERT INTO problems (id, question_latex, answer_key, difficulty, main_topic, subtopic, grading_mode)
VALUES
  ('00000000-0000-0000-0000-000000000101', 'What is $2 + 2$?', '4', 800, 'arithmetic', 'addition_subtraction', 'equivalent'),
  ('00000000-0000-0000-0000-000000000102', 'Solve: $x^2 = 4$', '2', 1200, 'algebra1', 'quadratic_equations', 'equivalent');
```

**attempts.sql:**
```sql
INSERT INTO attempts (id, user_id, problem_id, user_input, is_correct, time_taken, elo_before, elo_after, created_at)
VALUES
  (gen_random_uuid(), '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000101', '4', true, 10.5, 1500, 1516, NOW());
```

## Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test --package common
cargo test --package backend

# Specific module
cargo test --package common --lib grader

# Specific test
cargo test test_equivalent_expressions

# With output (see println! statements)
cargo test -- --nocapture

# Backend integration tests (requires test DB)
export TEST_DATABASE_URL=postgres://postgres:test@localhost:5433/locus_test
cargo test --package backend --test api_test

# Run tests in parallel (default)
cargo test

# Run tests serially (for DB tests)
cargo test -- --test-threads=1
```

## Writing Your First Test

### Step 1: Create Test File

```bash
# Unit test (in same file)
# Add #[cfg(test)] mod tests { ... } to your .rs file

# Integration test (separate file)
touch crates/common/tests/my_feature_test.rs
```

### Step 2: Import Dependencies

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use common::grader::{check_answer_expr, GradingMode};
}
```

### Step 3: Write Test Function

```rust
#[test]
fn test_my_feature() {
    // Arrange
    let input = "x+1";
    let answer = "1+x";

    // Act
    let result = check_answer_expr(input, answer, GradingMode::Equivalent);

    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap());
}
```

### Step 4: Run Test

```bash
cargo test test_my_feature
```

## Coverage Goals

| Component | Target Coverage | Priority | Status |
|-----------|----------------|----------|--------|
| Grading system | 90% | P0 | Not started |
| ELO calculation | 90% | P0 | Not started |
| SymEngine FFI | 80% | P0 | Not started |
| Auth handlers | 80% | P0 | Not started |
| API endpoints | 70% | P1 | Not started |
| Database models | 80% | P1 | Not started |
| Frontend components | 60% | P2 | Not started |

## CI/CD Integration (Future)

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_PASSWORD: test
          POSTGRES_DB: locus_test
        ports:
          - 5433:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run tests
        env:
          TEST_DATABASE_URL: postgres://postgres:test@localhost:5433/locus_test
        run: cargo test --all

      - name: Generate coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml

      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

## Critical Test Cases Needed

### P0 (Must Have)
- [ ] Grading equivalence: `x+1` vs `1+x`
- [ ] Factor mode rejection: Reject `x^2-1` when answer is `(x+1)(x-1)`
- [ ] Expand mode rejection: Reject `(x+1)(x-1)` when answer is `x^2-1`
- [ ] SymEngine safety: `number_is_zero` without type guard → panic
- [ ] JWT expiry: Expired token → 401
- [ ] Rate limiting: Burst requests → 429
- [ ] Email verification: Expired token → error
- [ ] OAuth linking: Duplicate provider → error

### P1 (Should Have)
- [ ] ELO calculation accuracy across full range (800-2400)
- [ ] Time bonus calculation edge cases
- [ ] Database transaction rollback on error
- [ ] Password reset token expiry
- [ ] Problem difficulty filtering
- [ ] Leaderboard pagination

### P2 (Nice to Have)
- [ ] Frontend component rendering
- [ ] Form validation
- [ ] Navigation flows
- [ ] Error message display

## Test-Driven Development

When adding new features:

1. **Write test first** (failing test)
2. **Implement feature** (make test pass)
3. **Refactor** (keep test passing)
4. **Document** (update this file)

Example workflow:
```bash
# 1. Write failing test
cargo test test_new_grading_mode  # FAIL

# 2. Implement feature
# ... edit code ...

# 3. Run test
cargo test test_new_grading_mode  # PASS

# 4. Commit both test and implementation
git add .
git commit -m "Add new grading mode with tests"
```

## Troubleshooting

### "Database connection failed"
- Ensure test database is running: `docker ps | grep locus-test-db`
- Check TEST_DATABASE_URL environment variable
- Run migrations: `sqlx migrate run --database-url $TEST_DATABASE_URL`

### "Test timeout"
- Some integration tests may be slow - increase timeout with `#[tokio::test(flavor = "multi_thread")]`
- Check for infinite loops or deadlocks

### "Flaky tests"
- Avoid time-dependent tests - use fixed timestamps
- Avoid parallel DB tests - use `-- --test-threads=1`
- Clean up test data between runs

### "SymEngine segfault in tests"
- Ensure type guards are used before number_is_zero()
- Check that SYMENGINE_LOCK mutex is acquired
- Verify allocator bridge in WASM builds

## Resources

- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio Testing Guide](https://tokio.rs/tokio/topics/testing)
- [SQLx Testing Examples](https://github.com/launchbadge/sqlx/tree/main/tests)
- [Axum Testing Guide](https://github.com/tokio-rs/axum/tree/main/examples)

## Next Steps

1. **Start with unit tests** - Lowest barrier to entry
2. **Add grading tests** - Highest priority
3. **Set up test DB** - Required for integration tests
4. **Implement CI** - Automate test execution
5. **Track coverage** - Measure progress toward goals
