# Locus - Competitive Math Platform

A competitive mathematics platform with ELO-based ranking. Built with a full Rust stack:
- **Frontend**: Leptos (CSR mode) compiled to WASM
- **Backend**: Axum with PostgreSQL
- **Math Engine**: SymEngine WASM (prepared for integration)

## Features

- **Practice Mode**: Instant client-side grading for learning
- **Ranked Mode**: Server-verified answers with ELO rating changes
- **Leaderboard**: Track your ranking against other players
- **Topics**: Algebra, Calculus, Linear Algebra

## Quick Start

### Prerequisites

- Rust 1.75+
- Docker & Docker Compose
- [Trunk](https://trunkrs.dev/) (for frontend development)

```bash
# Install trunk
cargo install trunk
```

### Development Setup

**Quick start** (single command):
```bash
./dev.sh
```

This starts PostgreSQL, backend, and frontend together.

**Or manually:**

1. **Start the database**:
```bash
docker compose up -d
```

2. **Run the backend** (terminal 1):
```bash
cargo run -p locus-backend
```

3. **Run the frontend** (terminal 2):
```bash
cd crates/frontend
trunk serve
```

4. Open http://localhost:8080 in your browser

### Environment Variables

Copy `.env.example` to `.env` and adjust as needed:

```bash
cp .env.example .env
```

Key variables:
- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret for JWT token signing (change in production!)
- `PORT`: Backend server port (default: 3000)

## Project Structure

```
locus/
├── Cargo.toml                    # Workspace root
├── docker-compose.yml            # PostgreSQL
├── symengine.js/                 # SymEngine WASM (for future integration)
├── crates/
│   ├── common/                   # Shared types (API models)
│   ├── frontend/                 # Leptos CSR app
│   │   ├── src/
│   │   │   ├── main.rs          # App entry point
│   │   │   ├── api.rs           # HTTP client
│   │   │   ├── grader.rs        # Client-side grading
│   │   │   ├── symengine.rs     # SymEngine bindings (scaffolding)
│   │   │   ├── pages/           # Route pages
│   │   │   └── components/      # UI components
│   │   └── index.html
│   └── backend/                  # Axum server
│       ├── src/
│       │   ├── main.rs          # Server entry point
│       │   ├── api/             # API handlers
│       │   ├── auth/            # JWT authentication
│       │   ├── models/          # Database models
│       │   ├── elo.rs           # ELO calculation
│       │   └── grader.rs        # Server-side grading
│       └── migrations/          # SQL migrations
└── content-gen/                  # Python problem generator
```

## API Endpoints

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | /api/auth/register | No | Create account |
| POST | /api/auth/login | No | Get JWT token |
| GET | /api/problem | Opt | Get problem (with/without answer) |
| POST | /api/submit | Yes | Submit answer, get ELO update |
| GET | /api/leaderboard | No | Top 100 users |
| GET | /api/user/me | Yes | Current user profile |

## Generating Problems

Use the Python script to generate additional problems:

```bash
cd content-gen
pip install -r requirements.txt
python generate.py --topic algebra --count 50 --output algebra.sql
```

## Testing

```bash
cargo test
```

## License

MIT
