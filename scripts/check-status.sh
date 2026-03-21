#!/bin/bash
# Check deployment health status

set -e

NAMESPACE="${NAMESPACE:-locus}"
RELEASE_NAME="${RELEASE_NAME:-locus}"

echo "============================================"
echo "Locus Deployment Status"
echo "============================================"
echo ""

# Check if .env exists
if [ -f .env ]; then
    echo "✓ Configuration file exists"
    source .env
else
    echo "✗ .env not found"
    exit 1
fi

# Remap PRODUCTION_ vars for status checks
DATABASE_URL="${PRODUCTION_DATABASE_URL}"
FRONTEND_BASE_URL="${PRODUCTION_FRONTEND_BASE_URL}"
CLOUDFLARED_TUNNEL="${PRODUCTION_CLOUDFLARED_TUNNEL}"

echo ""
echo "1. Kubernetes Backend"
echo "--------------------"

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "✗ kubectl not found"
else
    # Check namespace
    if kubectl get namespace $NAMESPACE &> /dev/null; then
        echo "✓ Namespace '$NAMESPACE' exists"

        # Check pods
        echo ""
        echo "Pods:"
        kubectl get pods -n $NAMESPACE

        # Check if backend is running
        BACKEND_PODS=$(kubectl get pods -n $NAMESPACE -l app=locus-backend -o jsonpath='{.items[*].status.phase}' 2>/dev/null || echo "")
        if [[ "$BACKEND_PODS" == *"Running"* ]]; then
            echo "✓ Backend pods are running"
        else
            echo "✗ Backend pods not running"
        fi

        # Check service
        echo ""
        echo "Services:"
        kubectl get svc -n $NAMESPACE

    else
        echo "✗ Namespace '$NAMESPACE' not found"
    fi
fi

echo ""
echo "2. Cloudflare Tunnel"
echo "--------------------"

if [ -z "$CLOUDFLARED_TUNNEL" ]; then
    echo "✗ PRODUCTION_CLOUDFLARED_TUNNEL not configured"
else
    echo "✓ PRODUCTION_CLOUDFLARED_TUNNEL configured"
    if kubectl get pods -n $NAMESPACE -l app=cloudflared &> /dev/null 2>&1; then
        TUNNEL_STATUS=$(kubectl get pods -n $NAMESPACE -l app=cloudflared -o jsonpath='{.items[0].status.phase}' 2>/dev/null || echo "NotFound")
        if [ "$TUNNEL_STATUS" = "Running" ]; then
            echo "✓ Cloudflare Tunnel pod is running"
        else
            echo "⚠ Cloudflare Tunnel pod status: $TUNNEL_STATUS"
        fi
    fi
fi

echo ""
echo "3. Frontend (Cloudflare Pages)"
echo "-------------------------------"

if [ -n "$FRONTEND_BASE_URL" ]; then
    echo "URL: $FRONTEND_BASE_URL"
    # Try to reach frontend
    if command -v curl &> /dev/null; then
        HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "$FRONTEND_BASE_URL" || echo "000")
        if [ "$HTTP_CODE" = "200" ]; then
            echo "✓ Frontend is accessible (HTTP $HTTP_CODE)"
        else
            echo "⚠ Frontend returned HTTP $HTTP_CODE"
        fi
    else
        echo "  (curl not available for health check)"
    fi
else
    echo "✗ PRODUCTION_FRONTEND_BASE_URL not configured"
fi

echo ""
echo "4. Database"
echo "-----------"

if [ -n "$DATABASE_URL" ]; then
    echo "URL: ${DATABASE_URL%%@*}@***"

    # Try to connect and count problems
    if command -v psql &> /dev/null; then
        PROBLEM_COUNT=$(psql "$DATABASE_URL" -t -c "SELECT COUNT(*) FROM problems;" 2>/dev/null || echo "0")
        if [ "$PROBLEM_COUNT" -gt 0 ]; then
            echo "✓ Database connected - $PROBLEM_COUNT problems"
        else
            echo "⚠ Database connected but no problems found"
        fi
    else
        echo "  (psql not available for health check)"
    fi
else
    echo "✗ PRODUCTION_DATABASE_URL not configured"
fi

echo ""
echo "5. Configuration Status"
echo "-----------------------"

# Check OAuth configuration
OAUTH_CONFIGURED=true
if [ "$GOOGLE_CLIENT_ID" = "your_google_client_id_here" ]; then
    echo "⚠ Google OAuth not configured"
    OAUTH_CONFIGURED=false
fi
if [ "$GITHUB_CLIENT_ID" = "your_github_client_id_here" ]; then
    echo "⚠ GitHub OAuth not configured"
    OAUTH_CONFIGURED=false
fi
if [ "$OAUTH_CONFIGURED" = true ]; then
    echo "✓ OAuth providers configured"
fi

# Check email configuration
if [ "$RESEND_API_KEY" = "re_your_resend_api_key_here" ]; then
    echo "⚠ Email (Resend) not configured"
else
    echo "✓ Email configured"
fi

echo ""
echo "============================================"
echo ""
echo "Quick commands:"
echo "  kubectl logs -f deployment/$RELEASE_NAME-backend -n $NAMESPACE"
echo "  kubectl port-forward -n $NAMESPACE service/$RELEASE_NAME-backend 28743:80"
echo ""
