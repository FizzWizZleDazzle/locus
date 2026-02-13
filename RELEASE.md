# Locus Release Guide

This guide explains how to deploy Locus to production using the consolidated Makefile-based workflow.

## Quick Start

For a complete first-time deployment:

```bash
# 1. Generate secrets
make init

# 2. Edit .env.production with:
#    - OAuth credentials (Google/GitHub)
#    - Resend API key
#    - Frontend URLs
#    - Cloudflare Tunnel token (run: make tunnel-instructions)

# 3. Complete deployment
make all
```

That's it! The `make all` command will:
- Build the Docker image
- Push to registry
- Deploy backend to Kubernetes
- Deploy frontend to Cloudflare Pages
- Load problem data
- Run health checks

## Commands Reference

### Setup Commands

#### `make init`
First-time setup that generates secure secrets in `.env.production`.

**What it does:**
- Generates JWT_SECRET, API_KEY_SECRET, and DB_PASSWORD
- Creates `.env.production` with template configuration
- Shows what still needs manual configuration

**After running:**
1. Edit `.env.production` and set:
   - `RESEND_API_KEY` (from resend.com)
   - `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET`
   - `GITHUB_CLIENT_ID` and `GITHUB_CLIENT_SECRET`
   - `FRONTEND_BASE_URL` (your actual domain)
   - `ALLOWED_ORIGINS` (same as frontend URL)
   - `OAUTH_REDIRECT_BASE` (same as frontend URL)

2. Update database user password:
   ```bash
   psql -h YOUR_DB_HOST -U postgres -d locus -c "ALTER USER locus_srv PASSWORD 'generated_password';"
   ```

#### `make tunnel-instructions`
Shows instructions for setting up Cloudflare Tunnel.

**What it shows:**
- How to install cloudflared CLI
- How to create and configure the tunnel
- How to get the tunnel token
- Where to add the token in `.env.production`

**Note:** The Cloudflare Tunnel runs as a sidecar container in Kubernetes (managed by Helm). You only need to get the token and add it to `.env.production`.

### Build Commands

#### `make build`
Builds the Docker image for the backend.

**What it does:**
- Builds image from `Dockerfile`
- Tags as `ghcr.io/fizzwizzledazzle/locus:latest`
- Shows build progress

**Requirements:**
- Docker installed and running
- Insecure registry configured for `ghcr.io/fizzwizzledazzle`

#### `make push`
Pushes the Docker image to the registry.

**What it does:**
- Pushes `locus:latest` to the registry
- Verifies upload succeeded

**Requirements:**
- Image must be built first (`make build`)
- Registry must be accessible

### Deployment Commands

#### `make deploy-backend`
Deploys the backend to Kubernetes using Helm.

**What it does:**
- Validates `.env.production` exists and has required secrets
- Creates `locus` namespace if needed
- Deploys/upgrades Helm release
- Waits for pods to be ready
- Shows status and useful commands

**Requirements:**
- `kubectl` and `helm` installed
- Kubernetes cluster accessible
- `.env.production` configured
- Docker image pushed to registry

#### `make deploy-frontend`
Deploys the frontend to Cloudflare Pages.

**What it does:**
- Builds frontend with Trunk (release mode)
- Deploys to Cloudflare Pages using Wrangler
- Shows deployment URL and next steps

**Requirements:**
- `wrangler` CLI installed (`npm install -g wrangler`)
- Logged into Cloudflare (`wrangler login`)
- `trunk` installed for building

#### `make deploy`
Deploys both backend and frontend.

**What it does:**
- Runs `deploy-backend`
- Runs `deploy-frontend`
- Shows combined status

**Use this for:** Regular deployments after initial setup

### Data Commands

#### `make data`
Loads problem data and removes duplicates.

**What it does:**
- Loads `factory/backend/exports/problems_import.sql` into database
- Removes duplicate problems (keeps lowest ID)
- Shows final problem count

**Requirements:**
- `factory/backend/exports/problems_import.sql` exists
- `.env.production` configured with `DATABASE_URL`
- `psql` command available

### Maintenance Commands

#### `make status`
Comprehensive health check of all services.

**What it shows:**
- ✓ Kubernetes backend pods status
- ✓ Cloudflare Tunnel status
- ✓ Frontend accessibility (HTTP check)
- ✓ Database connectivity and problem count
- ⚠ Configuration warnings (missing OAuth, etc.)

**Use this for:** Quick overview of deployment health

#### `make clean`
Cleans build artifacts (does not affect deployments).

**What it does:**
- Removes `crates/frontend/dist/`

**Use this for:** Forcing a clean rebuild

#### `make all`
Complete deployment pipeline.

**What it does:**
1. `make build` - Build Docker image
2. `make push` - Push to registry
3. `make deploy` - Deploy backend + frontend
4. `make data` - Load problems
5. `make status` - Final health check

**Use this for:** Complete releases from scratch

#### `make help`
Shows available commands with descriptions.

## Deployment Workflows

### First-Time Deployment

```bash
# 1. Generate secrets
make init

# 2. Edit .env.production with your credentials
nano .env.production
# Configure: OAuth (Google/GitHub), Resend API key, Frontend URLs

# 3. Get Cloudflare Tunnel token
make tunnel-instructions
# Follow the instructions to get the token
# Add CLOUDFLARED_TUNNEL to .env.production

# 4. Update database password
psql -h YOUR_DB_HOST -U postgres -d locus -c "ALTER USER locus_srv PASSWORD 'your_generated_password';"

# 5. Complete deployment
make all

# 6. Verify everything is running
make status
```

### Regular Update (Code Changes)

```bash
# Build and deploy new version
make build
make push
make deploy

# Check status
make status
```

### Frontend-Only Update

```bash
make deploy-frontend
```

### Backend-Only Update

```bash
make build
make push
make deploy-backend
```

### Problem Data Reload

```bash
# Make sure you have fresh problems_import.sql
make data
```

## Troubleshooting

### Build Fails

```bash
# Check Docker is running
docker ps

# Check insecure registry is configured
cat /etc/docker/daemon.json
# Should include: "insecure-registries": ["ghcr.io/fizzwizzledazzle"]
```

### Backend Deploy Fails

```bash
# Check Kubernetes connection
kubectl cluster-info

# Check namespace and pods
kubectl get pods -n locus

# View logs
kubectl logs -f deployment/locus-backend -n locus

# Check secrets in .env.production
cat .env.production | grep -E "JWT_SECRET|API_KEY_SECRET|GOOGLE_CLIENT_ID"
```

### Frontend Deploy Fails

```bash
# Check wrangler is logged in
wrangler whoami

# Check trunk is installed
trunk --version

# Try manual build
cd crates/frontend
trunk build --release
```

### Data Load Fails

```bash
# Check database connectivity
psql "$DATABASE_URL" -c "SELECT 1;"

# Check file exists
ls -lh factory/backend/exports/problems_import.sql

# Load manually
psql "$DATABASE_URL" < factory/backend/exports/problems_import.sql
```

### Cloudflare Tunnel Issues

```bash
# Check tunnel status in Kubernetes
kubectl get pods -n locus -l app=cloudflared

# View tunnel logs
kubectl logs -n locus -l app=cloudflared

# Recreate tunnel token
make tunnel
# Update CLOUDFLARED_TUNNEL in .env.production
make deploy-backend
```

## Manual Deployment (Advanced)

If you need more control, you can run the individual scripts:

```bash
# Generate secrets
./scripts/generate-secrets.sh

# Set up tunnel
./scripts/setup-cloudflare-tunnel.sh

# Deploy backend
./scripts/deploy-helm.sh

# Deploy frontend
./scripts/deploy-cloudflare-pages.sh

# Load data
./scripts/load-problems.sh
./scripts/remove-duplicates.sh

# Check status
./scripts/check-status.sh
```

## Environment Variables

Key variables in `.env.production`:

| Variable | Required | Description |
|----------|----------|-------------|
| `DATABASE_URL` | Yes | PostgreSQL connection string |
| `JWT_SECRET` | Yes | Secret for JWT signing (auto-generated) |
| `API_KEY_SECRET` | Yes | API key secret (auto-generated) |
| `GOOGLE_CLIENT_ID` | Yes | Google OAuth client ID |
| `GOOGLE_CLIENT_SECRET` | Yes | Google OAuth secret |
| `GITHUB_CLIENT_ID` | Yes | GitHub OAuth client ID |
| `GITHUB_CLIENT_SECRET` | Yes | GitHub OAuth secret |
| `RESEND_API_KEY` | Yes | Resend.com API key for emails |
| `RESEND_FROM_EMAIL` | Yes | Email sender address |
| `FRONTEND_BASE_URL` | Yes | Frontend URL (Pages domain) |
| `ALLOWED_ORIGINS` | Yes | CORS allowed origins |
| `OAUTH_REDIRECT_BASE` | Yes | OAuth redirect base URL |
| `CLOUDFLARED_TUNNEL` | Yes | Cloudflare Tunnel token |

## Deployment Checklist

- [ ] Run `make init` to generate secrets
- [ ] Configure `.env.production`:
  - [ ] Set up OAuth apps (Google, GitHub)
  - [ ] Get Resend API key
  - [ ] Set frontend URLs
  - [ ] Get Cloudflare Tunnel token (run `make tunnel-instructions`)
- [ ] Update database user password
- [ ] Run `make all` for complete deployment
- [ ] Configure custom domain in Cloudflare Pages dashboard
- [ ] Update DNS if needed
- [ ] Test OAuth login flow
- [ ] Test problem grading
- [ ] Run `make status` to verify health

## Support

For issues or questions:
- Check logs: `kubectl logs -f deployment/locus-backend -n locus`
- Run health check: `make status`
- View pod status: `kubectl get pods -n locus`
- Check this guide's Troubleshooting section
