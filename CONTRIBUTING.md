# Contributing to Locus

Thank you for your interest in contributing to Locus! This guide will help you set up your development environment and follow best practices.

## Development Setup

### Prerequisites

- Rust (latest stable)
- Docker and Docker Compose
- Trunk (`cargo install trunk`)
- PostgreSQL client tools (optional, for manual DB access)

### Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/locus.git
   cd locus
   ```

2. Run the development script:
   ```bash
   ./dev.sh
   ```

   This will:
   - Start PostgreSQL in Docker
   - Start the backend on http://localhost:3000
   - Start the frontend on http://localhost:8080
   - Configure environment variables automatically

3. Open http://localhost:8080 in your browser

### Environment Variables

The project uses environment-specific configuration to support both development and production environments.

#### Frontend Environment Variables

Set at **build time** via environment variables:

- `LOCUS_API_URL` - API base URL (e.g., `http://localhost:3000/api` for dev)
- `LOCUS_FRONTEND_URL` - Frontend base URL (e.g., `http://localhost:8080` for dev)
- `LOCUS_ENV` - Environment name (e.g., `development` or `production`)

These are automatically set by `dev.sh` for local development.

#### Backend Environment Variables

Set at **runtime** via `.env` file (see `.env.example`):

- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret key for JWT tokens (generate a secure random string)
- `SMTP_*` - Email configuration for verification/password reset
- OAuth provider credentials (optional)

## Environment Configuration Rules

**CRITICAL: Never hardcode production URLs in the codebase.**

### Frontend API Calls

**WRONG:**
```rust
let url = format!("https://api.locusmath.org/api/auth/{}", path);
```

**RIGHT:**
```rust
let url = format!("{}/auth/{}", crate::env::api_base(), path);
```

### Environment Configuration Module

The `crates/frontend/src/env.rs` module provides centralized environment configuration:

```rust
use crate::env;

// Get API base URL
let api_url = env::api_base();

// Get frontend base URL (for OAuth redirects, etc.)
let frontend_url = env::frontend_base();
```

### Why This Matters

Hardcoding production URLs breaks the development environment:
- Dev environment expects `http://localhost:3000/api`
- Production uses `https://api.locusmath.org/api`
- Hardcoded URLs cause OAuth failures, API errors, and confusion

### Pre-commit Hook

A pre-commit hook automatically checks for hardcoded production URLs:

```bash
# Configure git to use hooks (one-time setup)
git config core.hooksPath .githooks
```

The hook will block commits containing hardcoded URLs like:
- `https://api.locusmath.org`
- `https://locusmath.org`

**Exception:** The `env.rs` file is allowed to have these URLs as fallbacks.

## Code Style

### Rust

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Write tests for new functionality
- Document public APIs with doc comments

### Frontend (Leptos)

- Use reactive signals for state management
- Keep components focused and composable
- Handle errors gracefully with user-friendly messages

### Backend (Axum)

- Use middleware for cross-cutting concerns (auth, logging, rate limiting)
- Return proper HTTP status codes
- Validate input thoroughly
- Log errors with context

## Security Guidelines

### Authentication & Authorization

- Always use `RequireAuth` middleware for protected endpoints
- Never trust client-side data - validate on the backend
- Use rate limiting to prevent abuse

### Input Validation

- Validate all user input on the backend
- Sanitize data before database queries (use parameterized queries)
- Implement proper error handling without leaking sensitive information

### Secrets Management

- Never commit secrets to git
- Use `.env` for local development (gitignored)
- Use environment variables in production
- Generate secure random values for JWT_SECRET

## Testing

### Backend Tests

```bash
cargo test -p locus-backend
```

### Frontend Tests

```bash
cd crates/frontend
cargo test
```

### Integration Tests

```bash
cargo test --workspace
```

## Database Migrations

Migrations are handled automatically by the backend on startup using embedded SQL files.

To add a new migration:

1. Create a new SQL file in `crates/backend/migrations/`
2. Name it with a timestamp: `YYYYMMDD_HHMMSS_description.sql`
3. Write your migration SQL
4. Restart the backend - it will apply the migration automatically

## Git Workflow

1. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and commit:
   ```bash
   git add .
   git commit -m "Add feature: description"
   ```

   The pre-commit hook will run automatically to check for issues.

3. Push your branch:
   ```bash
   git push origin feature/your-feature-name
   ```

4. Create a pull request on GitHub

## Common Issues

### "env" module import error (WASM)

This is caused by SymEngine WASM build trying to import missing allocator functions. The frontend's `main.rs` provides C allocator bridges (`malloc`, `free`, etc.) to fix this.

If you see this error, ensure the allocator bridge code in `main.rs` is present.

### Database connection failed

1. Check if PostgreSQL is running: `docker ps | grep locus-db`
2. Check `.env` has correct `DATABASE_URL`
3. Restart the database: `docker restart locus-db`

### Frontend build fails

1. Clear trunk cache: `rm -rf crates/frontend/dist`
2. Check environment variables are set (run via `./dev.sh`)
3. Rebuild: `cd crates/frontend && trunk build`

### OAuth not working in dev

1. Verify `LOCUS_API_URL=http://localhost:3000/api` is set
2. Check browser console for errors
3. Ensure popup blockers are disabled
4. Check OAuth provider credentials in `.env`

## Resources

- [Leptos Documentation](https://leptos.dev/)
- [Axum Documentation](https://docs.rs/axum/)
- [SymEngine Documentation](https://github.com/symengine/symengine)
- [Rust Book](https://doc.rust-lang.org/book/)

## Questions?

If you have questions or run into issues:

1. Check existing [GitHub Issues](https://github.com/yourusername/locus/issues)
2. Create a new issue with details about your problem
3. Include relevant logs, error messages, and steps to reproduce

Thank you for contributing to Locus!
