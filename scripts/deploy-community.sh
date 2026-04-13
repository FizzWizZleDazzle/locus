#!/bin/bash
# Deploy Locus community services (forum + status backend) to Kubernetes using Helm

set -e

# Check for kubectl and helm
for cmd in kubectl helm; do
    if ! command -v $cmd &> /dev/null; then
        echo "ERROR: $cmd not found"
        exit 1
    fi
done

NAMESPACE="locus"
RELEASE_NAME="locus"

echo "Deploying Locus community services to Kubernetes"
echo "Namespace: $NAMESPACE"
echo ""

# Check if .env exists
if [ ! -f .env ]; then
    echo "ERROR: .env not found!"
    echo "Run ./scripts/generate-secrets.sh first and configure it."
    exit 1
fi

# Source .env and remap PRODUCTION_ vars
set -a
source .env
set +a

DATABASE_URL="${PRODUCTION_DATABASE_URL}"
JWT_SECRET="${PRODUCTION_JWT_SECRET}"
COMMUNITY_ALLOWED_ORIGINS="${PRODUCTION_COMMUNITY_ALLOWED_ORIGINS}"
COMMUNITY_CLOUDFLARED_TOKEN="${PRODUCTION_COMMUNITY_CLOUDFLARED_TOKEN}"
COOKIE_DOMAIN="${PRODUCTION_COOKIE_DOMAIN}"

# Verify required variables
if [ -z "$JWT_SECRET" ]; then
    echo "ERROR: PRODUCTION_JWT_SECRET not set in .env"
    exit 1
fi

if [ -z "$DATABASE_URL" ]; then
    echo "ERROR: PRODUCTION_DATABASE_URL not set in .env"
    exit 1
fi

if [ -z "$COMMUNITY_ALLOWED_ORIGINS" ]; then
    echo "ERROR: PRODUCTION_COMMUNITY_ALLOWED_ORIGINS not set in .env"
    echo "Example: https://forum.locusmath.org,https://status.locusmath.org"
    exit 1
fi

if [ -z "$COMMUNITY_CLOUDFLARED_TOKEN" ]; then
    echo "WARNING: PRODUCTION_COMMUNITY_CLOUDFLARED_TOKEN not set in .env"
    read -p "Continue without tunnel? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
    COMMUNITY_CLOUDFLARED_TOKEN="placeholder"
fi

# Create namespace if it doesn't exist
kubectl create namespace $NAMESPACE --dry-run=client -o yaml | kubectl apply -f -

echo "Installing/upgrading community services with Helm..."
echo ""

# Escape commas for Helm --set (commas are key-value separators in Helm)
ESCAPED_ORIGINS=$(echo "$COMMUNITY_ALLOWED_ORIGINS" | sed 's/,/\\,/g')

helm upgrade --install ${RELEASE_NAME}-community ./helm/community \
    --namespace $NAMESPACE \
    --values helm/community/values.production.yaml \
    --set secrets.databaseUrl="$DATABASE_URL" \
    --set secrets.jwtSecret="$JWT_SECRET" \
    --set secrets.cloudflaredToken="$COMMUNITY_CLOUDFLARED_TOKEN" \
    --set backend.allowedOrigins="$ESCAPED_ORIGINS" \
    --set backend.cookieDomain="$COOKIE_DOMAIN" \
    --wait

echo ""
echo "Community services deployment complete!"
echo ""
echo "Check status:"
echo "  kubectl get pods -n $NAMESPACE -l app=${RELEASE_NAME}-community"
echo "  kubectl logs -f deployment/${RELEASE_NAME}-community -n $NAMESPACE -c backend"
