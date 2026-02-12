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

# Security Secrets
JWT_SECRET=${JWT_SECRET}
API_KEY_SECRET=${API_KEY_SECRET}

# CORS & Frontend URLs (UPDATE THESE!)
ALLOWED_ORIGINS=https://yourdomain.com
FRONTEND_BASE_URL=https://yourdomain.com
OAUTH_REDIRECT_BASE=https://api.yourdomain.com

# OAuth (optional - update if using)
GOOGLE_CLIENT_ID=
GOOGLE_CLIENT_SECRET=
GITHUB_CLIENT_ID=
GITHUB_CLIENT_SECRET=

# Email Service (optional)
RESEND_API_KEY=
RESEND_FROM_EMAIL=no-reply@yourdomain.com
RESEND_FROM_NAME=Locus

# Server Config
HOST=0.0.0.0
PORT=3000
JWT_EXPIRY_HOURS=24
EOF

chmod 600 .env.production

echo "SUCCESS: Secrets generated and saved to .env.production"
echo "WARNING: Edit .env.production and update the domain URLs!"
echo "WARNING: Add .env.production to .gitignore if not already there!"
