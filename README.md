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

#### Backend (Runtime)

Copy `.env.example` to `.env` and adjust as needed:

```bash
cp .env.example .env
```

Key variables:
- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret for JWT token signing (change in production!)
- `PORT`: Backend server port (default: 3000)
- `SMTP_*`: Email configuration (Resend, SendGrid, etc.)
- `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`: OAuth credentials (optional)
- `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET`: OAuth credentials (optional)

#### Frontend (Build-time)

The frontend uses compile-time environment variables that are baked into the WASM binary:

- `LOCUS_API_URL`: API base URL (e.g., `http://localhost:3000/api` for dev)
- `LOCUS_FRONTEND_URL`: Frontend base URL (e.g., `http://localhost:8080` for dev)
- `LOCUS_ENV`: Environment name (e.g., `development` or `production`)

**Development**: These are automatically set by `./dev.sh`

**Production**: Set before building:
```bash
export LOCUS_API_URL=https://api.locusmath.org/api
export LOCUS_FRONTEND_URL=https://locusmath.org
export LOCUS_ENV=production
cd crates/frontend
trunk build --release
```

**Important**: Never hardcode production URLs in code. Always use `crate::env::api_base()` and `crate::env::frontend_base()`. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

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
├── docs/                         # Comprehensive documentation
│   ├── SUMMARY.md               # Documentation index
│   ├── ARCHITECTURE.md          # System architecture
│   ├── TESTING.md               # Test strategy
│   ├── GRADING.md               # Grading system details
│   ├── AUTHENTICATION.md        # Auth & OAuth flows
│   ├── SYMENGINE_FFI.md         # SymEngine safety guide
│   └── ... (17 documentation files total)
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

## Documentation

Comprehensive documentation is available in the `docs/` directory:

**Getting Started:**
- [DEVELOPMENT.md](docs/DEVELOPMENT.md) - Local development setup
- [SUMMARY.md](docs/SUMMARY.md) - Complete documentation index

**Architecture & Design:**
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - System architecture and data flow
- [GRADING.md](docs/GRADING.md) - Two-stage grading system
- [AUTHENTICATION.md](docs/AUTHENTICATION.md) - Auth flows (email, OAuth)

**Implementation Guides:**
- [BACKEND.md](docs/BACKEND.md) - Backend implementation (email, rate limiting, ELO)
- [FRONTEND.md](docs/FRONTEND.md) - Frontend components and pages
- [LEPTOS_PATTERNS.md](docs/LEPTOS_PATTERNS.md) - Leptos state management
- [SYMENGINE_FFI.md](docs/SYMENGINE_FFI.md) - SymEngine safety guide (CRITICAL)

**Reference:**
- [API.md](docs/API.md) - Complete API endpoint reference
- [DATABASE.md](docs/DATABASE.md) - Database schema and migrations
- [TESTING.md](docs/TESTING.md) - Testing strategy and examples

**Production:**
- [DEPLOYMENT.md](docs/DEPLOYMENT.md) - Production deployment guide
- [RELEASE.md](RELEASE.md) - Makefile-based deployment process

**Contributing:**
- [CONTRIBUTING.md](CONTRIBUTING.md) - Code contribution guidelines
- [CONTRIBUTING_DOCS.md](docs/CONTRIBUTING_DOCS.md) - Documentation guidelines

**Factory (Problem Generation):**
- [factory/README.md](factory/README.md) - Factory system overview
- [factory/DEVELOPER_GUIDE.md](factory/DEVELOPER_GUIDE.md) - Creating custom generators

**Total:** 17 documentation files, ~15,000 lines

## Testing

```bash
cargo test
```

See [TESTING.md](docs/TESTING.md) for comprehensive test strategy and examples.

## License

IAUL - Custom license (see LICENSE file)
