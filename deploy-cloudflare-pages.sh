#!/bin/bash
# Deploy Locus frontend to Cloudflare Pages

set -e

echo "Deploying Locus Frontend to Cloudflare Pages"
echo ""

# Check if wrangler is installed
if ! command -v wrangler &> /dev/null; then
    echo "ERROR: Wrangler CLI not found!"
    echo "Install it with: npm install -g wrangler"
    echo "Or: pnpm add -g wrangler"
    exit 1
fi

# Check if logged in to Cloudflare
if ! wrangler whoami &> /dev/null; then
    echo "Please login to Cloudflare first:"
    wrangler login
fi

# Configuration
PROJECT_NAME="${CLOUDFLARE_PROJECT_NAME:-locus}"
BRANCH="${CLOUDFLARE_BRANCH:-main}"

echo "Building frontend..."
cd crates/frontend

# Clean previous build
rm -rf dist/

# Build with trunk (release mode)
trunk build --release

if [ ! -d "dist" ]; then
    echo "ERROR: Build failed! dist/ directory not found"
    exit 1
fi

echo "Build complete!"
echo ""

echo "Deploying to Cloudflare Pages..."
echo "Project: $PROJECT_NAME"
echo "Branch: $BRANCH"
echo ""

# Deploy to Cloudflare Pages
wrangler pages deploy dist \
    --project-name="$PROJECT_NAME" \
    --branch="$BRANCH"

echo ""
echo "Deployment complete!"
echo ""
echo "Your site should be available at:"
echo "   https://$PROJECT_NAME.pages.dev"
echo ""
echo "Next steps:"
echo "   - Set custom domain in Cloudflare Pages dashboard"
echo "   - Update ALLOWED_ORIGINS in backend to include your Pages URL"
echo "   - Update FRONTEND_BASE_URL in backend .env"
