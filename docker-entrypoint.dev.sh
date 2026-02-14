#!/usr/bin/env bash
#
# Locus Development Entrypoint
# Starts backend and frontend inside the dev container.
#

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $*"; }
log_success() { echo -e "${GREEN}[OK]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }

BACKEND_PID=
FRONTEND_PID=

cleanup() {
    log_info "Shutting down..."
    [ -n "$BACKEND_PID" ] && kill "$BACKEND_PID" 2>/dev/null || true
    [ -n "$FRONTEND_PID" ] && kill "$FRONTEND_PID" 2>/dev/null || true
    wait 2>/dev/null || true
    exit 0
}

trap cleanup SIGINT SIGTERM

# Create .env from example if it doesn't exist
if [ ! -f .env ]; then
    if [ -f .env.example ]; then
        log_info "Creating .env from .env.example"
        cp .env.example .env
    fi
fi

# Override DATABASE_URL to point to the db service in Docker network
export DATABASE_URL="postgres://locus:locus_dev_password@db:5432/locus"

echo ""
echo "  ╦  ╔═╗╔═╗╦ ╦╔═╗"
echo "  ║  ║ ║║  ║ ║╚═╗"
echo "  ╩═╝╚═╝╚═╝╚═╝╚═╝"
echo "  Docker Dev Environment"
echo ""
echo "  Frontend:  http://localhost:8080"
echo "  Backend:   http://localhost:3000"
echo "  Database:  db:5432"
echo ""

# Start backend
log_info "Starting backend on port ${PORT:-3000}..."
cargo run -p locus-backend &
BACKEND_PID=$!

# Start frontend (trunk serve)
log_info "Starting frontend on port 8080..."
cd crates/frontend
LOCUS_API_URL="${LOCUS_API_URL:-http://localhost:3000/api}" \
LOCUS_FRONTEND_URL="${LOCUS_FRONTEND_URL:-http://localhost:8080}" \
LOCUS_ENV="${LOCUS_ENV:-development}" \
trunk serve --address 0.0.0.0 &
FRONTEND_PID=$!
cd /workspace

echo ""
log_success "Development servers running!"
log_info "Press Ctrl+C to stop all servers"
echo ""

# Wait for either process to exit
wait $BACKEND_PID $FRONTEND_PID
