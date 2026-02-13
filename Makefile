.PHONY: all init build push deploy-backend deploy-frontend deploy data status clean help

# Configuration
REGISTRY := ghcr.io/fizzwizzledazzle
IMAGE_NAME := locus
IMAGE_TAG := latest
NAMESPACE := locus
RELEASE_NAME := locus

# Default target
.DEFAULT_GOAL := help

# Complete deployment pipeline
all: build push deploy data
	@echo ""
	@echo "============================================"
	@echo "✓ Complete deployment finished!"
	@echo "============================================"
	@$(MAKE) status

# First-time setup (secrets + tunnel)
init:
	@echo "Initializing Locus deployment..."
	@echo ""
	@if [ ! -f .env.production ]; then \
		./scripts/generate-secrets.sh; \
		echo ""; \
		echo "Please edit .env.production and configure:"; \
		echo "  - OAuth credentials (Google/GitHub)"; \
		echo "  - Resend API key"; \
		echo "  - Frontend URLs"; \
		echo ""; \
		echo "Then run: make tunnel"; \
	else \
		echo "✓ .env.production already exists"; \
		echo ""; \
		echo "Run: make tunnel"; \
	fi

# Set up Cloudflare Tunnel
tunnel:
	@echo "Setting up Cloudflare Tunnel..."
	@./scripts/setup-cloudflare-tunnel.sh
	@echo ""
	@echo "Don't forget to add CLOUDFLARED_TUNNEL to .env.production!"

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
	@echo "============================================"
	@echo "Loading problem data..."
	@echo "============================================"
	@if [ ! -f factory/backend/exports/problems_import.sql ]; then \
		echo "ERROR: factory/backend/exports/problems_import.sql not found"; \
		echo "Run the factory to generate problems first."; \
		exit 1; \
	fi
	@if [ ! -f .env.production ]; then \
		echo "ERROR: .env.production not found"; \
		echo "Run: make init"; \
		exit 1; \
	fi
	@. .env.production && ./scripts/load-problems.sh
	@. .env.production && ./scripts/remove-duplicates.sh
	@echo ""
	@echo "✓ Problem data loaded and cleaned"

# Check deployment status
status:
	@./scripts/check-status.sh

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
	@echo "  make init              First-time setup (generate secrets)"
	@echo "  make tunnel            Set up Cloudflare Tunnel"
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
	@echo "  make clean             Clean build artifacts"
	@echo "  make help              Show this help message"
	@echo ""
	@echo "Quick Start:"
	@echo "  1. make init           (configure .env.production)"
	@echo "  2. make tunnel         (set up Cloudflare)"
	@echo "  3. make all            (complete deployment)"
	@echo ""
