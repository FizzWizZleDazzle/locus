#!/bin/bash
# Generate secure secrets and append PRODUCTION_ vars to .env

set -e

echo "Generating secrets for Locus production deployment..."
echo ""

# Generate strong random secrets
JWT_SECRET=$(openssl rand -base64 32)
DB_PASSWORD=$(openssl rand -base64 32)

# Check if .env exists; if not, copy from .env.example
if [ ! -f .env ]; then
    if [ -f .env.example ]; then
        cp .env.example .env
        echo "✓ Created .env from .env.example"
    else
        echo "ERROR: Neither .env nor .env.example found"
        exit 1
    fi
fi

# Remove existing PRODUCTION_ lines to avoid duplicates
sed -i '/^PRODUCTION_/d' .env
sed -i '/^# ── Production/d' .env

# Append production vars
cat >> .env << EOF

# ── Production (used by deploy scripts only) ──
PRODUCTION_DATABASE_URL=postgresql://locus_srv:${DB_PASSWORD}@YOUR_DB_HOST:5432/locus
PRODUCTION_DB_PASSWORD=${DB_PASSWORD}
PRODUCTION_JWT_SECRET=${JWT_SECRET}
PRODUCTION_ALLOWED_ORIGINS=https://locus.pages.dev
PRODUCTION_FRONTEND_BASE_URL=https://locus.pages.dev
PRODUCTION_OAUTH_REDIRECT_BASE=https://locus.pages.dev
PRODUCTION_RESEND_FROM_EMAIL=noreply@yourdomain.com
PRODUCTION_CLOUDFLARED_TUNNEL=
PRODUCTION_SERVICES_ALLOWED_ORIGINS=https://status.locusmath.org
PRODUCTION_SERVICES_CLOUDFLARED_TOKEN=
EOF

chmod 600 .env

echo "✓ Production secrets generated and saved to .env"
echo ""
echo "NEXT STEPS:"
echo "============"
echo ""
echo "1. Edit .env and configure the PRODUCTION_ variables:"
echo "   - RESEND_API_KEY (from resend.com)"
echo "   - GOOGLE_CLIENT_ID/SECRET (from Google Cloud Console)"
echo "   - GITHUB_CLIENT_ID/SECRET (from GitHub OAuth Apps)"
echo "   - PRODUCTION_FRONTEND_BASE_URL (your actual domain)"
echo ""
echo "2. Get Cloudflare Tunnel token:"
echo "   Run: make tunnel-instructions"
echo "   Then set PRODUCTION_CLOUDFLARED_TUNNEL in .env"
echo ""
echo "3. Update database password for locus_srv user:"
echo "   psql -h YOUR_DB_HOST -U postgres -d locus -c \"ALTER USER locus_srv PASSWORD '${DB_PASSWORD}';\""
echo ""
echo "4. Deploy everything:"
echo "   make all"
echo ""
echo "Generated secrets (DO NOT COMMIT):"
echo "  - JWT_SECRET: ${JWT_SECRET:0:16}..."
echo "  - DB_PASSWORD: ${DB_PASSWORD:0:16}..."
echo ""
