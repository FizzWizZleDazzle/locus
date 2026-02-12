# Locus Deployment Quick Start

## Step 1: Generate Secrets

```bash
./generate-secrets.sh
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

### Option A: Kubernetes with Helm

```bash
# Deploy to K8s
./deploy-helm.sh

# Check status
kubectl get pods -n locus
kubectl logs -f deployment/locus-backend -n locus
```

### Option B: Docker Compose

```bash
# Deploy locally
docker compose --env-file .env.production -f docker-compose.prod.yml up -d

# Check logs
docker compose -f docker-compose.prod.yml logs -f

# Verify health
curl http://localhost:3000/health
```

### Option C: Cloudflare Pages (Frontend Only)

```bash
# Deploy frontend to Cloudflare Pages
./deploy-cloudflare-pages.sh

# Then deploy backend to K8s/Docker
# Update backend ALLOWED_ORIGINS to include Pages URL
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
