#!/bin/bash
# Generate secure secrets for .env.production

set -e

echo "Generating secrets for Locus production deployment..."
echo ""

# Generate strong random secrets
JWT_SECRET=$(openssl rand -base64 32)
API_KEY_SECRET=$(openssl rand -base64 32)
DB_PASSWORD=$(openssl rand -base64 32)

# Create .env.production with generated secrets
cat > .env.production << EOF
# ========================================
# Locus Production Configuration
# Generated: $(date)
# ========================================

# Environment
ENVIRONMENT=production

# Database
DATABASE_URL=postgresql://locus_srv:${DB_PASSWORD}@YOUR_DB_HOST:5432/locus

# Security Secrets (auto-generated)
JWT_SECRET=${JWT_SECRET}
API_KEY_SECRET=${API_KEY_SECRET}

# Email (Resend) - CONFIGURE THESE!
RESEND_API_KEY=re_your_resend_api_key_here
RESEND_FROM_EMAIL=noreply@yourdomain.com
RESEND_FROM_NAME=Locus

# OAuth Providers - CONFIGURE THESE!
GOOGLE_CLIENT_ID=your_google_client_id_here
GOOGLE_CLIENT_SECRET=your_google_client_secret_here
GITHUB_CLIENT_ID=your_github_client_id_here
GITHUB_CLIENT_SECRET=your_github_client_secret_here

# Frontend URLs - CONFIGURE THESE!
FRONTEND_BASE_URL=https://locus.pages.dev
ALLOWED_ORIGINS=https://locus.pages.dev
OAUTH_REDIRECT_BASE=https://locus.pages.dev

# Cloudflare Tunnel - Run setup-cloudflare-tunnel.sh to get this
CLOUDFLARED_TUNNEL=

# Cloudflare Pages
CLOUDFLARE_PROJECT_NAME=locus
CLOUDFLARE_BRANCH=main

# Database password (for PostgreSQL user creation)
DB_PASSWORD=${DB_PASSWORD}
EOF

chmod 600 .env.production

echo "✓ Secrets generated and saved to .env.production"
echo ""
echo "NEXT STEPS:"
echo "============"
echo ""
echo "1. Edit .env.production and configure:"
echo "   - RESEND_API_KEY (from resend.com)"
echo "   - GOOGLE_CLIENT_ID/SECRET (from Google Cloud Console)"
echo "   - GITHUB_CLIENT_ID/SECRET (from GitHub OAuth Apps)"
echo "   - FRONTEND_BASE_URL (your actual domain)"
echo ""
echo "2. Get Cloudflare Tunnel token:"
echo "   Run: make tunnel-instructions"
echo "   Then add CLOUDFLARED_TUNNEL to .env.production"
echo ""
echo "3. Update database password for locus_srv user:"
echo "   psql -h YOUR_DB_HOST -U postgres -d locus -c \"ALTER USER locus_srv PASSWORD '${DB_PASSWORD}';\""
echo ""
echo "4. Deploy everything:"
echo "   make all"
echo ""
echo "Generated secrets (DO NOT COMMIT):"
echo "  - JWT_SECRET: ${JWT_SECRET:0:16}..."
echo "  - API_KEY_SECRET: ${API_KEY_SECRET:0:16}..."
echo "  - DB_PASSWORD: ${DB_PASSWORD:0:16}..."
echo ""
