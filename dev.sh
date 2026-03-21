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
    kill $SERVICES_PID 2>/dev/null || true
    kill $FORUM_PID 2>/dev/null || true
    kill $STATUS_PID 2>/dev/null || true
    exit 0
}

trap cleanup SIGINT SIGTERM

# Check dependencies
check_deps() {
    local missing=()
    command -v cargo >/dev/null || missing+=("cargo")
    command -v docker >/dev/null || missing+=("docker")
    command -v trunk >/dev/null || missing+=("trunk (cargo install trunk)")
    command -v cargo-watch >/dev/null || missing+=("cargo-watch (cargo install cargo-watch)")

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

# Print development environment configuration
print_config() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Development Environment Configuration${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo "  Frontend URL:    http://localhost:8080"
    echo "  Backend URL:     http://localhost:3000"
    echo "  Database:        localhost:5432"
    echo "  API Base:        http://localhost:3000/api (LOCUS_API_URL)"
    echo "  Frontend Base:   http://localhost:8080 (LOCUS_FRONTEND_URL)"
    echo ""

    # Check for default/insecure secrets
    if [ -f .env ]; then
        if grep -q "JWT_SECRET=your-secret-key-here" .env 2>/dev/null; then
            log_warn "Using default JWT_SECRET from .env.example"
            log_warn "Generate a secure secret for production!"
        fi
    fi

    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

# Validate development environment
validate_env() {
    local issues=0

    # Check if .env exists
    if [ ! -f .env ]; then
        log_error ".env file not found!"
        issues=$((issues + 1))
    fi

    # Check if database is accessible
    if ! docker ps --format '{{.Names}}' | grep -q "^locus-db$"; then
        log_warn "Database container not running (will be started)"
    fi

    return $issues
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

# Build and run backend (with hot reload)
start_backend() {
    log_info "Starting backend on http://localhost:3000 (hot reload)"
    # Set CORS allowed origins from frontend URL
    local frontend_url="${LOCUS_FRONTEND_URL:-http://localhost:8080}"
    export ALLOWED_ORIGINS="$frontend_url"
    cargo watch -w crates/backend/src -w crates/common/src -x 'run --bin locus-backend' &
    BACKEND_PID=$!
    sleep 2
}

# Build and run frontend
start_frontend() {
    log_info "Starting frontend on http://localhost:8080"
    # Set environment variables for development (use existing or defaults)
    export LOCUS_API_URL="${LOCUS_API_URL:-http://localhost:3000/api}"
    export LOCUS_FRONTEND_URL="${LOCUS_FRONTEND_URL:-http://localhost:8080}"
    export LOCUS_ENV=development
    cd crates/frontend
    
    # Update Trunk.toml with dynamic public_url if TRUNK_PUBLIC_URL is set
    if [ -n "$TRUNK_PUBLIC_URL" ]; then
        # Extract the URL and ensure trailing slash
        public_url="${TRUNK_PUBLIC_URL%/}/"
        # Create a temporary Trunk config with the public_url
        {
            echo '[build]'
            echo 'target = "index.html"'
            echo 'dist = "dist"'
            echo ''
            echo '[watch]'
            echo 'watch = ["src", "index.html"]'
            echo ''
            echo '[serve]'
            echo "address = \"${TRUNK_SERVE_ADDRESS:-127.0.0.1}\""
            echo 'port = 8080'
            echo 'open = false'
            echo "public_url = \"$public_url\""
            echo ''
            echo '# Proxy API requests to backend'
            echo '[[proxy]]'
            echo 'rewrite = "/api/"'
            echo "backend = \"${LOCUS_API_URL%/api}/api/\""
        } > Trunk.toml
    fi
    
    # Pass wasm-bindgen flags to disable reference types (fixes "env" module errors)
    trunk serve &
    FRONTEND_PID=$!
    cd ../..
}

# Start community services backend
start_services_backend() {
    log_info "Starting services backend on http://localhost:8090"
    PORT=8090 ALLOWED_ORIGINS="http://localhost:8081,http://localhost:8082" \
        cargo watch -w crates/services-backend/src -x 'run -p locus-services-backend' &
    SERVICES_PID=$!
    sleep 2
}

# Start forum frontend
start_forum_frontend() {
    log_info "Starting forum frontend on http://localhost:8081"
    export COMMUNITY_API_URL="http://localhost:8090/api"
    export LOCUS_FRONTEND_URL="http://localhost:8080"
    cd crates/forum
    trunk serve &
    FORUM_PID=$!
    cd ../..
}

# Start status frontend
start_status_frontend() {
    log_info "Starting status frontend on http://localhost:8082"
    export COMMUNITY_API_URL="http://localhost:8090/api"
    cd crates/status
    trunk serve &
    STATUS_PID=$!
    cd ../..
}

main() {
    local run_community=false

    for arg in "$@"; do
        case "$arg" in
            --community) run_community=true ;;
        esac
    done

    echo ""
    echo "  ╦  ╔═╗╔═╗╦ ╦╔═╗"
    echo "  ║  ║ ║║  ║ ║╚═╗"
    echo "  ╩═╝╚═╝╚═╝╚═╝╚═╝"
    echo "  Development Server"
    echo ""

    check_deps

    if [ "$run_community" = true ]; then
        # Community services mode
        for port in 8090 8081 8082; do
            pid=$(lsof -ti ":$port" 2>/dev/null) && {
                log_warn "Killing process on port $port (PID $pid)"
                kill -9 $pid 2>/dev/null || true
            }
        done
        start_db
        start_services_backend
        start_forum_frontend
        start_status_frontend

        echo ""
        log_success "Community services running!"
        echo "  Services API:  http://localhost:8090"
        echo "  Forum:         http://localhost:8081/forum"
        echo "  Status:        http://localhost:8082/status"
        echo ""
        log_info "Press Ctrl+C to stop all servers"
        echo ""

        wait $SERVICES_PID $FORUM_PID $STATUS_PID
    else
        # Main app mode
        for port in 3000 8080; do
            pid=$(lsof -ti ":$port" 2>/dev/null) && {
                log_warn "Killing process on port $port (PID $pid)"
                kill -9 $pid 2>/dev/null || true
            }
        done
        ensure_env
        validate_env || exit 1
        print_config
        start_db
        start_backend
        start_frontend

        echo ""
        log_success "Development servers running!"
        echo ""
        log_info "Press Ctrl+C to stop all servers"
        echo ""

        wait $BACKEND_PID $FRONTEND_PID
    fi
}

main "$@"
