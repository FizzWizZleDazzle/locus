#!/bin/bash
# Generate production secrets and create .env.production file
# Secrets are NOT printed to terminal

set -e

echo "Generating production secrets..."

# Generate secrets
JWT_SECRET=$(openssl rand -base64 32)
API_KEY_SECRET=$(openssl rand -base64 32)
DB_PASSWORD=$(openssl rand -base64 32)

# Create .env.production file
cat > .env.production << EOF
# Production Environment Variables
# Generated: $(date)
# KEEP THIS FILE SECURE! Add to .gitignore!

# Environment
ENVIRONMENT=production

# Database
DATABASE_URL=postgresql://locus:${DB_PASSWORD}@localhost:5432/locus
DB_PASSWORD=${DB_PASSWORD}

# Security Secrets (auto-generated)
JWT_SECRET=${JWT_SECRET}
API_KEY_SECRET=${API_KEY_SECRET}

# CORS & Frontend URLs (UPDATE THESE!)
ALLOWED_ORIGINS=https://yourdomain.com
FRONTEND_BASE_URL=https://yourdomain.com
OAUTH_REDIRECT_BASE=https://yourdomain.com

# OAuth (REQUIRED - get these from Google/GitHub OAuth apps)
GOOGLE_CLIENT_ID=your_google_client_id_here
GOOGLE_CLIENT_SECRET=your_google_client_secret_here
GITHUB_CLIENT_ID=your_github_client_id_here
GITHUB_CLIENT_SECRET=your_github_client_secret_here

# Email Service (REQUIRED - get from resend.com)
RESEND_API_KEY=re_your_resend_api_key_here
RESEND_FROM_EMAIL=no-reply@yourdomain.com
RESEND_FROM_NAME=Locus

# Server Config
HOST=0.0.0.0
PORT=28743
JWT_EXPIRY_HOURS=24
EOF

chmod 600 .env.production

echo "SUCCESS: Secrets generated and saved to .env.production"
echo ""
echo "NEXT STEPS:"
echo "1. Edit .env.production and update:"
echo "   - All domain URLs (yourdomain.com)"
echo "   - GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET"
echo "   - GITHUB_CLIENT_ID and GITHUB_CLIENT_SECRET"
echo "   - RESEND_API_KEY (from resend.com)"
echo "   - RESEND_FROM_EMAIL"
echo ""
echo "2. Get OAuth credentials:"
echo "   - Google: https://console.cloud.google.com/apis/credentials"
echo "   - GitHub: https://github.com/settings/developers"
echo ""
echo "3. Get Resend API key:"
echo "   - Sign up at https://resend.com"
echo "   - Get API key from dashboard"
