.PHONY: all init build push deploy-backend deploy-frontend deploy data status clean help tunnel-instructions restart-backend delete-secrets

# Configuration
REGISTRY := ghcr.io/fizzwizzledazzle
IMAGE_NAME := locus
IMAGE_TAG := latest
NAMESPACE := locus
RELEASE_NAME := locus

# Use local kubeconfig if it exists
ifneq (,$(wildcard ./kubeconfig))
    export KUBECONFIG := $(PWD)/kubeconfig
endif

# Default target
.DEFAULT_GOAL := help

# Complete deployment pipeline
all: build push deploy data
	@echo ""
	@echo "============================================"
	@echo "✓ Complete deployment finished!"
	@echo "============================================"
	@$(MAKE) status

# First-time setup (secrets)
init:
	@echo "Initializing Locus deployment..."
	@echo ""
	@if [ ! -f .env.production ]; then \
		./scripts/generate-secrets.sh; \
	else \
		echo "✓ .env.production already exists"; \
		echo ""; \
		echo "Edit .env.production and configure:"; \
		echo "  - OAuth credentials (Google/GitHub)"; \
		echo "  - Resend API key"; \
		echo "  - Frontend URLs"; \
		echo "  - Cloudflare Tunnel token (see: make tunnel-instructions)"; \
		echo ""; \
		echo "Then run: make all"; \
	fi

# Show Cloudflare Tunnel setup (managed by Helm as sidecar)
tunnel-instructions:
	@echo "Get Cloudflare Tunnel token:"
	@echo "  cloudflared tunnel login"
	@echo "  cloudflared tunnel create locus-backend"
	@echo "  cloudflared tunnel route dns locus-backend api.locusmath.org"
	@echo "  cloudflared tunnel token locus-backend"
	@echo ""
	@echo "Add token to .env.production: CLOUDFLARED_TUNNEL=<token>"
	@echo "See RELEASE.md for detailed instructions."

# Build Docker image
build:
	@echo "============================================"
	@echo "Building Docker image..."
	@echo "============================================"
	docker build -t $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG) .
	@echo ""
	@echo "✓ Image built: $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG)"

# Push Docker image to registry
push:
	@echo "============================================"
	@echo "Pushing to registry..."
	@echo "============================================"
	docker push $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG)
	@echo ""
	@echo "✓ Image pushed to registry"

# Deploy backend to Kubernetes
deploy-backend:
	@echo "============================================"
	@echo "Deploying backend to Kubernetes..."
	@echo "============================================"
	@./scripts/deploy-helm.sh
	@echo ""
	@echo "✓ Backend deployed"

# Deploy frontend to Cloudflare Pages
deploy-frontend:
	@echo "============================================"
	@echo "Deploying frontend to Cloudflare Pages..."
	@echo "============================================"
	@./scripts/deploy-cloudflare-pages.sh
	@echo ""
	@echo "✓ Frontend deployed"

# Deploy both backend and frontend
deploy: deploy-backend deploy-frontend
	@echo ""
	@echo "✓ Both backend and frontend deployed"

# Load problems and clean duplicates
data:
	@if [ ! -f .env.production ]; then \
		echo "ERROR: .env.production not found. Run: make init"; \
		exit 1; \
	fi
	@. .env.production && ./scripts/load-data.sh

# Check deployment status
status:
	@./scripts/check-status.sh

# Delete all backend pods (forces restart with latest image)
restart-backend:
	@echo "Deleting all pods in $(RELEASE_NAME)-backend deployment..."
	@kubectl delete pods -n $(NAMESPACE) -l app=locus-backend || echo "No pods found or already deleted"
	@echo ""
	@echo "✓ Pods deleted. Kubernetes will recreate them automatically."
	@echo ""
	@echo "Watch status:"
	@echo "  kubectl get pods -n $(NAMESPACE) -w"

# Delete Kubernetes secrets (useful before redeploying with new credentials)
delete-secrets:
	@echo "Deleting secrets in $(NAMESPACE) namespace..."
	@kubectl delete secret $(RELEASE_NAME)-secrets -n $(NAMESPACE) 2>/dev/null || echo "No secrets found"
	@echo ""
	@echo "✓ Secrets deleted. Run 'make deploy-backend' to recreate them."

# Clean build artifacts (does not affect deployments)
clean:
	@echo "Cleaning build artifacts..."
	@cd crates/frontend && rm -rf dist/
	@echo "✓ Frontend dist/ cleaned"

# Show help
help:
	@echo ""
	@echo "Locus Deployment Commands"
	@echo "============================================"
	@echo ""
	@echo "Setup:"
	@echo "  make init                   First-time setup (generate secrets)"
	@echo "  make tunnel-instructions    Show Cloudflare Tunnel setup steps"
	@echo ""
	@echo "Build & Deploy:"
	@echo "  make build             Build Docker image"
	@echo "  make push              Push image to registry"
	@echo "  make deploy-backend    Deploy backend to Kubernetes"
	@echo "  make deploy-frontend   Deploy frontend to Cloudflare Pages"
	@echo "  make deploy            Deploy both backend and frontend"
	@echo ""
	@echo "Data:"
	@echo "  make data              Load problems and remove duplicates"
	@echo ""
	@echo "Complete Pipeline:"
	@echo "  make all               Build + Push + Deploy + Data"
	@echo ""
	@echo "Maintenance:"
	@echo "  make status            Check deployment health"
	@echo "  make restart-backend   Delete backend pods (force restart)"
	@echo "  make delete-secrets    Delete Kubernetes secrets"
	@echo "  make clean             Clean build artifacts"
	@echo "  make help              Show this help message"
	@echo ""
	@echo "Quick Start:"
	@echo "  1. make init                      (generate secrets)"
	@echo "  2. Edit .env.production           (add OAuth, Resend, URLs)"
	@echo "  3. make tunnel-instructions       (get Cloudflare token)"
	@echo "  4. make all                       (complete deployment)"
	@echo ""
