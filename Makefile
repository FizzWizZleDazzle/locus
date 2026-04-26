.PHONY: all init build push deploy-backend deploy-frontend deploy data status clean help tunnel-instructions restart-backend delete-secrets build-services-backend push-services-backend deploy-services-backend deploy-status-frontend

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
	@if ! grep -q '^PRODUCTION_JWT_SECRET' .env 2>/dev/null; then \
		./scripts/generate-secrets.sh; \
	else \
		echo "✓ Production secrets already exist in .env"; \
		echo ""; \
		echo "Edit .env and configure the PRODUCTION_ variables:"; \
		echo "  - OAuth credentials (Google/GitHub)"; \
		echo "  - Resend API key"; \
		echo "  - PRODUCTION_FRONTEND_BASE_URL"; \
		echo "  - PRODUCTION_CLOUDFLARED_TUNNEL (see: make tunnel-instructions)"; \
		echo ""; \
		echo "Then run: make all"; \
	fi


# Fetch vendored binary assets (cetz_core.wasm). Idempotent.
.PHONY: assets
assets:
	@scripts/fetch-diagram-assets.sh

# Build Docker image
build: assets
	@echo "============================================"
	@echo "Building Docker image..."
	@echo "============================================"
	docker build -f docker/Dockerfile -t $(REGISTRY)/$(IMAGE_NAME):$(IMAGE_TAG) .
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
	@if [ ! -f .env ]; then \
		echo "ERROR: .env not found. Run: make init"; \
		exit 1; \
	fi
	@. .env && DATABASE_URL="$${PRODUCTION_DATABASE_URL}" ./scripts/load-data.sh

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

# === Services (status page) ===

build-services-backend:
	@echo "Building services backend Docker image..."
	docker build -f crates/services-backend/Dockerfile -t $(REGISTRY)/locus-services:$(IMAGE_TAG) .
	@echo "✓ Services image built"

push-services-backend:
	docker push $(REGISTRY)/locus-services:$(IMAGE_TAG)
	@echo "✓ Services image pushed"

deploy-services-backend: build-services-backend push-services-backend
	@echo "✓ Services backend deployed"

deploy-status-frontend:
	cd crates/status && COMMUNITY_API_URL=https://community-api.locusmath.org/api trunk build --release && wrangler pages deploy dist --project-name locus-status
	@echo "✓ Status frontend deployed"

# Show help
help:
	@echo ""
	@echo "Locus Deployment Commands"
	@echo "============================================"
	@echo ""
	@echo "Setup:"
	@echo "  make init                   First-time setup (generate secrets)"
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
	@echo "  2. Edit .env PRODUCTION_ vars     (add OAuth, Resend, URLs)"
	@echo "  3. make tunnel-instructions       (get Cloudflare token)"
	@echo "  4. make all                       (complete deployment)"
	@echo ""
