# Locus - Competitive Math Platform

A competitive mathematics platform with ELO-based ranking and email verification. Built with a full Rust stack:
- **Frontend**: Leptos (CSR mode) compiled to WASM with MathLive editor
- **Backend**: Axum with PostgreSQL and OAuth (Google/GitHub)
- **Math Engine**: SymEngine WASM/Native for symbolic math grading
- **Problem Factory**: Automated generation of 5,376 problems across topics

## Features

- **Email Verification**: Secure account creation with Resend
- **OAuth Login**: Google and GitHub authentication
- **Practice Mode**: Client-side grading with instant feedback
- **Ranked Mode**: Server-verified answers with ELO rating
- **Leaderboard**: Real-time rankings
- **Topics**: Algebra, Geometry, Calculus, Number Theory, Pre-Algebra
- **Grading Modes**: Equivalent, Expand, Factor
- **Calculator Levels**: Scientific to TI-84 level problems

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
├── Makefile                      # Deployment orchestration
├── dev.sh                        # Development startup script
├── Dockerfile                    # Backend production build
├── docker-compose.yml            # PostgreSQL for development
├── .env.production               # Production secrets (gitignored)
│
├── scripts/                      # Deployment scripts
│   ├── generate-secrets.sh       # Generate JWT/API secrets
│   ├── deploy-helm.sh            # Deploy backend to Kubernetes
│   ├── deploy-cloudflare-pages.sh # Deploy frontend to Pages
│   ├── load-data.sh              # Load problems + remove duplicates
│   └── check-status.sh           # Health check all services
│
├── helm/locus/                   # Kubernetes manifests
│   ├── templates/                # K8s resources (deployment, service, etc)
│   ├── values.yaml               # Default values
│   └── values.production.yaml    # Production overrides
│
├── docs/                         # Documentation
│   ├── RELEASE.md               # Complete deployment guide
│   ├── QUICKSTART.md            # Quick start guide
│   └── DEPLOYMENT.md            # Detailed deployment docs
│
├── crates/                       # Rust workspace
│   ├── common/                   # Shared types + SymEngine FFI
│   │   ├── src/
│   │   │   ├── lib.rs           # Shared types (Problem, User, etc)
│   │   │   ├── grader.rs        # Grading engine (ExprEngine trait)
│   │   │   ├── symengine.rs     # SymEngine FFI (WASM + native)
│   │   │   ├── mathjson.rs      # MathJSON → SymEngine converter
│   │   │   └── latex.rs         # LaTeX fallback converter
│   │   └── build.rs             # Conditional WASM/native linking
│   │
│   ├── frontend/                 # Leptos WASM app
│   │   ├── src/
│   │   │   ├── main.rs          # App entry + C allocator bridge
│   │   │   ├── api.rs           # HTTP client
│   │   │   ├── grader.rs        # Client-side grading
│   │   │   ├── oauth.rs         # OAuth flow handling
│   │   │   ├── pages/           # Route pages
│   │   │   └── components/      # UI components (MathInput, etc)
│   │   ├── index.html
│   │   └── Trunk.toml
│   │
│   └── backend/                  # Axum server
│       ├── src/
│       │   ├── main.rs          # Server entry point
│       │   ├── api/             # API handlers (problems, auth, etc)
│       │   ├── auth/            # JWT + OAuth providers
│       │   ├── models/          # Database models
│       │   ├── email.rs         # Email verification (Resend)
│       │   ├── elo.rs           # ELO calculation
│       │   └── grader.rs        # Server-side grading
│       └── migrations/          # SQL migrations
│
├── factory/                      # Problem generation system
│   ├── backend/                  # Rust problem generator
│   │   ├── src/                 # Generation logic by topic
│   │   └── exports/             # Generated SQL (5,376 problems)
│   └── QUICKSTART.md            # Factory usage guide
│
└── symengine.js/                 # SymEngine WASM build
    └── dist/wasm-unknown/lib/   # Precompiled WASM libraries
```

## API Endpoints

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | /api/auth/register | No | Create account + send verification |
| POST | /api/auth/verify-email | No | Verify email with token |
| POST | /api/auth/login | No | Get JWT token |
| GET | /api/auth/google | No | OAuth Google login |
| GET | /api/auth/github | No | OAuth GitHub login |
| GET | /api/topics | No | List all topics |
| GET | /api/problem | Opt | Get problem (practice/ranked) |
| POST | /api/submit | Yes | Submit answer, get ELO update |
| GET | /api/leaderboard | No | Top 100 users |
| GET | /api/user/me | Yes | Current user profile |
| POST | /api/user/resend-verification | Yes | Resend verification email |

## Production Deployment

**Quick deployment** (recommended):
```bash
make init              # Generate secrets
# Edit .env.production with OAuth keys, Resend API key, etc.
make tunnel-instructions  # Get Cloudflare tunnel token
make all              # Build + Deploy + Load data
```

See [RELEASE.md](RELEASE.md) for the complete deployment guide.

**Available commands:**
- `make build` - Build Docker image
- `make push` - Push to registry
- `make deploy` - Deploy backend + frontend
- `make data` - Load problems into database
- `make status` - Health check all services
- `make help` - Show all commands

## Generating Problems

The factory generates 5,376 problems automatically:

```bash
cd factory/backend
cargo run --release
```

Output: `factory/backend/exports/problems_import.sql`

Then load with:
```bash
make data
```

## Testing

```bash
cargo test
```

## License

IAUL - Custom license (see LICENSE file)
