# Deployment

## Environment Variables

### Backend (runtime)

| Variable | Required | Default | Description |
|---|---|---|---|
| `DATABASE_URL` | yes | - | PostgreSQL connection string |
| `JWT_SECRET` | yes | - | JWT signing secret (32+ chars) |
| `HOST` | no | 0.0.0.0 | Bind address |
| `PORT` | no | 3000 | Bind port |
| `ENVIRONMENT` | no | development | `development` or `production` |
| `ALLOWED_ORIGINS` | no | - | Comma-separated CORS origins |
| `JWT_EXPIRY_HOURS` | no | 24 | JWT token lifetime |
| `SKIP_MIGRATIONS` | no | false | Skip SQLx migrations on startup |
| `GOOGLE_CLIENT_ID` | no | - | Google OAuth client ID |
| `GOOGLE_CLIENT_SECRET` | no | - | Google OAuth client secret |
| `GITHUB_CLIENT_ID` | no | - | GitHub OAuth client ID |
| `GITHUB_CLIENT_SECRET` | no | - | GitHub OAuth client secret |
| `OAUTH_REDIRECT_BASE` | no | - | Base URL for OAuth callbacks |
| `RESEND_API_KEY` | no | - | Resend email service API key |
| `RESEND_FROM_EMAIL` | no | - | Sender email address |
| `RESEND_FROM_NAME` | no | - | Sender display name |
| `FRONTEND_BASE_URL` | no | - | Frontend URL (for email links) |
| `MAX_DB_CONNECTIONS` | no | 10 dev / 50 prod | PostgreSQL connection pool size |
| `RATE_LIMIT_AUTH_PER_15MIN` | no | 5 | Auth endpoint rate limit |
| `RATE_LIMIT_LOGIN_PER_15MIN` | no | 10 | Login rate limit |
| `RATE_LIMIT_SENSITIVE_PER_15MIN` | no | 5 | Sensitive endpoint rate limit (forgot-password, resend-verification, reset-password) |
| `RATE_LIMIT_GENERAL_PER_MIN` | no | 1000 | General rate limit |

### Frontend (compile-time)

| Variable | Default | Description |
|---|---|---|
| `LOCUS_API_URL` | `https://api.locusmath.org/api` | Backend API base URL |
| `LOCUS_FRONTEND_URL` | - | Frontend base URL |

## Development Setup

### Option 1: Dev container (recommended)

Open in VS Code Dev Containers or GitHub Codespaces. Everything is pre-installed:
- Rust toolchain + WASM target
- Trunk, cargo-watch
- Native and WASM SymEngine
- PostgreSQL via Docker-in-Docker

Post-create script (`.devcontainer/setup.sh`) validates tools, seeds the database, and creates `.env` files.

### Option 2: Local with dev.sh

```bash
./dev.sh
```

Starts:
- PostgreSQL on `localhost:5433` (via docker-compose)
- Backend on `localhost:3000` (via cargo-watch with hot reload)
- Frontend on `localhost:8080` (via trunk serve)

Prerequisites: `cargo`, `trunk`, `cargo-watch`, `docker`, SymEngine installed at `/usr/local/lib/`.

### Option 3: Full Docker

```bash
docker compose -f docker/docker-compose.dev.yml up
```

Builds from `docker/Dockerfile.dev`. Mounts workspace with cargo caches. Same ports as dev.sh.

### Problem Generation

```bash
cargo run --bin dsl-cli -- generate problems/calculus/derivative_rules.yaml -n 100
cargo run --bin dsl-cli -- ai 'algebra1/quadratic_formula' -n 5 -j 3
```

Output is JSONL; pipe to `scripts/import_jsonl.py` to bulk-load into PostgreSQL.

## Production Deployment

### Initial Setup

```bash
make init                   # Generate production secrets in .env
# Edit .env — configure PRODUCTION_ variables (OAuth, email, domains)
```

### Build and Deploy

```bash
make build                  # Build Docker image
make push                   # Push to ghcr.io/fizzwizzledazzle
make deploy-backend         # Deploy to Kubernetes via Helm
make deploy-frontend        # Build trunk + deploy to Cloudflare Pages
make data                   # Load problems into database
make status                 # Verify health
```

Or all at once: `make all`

### Docker Image

Multi-stage build (`docker/Dockerfile`):
1. `rust:1.93.0-slim` builder: compiles SymEngine from source, builds backend in release mode
2. `debian:trixie-slim` runtime: copies binary + migrations, runs as non-root `locus` user on port 28743

### Kubernetes

Helm chart in `helm/locus/`. Key resources:
- Deployment with backend container + Cloudflare Tunnel sidecar
- Secret with all env vars from `.env` (PRODUCTION_ prefixed vars)
- ClusterIP Service (port 80 -> 28743)
- Ingress with nginx + cert-manager TLS
- Optional HPA for autoscaling
- Optional PostgreSQL StatefulSet

```bash
# Override defaults
helm upgrade --install locus helm/locus/ \
  -n locus \
  -f helm/locus/values.production.yaml \
  --set backend.image.tag=v1.2.3
```

### Cloudflare Pages

Frontend deployed via wrangler:

```bash
# Automated
make deploy-frontend

# Manual
LOCUS_API_URL=https://api.yourdomain.com/api trunk build --release
npx wrangler pages deploy dist --project-name locus --branch main
```

## Scripts

| Script | Purpose |
|---|---|
| `scripts/generate-secrets.sh` | Generate JWT, API key, DB password via openssl |
| `scripts/deploy-helm.sh` | Validate config and deploy to Kubernetes |
| `scripts/deploy-cloudflare-pages.sh` | Build frontend and deploy to Cloudflare Pages |
| `scripts/check-status.sh` | Check pods, tunnel, frontend, DB, OAuth, email |
| `scripts/load-data.sh` | Load problems into database |
| `scripts/seed_test_topic.sql` | Seed test topic with one problem per answer type |
| `scripts/fix-katex-rendering.sql` | Fix KaTeX rendering issues in problem data |

## Docker Compose Files

| File | Purpose | Services |
|---|---|---|
| `docker/docker-compose.yml` | Dev database only | PostgreSQL (port 5433) |
| `docker/docker-compose.dev.yml` | Full dev environment | PostgreSQL + app (ports 3000, 8080) |
| `docker/docker-compose.prod.yml` | Production-like local | PostgreSQL + backend (port 28743) |

## Port Reference

| Port | Service | Environment |
|---|---|---|
| 3000 | Backend (Axum) | Development |
| 8080 | Frontend (Trunk) | Development |
| 5433 | PostgreSQL | Development |
| 5432 | PostgreSQL | Production |
| 28743 | Backend (Docker) | Production |
| 8090 | Services backend | Both |
| 8082 | Status frontend (Trunk) | Development |

## Services (status page)

### Services Backend Env Vars

| Variable | Required | Default | Description |
|---|---|---|---|
| `DATABASE_URL` | yes | - | PostgreSQL connection string |
| `JWT_SECRET` | yes | - | JWT signing secret (must match main backend) |
| `PORT` | no | 8090 | Bind port |
| `ENVIRONMENT` | no | development | `development` or `production` |
| `ALLOWED_ORIGINS` | no | localhost:8082 | Comma-separated CORS origins |
| `HEALTH_CHECK_URL` | no | api.locusmath.org/api/health | URL to monitor |
| `HEALTH_CHECK_INTERVAL_SECS` | no | 300 | Health check interval (seconds) |

### Env Files

All environment configuration lives in a single `.env` file. Services dev vars (`PORT`, `ALLOWED_ORIGINS`) are passed as inline overrides in `dev.sh`. Production services vars use the `PRODUCTION_SERVICES_` prefix in `.env`.

### Deployment

```bash
# Backend (Docker + K8s)
make build-services-backend
make push-services-backend

# Status frontend (Cloudflare Pages)
make deploy-status-frontend
```

### Cloudflare Setup

- Tunnel: `community-api.locusmath.org` -> K8s services pod
- Pages: `status.locusmath.org` (locus-status project)

> **Note:** The `forum_*` DB tables and the `locus-forum` Cloudflare Pages project are deprecated. The forum was migrated to GitHub Discussions; tables remain in the DB but no code reads them.
