# Development Guide

Complete guide for setting up and developing Locus locally.

## Prerequisites

### Required Software

**Rust (1.75+)**
```bash
# Install via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

**Trunk (WASM Build Tool)**
```bash
cargo install trunk
```

**Docker & Docker Compose**
- [Docker Desktop](https://www.docker.com/products/docker-desktop/) (macOS, Windows)
- Docker Engine + Docker Compose (Linux)

```bash
# Verify installations
cargo --version
trunk --version
docker --version
docker compose version
```

### Optional Tools

**PostgreSQL Client (psql)**
```bash
# macOS
brew install postgresql

# Ubuntu/Debian
sudo apt-get install postgresql-client

# For connecting to database directly
psql postgres://locus:locus_dev_password@localhost:5433/locus
```

**Python 3.10+ (for content-gen)**
```bash
cd content-gen
pip install -r requirements.txt
```

---

## Quick Start

### One-Command Setup

```bash
./dev.sh
```

This script:
1. Checks dependencies
2. Creates `.env` from `.env.example`
3. Starts PostgreSQL in Docker
4. Runs backend with migrations
5. Starts frontend dev server

**Access:**
- Frontend: http://localhost:8080
- Backend API: http://localhost:3000
- Database: localhost:5433

**Stop:**
Press `Ctrl+C` to stop all servers.

---

## Manual Setup

If you prefer to run components separately:

### 1. Environment Configuration

```bash
cp .env.example .env
```

**Edit `.env` if needed:**
```bash
# Database
DATABASE_URL=postgres://locus:locus_dev_password@localhost:5433/locus

# Server
HOST=0.0.0.0
PORT=3000

# JWT
JWT_SECRET=change-this-in-production-use-a-long-random-string
JWT_EXPIRY_HOURS=24
```

### 2. Start PostgreSQL

```bash
docker compose up -d
```

**Check status:**
```bash
docker compose ps
```

**View logs:**
```bash
docker compose logs -f
```

**Stop database:**
```bash
docker compose down
```

**Reset database (destructive):**
```bash
docker compose down -v  # Removes volumes
docker compose up -d
```

### 3. Run Backend

```bash
# From project root
cargo run -p locus-backend
```

**Or with logging:**
```bash
RUST_LOG=debug cargo run -p locus-backend
```

**Backend runs on:** http://localhost:3000

**Migrations run automatically on startup**

### 4. Run Frontend

```bash
# In a new terminal
cd crates/frontend
trunk serve
```

**Frontend runs on:** http://localhost:8080

**Auto-reload:** Trunk watches for file changes and recompiles.

---

## Project Structure

```
locus/
├── Cargo.toml                    # Workspace root
├── .env                          # Environment variables (git-ignored)
├── .env.example                  # Template for .env
├── docker-compose.yml            # PostgreSQL container
├── dev.sh                        # Development startup script
│
├── crates/
│   ├── common/                   # Shared types
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   │
│   ├── backend/                  # Axum API server
│   │   ├── Cargo.toml
│   │   ├── migrations/           # SQL migrations
│   │   └── src/
│   │       ├── main.rs
│   │       ├── api/
│   │       ├── auth/
│   │       └── models/
│   │
│   └── frontend/                 # Leptos WASM app
│       ├── Cargo.toml
│       ├── Trunk.toml
│       ├── index.html
│       └── src/
│           ├── main.rs
│           ├── pages/
│           └── components/
│
├── content-gen/                  # Python problem generator
│   ├── generate.py
│   └── requirements.txt
│
├── symengine.js/                 # SymEngine WASM (future)
│   ├── build_wasm.sh
│   └── src/
│
└── docs/                         # Documentation
    ├── ARCHITECTURE.md
    ├── API.md
    ├── DATABASE.md
    ├── FRONTEND.md
    ├── BACKEND.md
    ├── DEVELOPMENT.md (this file)
    ├── DEPLOYMENT.md
    └── LOOKUP_TABLE.md
```

---

## Development Workflow

### Making Changes

**Backend Changes:**
1. Edit files in `crates/backend/src/`
2. Backend auto-recompiles on save (if using `cargo watch`)
3. Restart backend manually or use: `cargo install cargo-watch && cargo watch -x 'run -p locus-backend'`

**Frontend Changes:**
1. Edit files in `crates/frontend/src/`
2. Trunk auto-recompiles and hot-reloads
3. Browser refreshes automatically

**Common Changes:**
1. Edit `crates/common/src/lib.rs`
2. Rebuild both backend and frontend

**Database Changes:**
1. Create new migration in `crates/backend/migrations/`
2. Name: `NNN_description.sql` (NNN = next number)
3. Restart backend to run migration

### Running Tests

**All tests:**
```bash
cargo test
```

**Backend only:**
```bash
cargo test -p locus-backend
```

**Frontend only:**
```bash
cargo test -p locus-frontend
```

**Specific test:**
```bash
cargo test test_elo_calculation
```

**With output:**
```bash
cargo test -- --nocapture
```

### Code Formatting

**Format code:**
```bash
cargo fmt
```

**Check formatting:**
```bash
cargo fmt -- --check
```

### Linting

**Run Clippy:**
```bash
cargo clippy
```

**Fix warnings:**
```bash
cargo clippy --fix
```

---

## Database Management

### Connecting to Database

```bash
psql postgres://locus:locus_dev_password@localhost:5433/locus
```

**Common queries:**
```sql
-- List tables
\dt

-- Describe table
\d users

-- View users
SELECT * FROM users;

-- View ELO ratings
SELECT * FROM user_topic_elo;

-- View problems
SELECT id, LEFT(question_latex, 50), difficulty, main_topic FROM problems;

-- View attempts
SELECT * FROM attempts ORDER BY created_at DESC LIMIT 10;
```

### Creating Migrations

1. **Create file:**
```bash
touch crates/backend/migrations/005_new_feature.sql
```

2. **Write SQL:**
```sql
-- Add your schema changes
ALTER TABLE problems ADD COLUMN hints TEXT[];
CREATE INDEX idx_problems_hints ON problems USING gin(hints);
```

3. **Restart backend** to apply migration

4. **Rollback (manual):**
```sql
-- Write rollback commands manually
ALTER TABLE problems DROP COLUMN hints;
DROP INDEX idx_problems_hints;
```

### Seeding Data

**Generate problems:**
```bash
cd content-gen
python generate.py --topic calculus --count 100 --output calculus_100.sql
```

**Import to database:**
```bash
psql postgres://locus:locus_dev_password@localhost:5433/locus < content-gen/calculus_100.sql
```

**Or add to migration:**
```bash
cat content-gen/calculus_100.sql >> crates/backend/migrations/006_seed_calculus.sql
```

---

## Frontend Development

### Trunk Commands

**Dev server:**
```bash
trunk serve
```

**Production build:**
```bash
trunk build --release
```

**Clean build artifacts:**
```bash
trunk clean
```

**Watch and rebuild:**
```bash
trunk watch
```

### Debugging Frontend

**Browser Console:**
- Open DevTools (F12)
- Check console for errors
- Network tab for API calls

**Rust Panics:**
```rust
// Add to main.rs
console_error_panic_hook::set_once();

// Now panics show in browser console
```

**Logging:**
```rust
use web_sys::console;

console::log_1(&"Debug message".into());
console::error_1(&format!("Error: {:?}", err).into());
```

### Styling

**Tailwind CSS:**
- Loaded via CDN in `index.html`
- All utility classes available
- [Tailwind Docs](https://tailwindcss.com/docs)

**Custom CSS:**
Add `<style>` tags to `index.html` or component `view!` macros.

---

## Backend Development

### Adding New Endpoints

1. **Define handler in `crates/backend/src/api/`:**
```rust
pub async fn my_endpoint(
    State(db): State<PgPool>,
    auth: AuthUser,
    Json(req): Json<MyRequest>,
) -> Result<Json<MyResponse>, ApiError> {
    // Implementation
    Ok(Json(MyResponse { ... }))
}
```

2. **Add route in `crates/backend/src/api/mod.rs`:**
```rust
Router::new()
    .route("/api/my-endpoint", post(my_endpoint))
```

3. **Define types in `crates/common/src/lib.rs`:**
```rust
#[derive(Serialize, Deserialize)]
pub struct MyRequest { ... }

#[derive(Serialize, Deserialize)]
pub struct MyResponse { ... }
```

4. **Add frontend API call in `crates/frontend/src/api.rs`:**
```rust
pub async fn my_api_call(token: &str, data: &MyRequest) -> Result<MyResponse, String> {
    // HTTP request
}
```

### Debugging Backend

**Logging:**
```rust
use tracing::{info, warn, error, debug, trace};

info!("User {} logged in", username);
debug!("Database query: {:?}", query);
error!("Failed to connect: {}", err);
```

**Set log level:**
```bash
RUST_LOG=debug cargo run -p locus-backend
RUST_LOG=locus_backend=trace cargo run -p locus-backend
```

**Database query debugging:**
```rust
let result = sqlx::query!("SELECT * FROM users")
    .fetch_all(&db)
    .await?;

println!("Query result: {:?}", result);
```

---

## Common Issues

### Port Already in Use

**Backend (port 3000):**
```bash
# Find process
lsof -i :3000

# Kill it
kill -9 <PID>
```

**Frontend (port 8080):**
```bash
lsof -i :8080
kill -9 <PID>
```

**Database (port 5433):**
```bash
docker compose down
docker compose up -d
```

### Database Connection Errors

**Check if database is running:**
```bash
docker compose ps
```

**Restart database:**
```bash
docker compose restart
```

**Check logs:**
```bash
docker compose logs postgres
```

**Test connection:**
```bash
psql postgres://locus:locus_dev_password@localhost:5433/locus -c "SELECT 1;"
```

### Frontend Not Loading

**Clear browser cache:**
- Hard refresh: Ctrl+Shift+R (Windows/Linux) or Cmd+Shift+R (macOS)

**Check trunk output:**
- Look for compile errors in terminal

**Check API proxy:**
- Verify backend is running on port 3000
- Check Trunk.toml proxy configuration

### Migration Errors

**Migration already applied:**
- SQLx tracks applied migrations in `_sqlx_migrations` table
- Don't modify existing migrations
- Create new migration to fix issues

**Rollback migration:**
```bash
# SQLx doesn't support automatic rollback
# Manually write and run SQL to undo changes
psql postgres://locus:locus_dev_password@localhost:5433/locus < rollback.sql
```

**Reset database:**
```bash
docker compose down -v
docker compose up -d
cargo run -p locus-backend  # Re-runs all migrations
```

---

## Performance Tips

### Backend

**Compilation speed:**
```bash
# Use mold linker (Linux)
sudo apt install mold
RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo build

# Or add to .cargo/config.toml:
# [target.x86_64-unknown-linux-gnu]
# linker = "clang"
# rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

**Faster rebuilds:**
```bash
cargo install cargo-watch
cargo watch -x 'run -p locus-backend'
```

### Frontend

**Faster WASM builds:**
```bash
# Use trunk with --release for smaller bundles
trunk serve --release
```

**Reduce bundle size:**
- Enable wasm-opt in Trunk.toml
- Use `wee_alloc` for smaller allocator

---

## Git Workflow

### Branching

```bash
# Create feature branch
git checkout -b feature/my-feature

# Make changes and commit
git add .
git commit -m "Add my feature"

# Push to remote
git push -u origin feature/my-feature
```

### Commit Messages

**Format:**
```
<type>: <short description>

<optional longer description>

Co-Authored-By: Claude Sonnet 4.5 (1M context) <noreply@anthropic.com>
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `refactor` - Code refactoring
- `test` - Adding tests
- `chore` - Maintenance

**Examples:**
```
feat: Add practice mode with client-side grading

fix: Correct ELO calculation for edge cases

docs: Update API documentation for submit endpoint

refactor: Extract auth middleware to separate module
```

---

## Code Style Guide

### Rust

**Follow standard Rust conventions:**
- Use `cargo fmt` for formatting
- Follow `cargo clippy` suggestions
- Use descriptive variable names
- Add comments for complex logic
- Write tests for new functionality

**Naming:**
- `snake_case` for functions and variables
- `PascalCase` for types and traits
- `SCREAMING_SNAKE_CASE` for constants

**Error Handling:**
- Use `Result<T, E>` for fallible operations
- Propagate errors with `?` operator
- Provide context with custom error types

### SQL

**Formatting:**
- Keywords in UPPERCASE
- Indentation for readability
- One column per line for long SELECT

**Example:**
```sql
SELECT
    id,
    username,
    email,
    created_at
FROM users
WHERE email = $1
ORDER BY created_at DESC
LIMIT 10;
```

---

## Environment Variables

### Development (.env)

```bash
DATABASE_URL=postgres://locus:locus_dev_password@localhost:5433/locus
HOST=0.0.0.0
PORT=3000
JWT_SECRET=dev-secret-change-in-production
JWT_EXPIRY_HOURS=24
RUST_LOG=info
```

### Production

**Never commit:**
- Real JWT secrets
- Production database credentials
- API keys

**Use environment-specific values:**
- Long random JWT_SECRET (32+ characters)
- Strong database password
- Appropriate RUST_LOG level (warn or error)

---

## Useful Commands Cheat Sheet

```bash
# Start everything
./dev.sh

# Backend
cargo run -p locus-backend
RUST_LOG=debug cargo run -p locus-backend
cargo test -p locus-backend
cargo watch -x 'run -p locus-backend'

# Frontend
cd crates/frontend && trunk serve
cd crates/frontend && trunk build --release
cargo test -p locus-frontend

# Database
docker compose up -d
docker compose down
docker compose down -v  # Remove volumes
docker compose logs -f postgres
psql postgres://locus:locus_dev_password@localhost:5433/locus

# Code quality
cargo fmt
cargo clippy
cargo test

# Generate problems
cd content-gen
python generate.py --topic calculus --count 50

# Git
git status
git add .
git commit -m "message"
git push
```

---

## Next Steps

After setting up:

1. **Explore the codebase**
   - Read through `crates/common/src/lib.rs` for data types
   - Check `crates/backend/src/api/` for endpoints
   - Look at `crates/frontend/src/pages/` for UI

2. **Make a small change**
   - Add a new problem via psql
   - Modify a frontend component
   - Add a new API endpoint

3. **Read the documentation**
   - [API Reference](API.md)
   - [Database Schema](DATABASE.md)
   - [Architecture Overview](ARCHITECTURE.md)

4. **Join the development**
   - Check GitHub issues for tasks
   - Pick a feature to implement
   - Submit a pull request

---

## Getting Help

**Resources:**
- [Rust Book](https://doc.rust-lang.org/book/)
- [Leptos Docs](https://leptos.dev/)
- [Axum Docs](https://docs.rs/axum/)
- [SQLx Docs](https://docs.rs/sqlx/)

**Community:**
- GitHub Issues for bugs
- Discussions for questions
