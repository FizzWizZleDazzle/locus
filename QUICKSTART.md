# Locus Deployment Quick Start

> **NEW:** We now have a consolidated Makefile-based deployment system!
> See [RELEASE.md](RELEASE.md) for the complete guide.

## Quick Deployment (Recommended)

```bash
# 1. Generate secrets and configure
make init
# Edit .env.production with your OAuth keys and API tokens

# 2. Set up Cloudflare Tunnel
make tunnel
# Add CLOUDFLARED_TUNNEL to .env.production

# 3. Deploy everything
make all
```

That's it! The Makefile handles building, deploying, and loading data.

---

## Manual Steps (Advanced)

### Step 1: Generate Secrets

```bash
./scripts/generate-secrets.sh
```

This creates `.env.production` with auto-generated JWT and database secrets.

## Step 2: Configure .env.production

Edit `.env.production` and update:

**Required OAuth Credentials:**
- Get Google OAuth: https://console.cloud.google.com/apis/credentials
- Get GitHub OAuth: https://github.com/settings/developers
- Update callback URLs to: `https://yourdomain.com/api/auth/{google,github}/callback`

**Required Email Service:**
- Sign up at https://resend.com
- Get API key from dashboard
- Update `RESEND_API_KEY` in `.env.production`

**Required Domain URLs:**
- Update all `yourdomain.com` to your actual domain

## Step 3: Deploy

### Option A: Complete Deployment (Recommended - Using Makefile)

```bash
# Build and deploy everything
make all

# Or deploy in stages:
make build          # Build Docker image
make push           # Push to registry
make deploy         # Deploy backend + frontend
make data           # Load problems
make status         # Check health
```

### Option B: Kubernetes with Helm (Manual)

```bash
# Deploy to K8s
./scripts/deploy-helm.sh

# Check status
kubectl get pods -n locus
kubectl logs -f deployment/locus-backend -n locus
```

### Option C: Frontend to Cloudflare Pages (Manual)

```bash
# Deploy frontend to Cloudflare Pages
./scripts/deploy-cloudflare-pages.sh

# Then deploy backend to K8s
./scripts/deploy-helm.sh
```

## Step 4: Verify

```bash
# Health check
curl https://yourdomain.com/health

# Test topics API
curl https://yourdomain.com/api/topics

# Open in browser
open https://yourdomain.com
```

## Required Setup

Before deploying, you MUST have:

1. **OAuth Apps Registered:**
   - Google OAuth app with production callback URL
   - GitHub OAuth app with production callback URL

2. **Email Service:**
   - Resend.com account with API key

3. **Secrets Generated:**
   - JWT_SECRET (auto-generated)
   - API_KEY_SECRET (auto-generated)
   - DB_PASSWORD (auto-generated)

4. **Domain Configured:**
   - DNS pointing to your K8s cluster or server
   - SSL certificate (cert-manager in K8s, or manual)

## Environment Variables Reference

All variables in `.env.production`:

**Auto-generated:**
- JWT_SECRET
- API_KEY_SECRET
- DB_PASSWORD

**Must configure:**
- ALLOWED_ORIGINS
- FRONTEND_BASE_URL
- OAUTH_REDIRECT_BASE
- GOOGLE_CLIENT_ID
- GOOGLE_CLIENT_SECRET
- GITHUB_CLIENT_ID
- GITHUB_CLIENT_SECRET
- RESEND_API_KEY
- RESEND_FROM_EMAIL

## Troubleshooting

**Backend won't start:**
- Check: `kubectl logs deployment/locus-backend -n locus`
- Verify all required env vars are set
- Check database connection

**OAuth not working:**
- Verify callback URLs match exactly
- Check client IDs/secrets are correct
- Ensure HTTPS is configured

**Email not working:**
- Verify RESEND_API_KEY is valid
- Check Resend dashboard for errors
- Verify domain is verified in Resend
