#!/bin/bash
# Deploy Locus to Kubernetes using Helm

set -e

# Check for kubectl
if ! command -v kubectl &> /dev/null; then
    echo "ERROR: kubectl not found"
    echo "Install with: curl -LO https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
    echo "Or: sudo apt install kubectl"
    exit 1
fi

# Check for helm
if ! command -v helm &> /dev/null; then
    echo "ERROR: helm not found"
    echo "Install with: curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash"
    exit 1
fi

# Configuration
NAMESPACE="locus"
RELEASE_NAME="${RELEASE_NAME:-locus}"
VALUES_FILE="${VALUES_FILE:-helm/locus/values.production.yaml}"

echo "Deploying Locus to Kubernetes with Helm"
echo "Namespace: $NAMESPACE"
echo "Release: $RELEASE_NAME"
echo ""

# Check if .env.production exists
if [ ! -f .env.production ]; then
    echo "ERROR: .env.production not found!"
    echo "Run ./generate-secrets.sh first and configure it."
    exit 1
fi

# Source .env.production
set -a
source .env.production
set +a

# Verify required variables
if [ -z "$JWT_SECRET" ] || [ -z "$API_KEY_SECRET" ] || [ -z "$DB_PASSWORD" ]; then
    echo "ERROR: Required secrets not found in .env.production"
    exit 1
fi

if [ -z "$RESEND_API_KEY" ] || [ "$RESEND_API_KEY" = "re_your_resend_api_key_here" ]; then
    echo "ERROR: RESEND_API_KEY must be set in .env.production"
    exit 1
fi

if [ -z "$GOOGLE_CLIENT_ID" ] || [ "$GOOGLE_CLIENT_ID" = "your_google_client_id_here" ]; then
    echo "ERROR: GOOGLE_CLIENT_ID must be set in .env.production"
    exit 1
fi

if [ -z "$GITHUB_CLIENT_ID" ] || [ "$GITHUB_CLIENT_ID" = "your_github_client_id_here" ]; then
    echo "ERROR: GITHUB_CLIENT_ID must be set in .env.production"
    exit 1
fi

# Create namespace if it doesn't exist
kubectl create namespace $NAMESPACE --dry-run=client -o yaml | kubectl apply -f -

echo "Installing/upgrading Locus with Helm..."
echo ""

# Check for Cloudflare Tunnel token
if [ -z "$CLOUDFLARED_TUNNEL" ]; then
    echo "WARNING: CLOUDFLARED_TUNNEL not set in .env.production"
    echo "Get it from: cloudflared tunnel token locus-backend"
    read -p "Continue without tunnel? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Deploy with Helm
helm upgrade --install $RELEASE_NAME ./helm/locus \
    --namespace $NAMESPACE \
    --set secrets.databaseUrl="$DATABASE_URL" \
    --set secrets.jwtSecret="$JWT_SECRET" \
    --set secrets.apiKeySecret="$API_KEY_SECRET" \
    --set secrets.googleClientId="$GOOGLE_CLIENT_ID" \
    --set secrets.googleClientSecret="$GOOGLE_CLIENT_SECRET" \
    --set secrets.githubClientId="$GITHUB_CLIENT_ID" \
    --set secrets.githubClientSecret="$GITHUB_CLIENT_SECRET" \
    --set secrets.resendApiKey="$RESEND_API_KEY" \
    --set secrets.resendFromEmail="$RESEND_FROM_EMAIL" \
    --set secrets.resendFromName="$RESEND_FROM_NAME" \
    --set backend.allowedOrigins="$ALLOWED_ORIGINS" \
    --set backend.frontendBaseUrl="$FRONTEND_BASE_URL" \
    --set backend.oauthRedirectBase="$OAUTH_REDIRECT_BASE" \
    --set cloudflaredTunnel.token="$CLOUDFLARED_TUNNEL" \
    ${VALUES_FILE:+--values $VALUES_FILE} \
    --wait

echo ""
echo "Deployment complete!"
echo ""
echo "Check status:"
echo "  kubectl get pods -n $NAMESPACE"
echo "  kubectl logs -f deployment/$RELEASE_NAME-backend -n $NAMESPACE"
echo ""
echo "Port forward to test locally:"
echo "  kubectl port-forward -n $NAMESPACE service/$RELEASE_NAME-backend 28743:80"
