#!/usr/bin/env bash
#
# Locus Development Script
# Starts PostgreSQL, backend, and frontend for local development
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $*"; }
log_success() { echo -e "${GREEN}[OK]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }

cleanup() {
    log_info "Shutting down..."
    kill $BACKEND_PID 2>/dev/null || true
    kill $FRONTEND_PID 2>/dev/null || true
    exit 0
}

trap cleanup SIGINT SIGTERM

# Check dependencies
check_deps() {
    local missing=()
    command -v cargo >/dev/null || missing+=("cargo")
    command -v docker >/dev/null || missing+=("docker")
    command -v trunk >/dev/null || missing+=("trunk (cargo install trunk)")

    if [ ${#missing[@]} -ne 0 ]; then
        log_error "Missing dependencies: ${missing[*]}"
        exit 1
    fi
}

# Docker compose command (v1 or v2)
docker_compose() {
    if docker compose version >/dev/null 2>&1; then
        docker compose "$@"
    else
        docker-compose "$@"
    fi
}

# Create .env if it doesn't exist
ensure_env() {
    if [ ! -f .env ]; then
        log_info "Creating .env from .env.example"
        cp .env.example .env
    fi
}

# Start PostgreSQL
start_db() {
    log_info "Starting PostgreSQL..."

    # Check if container exists (running or stopped)
    if docker ps -a --format '{{.Names}}' | grep -q "^locus-db$"; then
        # Container exists, check if it's running
        if docker ps --format '{{.Names}}' | grep -q "^locus-db$"; then
            log_success "PostgreSQL already running"
        else
            log_info "Starting existing PostgreSQL container..."
            docker start locus-db
            log_info "Waiting for PostgreSQL to be ready..."
            sleep 3
            log_success "PostgreSQL started"
        fi
    else
        # Container doesn't exist, create it
        docker_compose up -d
        log_info "Waiting for PostgreSQL to be ready..."
        sleep 3
        log_success "PostgreSQL started"
    fi
}

# Build and run backend
start_backend() {
    log_info "Starting backend on http://localhost:3000"
    cargo run -p locus-backend &
    BACKEND_PID=$!
    sleep 2
}

# Build and run frontend
start_frontend() {
    log_info "Starting frontend on http://localhost:8080"
    cd crates/frontend
    # Set API URL to backend
    export LOCUS_API_URL=http://localhost:3000/api
    # Pass wasm-bindgen flags to disable reference types (fixes "env" module errors)
    trunk serve &
    FRONTEND_PID=$!
    cd ../..
}

main() {
    echo ""
    echo "  ╦  ╔═╗╔═╗╦ ╦╔═╗"
    echo "  ║  ║ ║║  ║ ║╚═╗"
    echo "  ╩═╝╚═╝╚═╝╚═╝╚═╝"
    echo "  Development Server"
    echo ""

    check_deps
    ensure_env
    start_db
    start_backend
    start_frontend

    echo ""
    log_success "Development servers running:"
    echo "  Frontend: http://localhost:8080"
    echo "  Backend:  http://localhost:3000"
    echo "  Database: localhost:5432"
    echo ""
    log_info "Press Ctrl+C to stop all servers"
    echo ""

    # Wait for either process to exit
    wait $BACKEND_PID $FRONTEND_PID
}

main "$@"
