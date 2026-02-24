#!/bin/bash
#
# Locus Dev Container Post-Create Script
# =======================================
# Lightweight verification + setup. All heavy compilation is baked into
# the Docker image. This script runs on Codespace creation (~30 seconds).
#

set -e

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info()    { echo -e "${BLUE}[INFO]${NC} $*"; }
log_success() { echo -e "${GREEN}[OK]${NC} $*"; }
log_warn()    { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error()   { echo -e "${RED}[ERROR]${NC} $*" >&2; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$REPO_ROOT"

# =============================================================================
# 1. Verify pre-baked tools
# =============================================================================

log_info "Verifying pre-baked tools..."

MISSING=0
check_tool() {
    if command -v "$1" &> /dev/null; then
        log_success "$1: $(eval "$2" 2>&1 | head -1)"
    else
        log_error "$1 not found!"
        MISSING=$((MISSING + 1))
    fi
}

check_file() {
    if [ -f "$1" ]; then
        log_success "$1 exists"
    else
        log_error "$1 not found!"
        MISSING=$((MISSING + 1))
    fi
}

check_dir() {
    if [ -d "$1" ]; then
        log_success "$1 exists"
    else
        log_error "$1 not found!"
        MISSING=$((MISSING + 1))
    fi
}

check_tool "rustc"       "rustc --version"
check_tool "cargo"       "cargo --version"
check_tool "trunk"       "trunk --version"
check_tool "cargo-watch" "cargo watch --version"
check_tool "node"        "node --version"
check_tool "tsc"         "tsc --version"
check_tool "python3"     "python3 --version"
check_tool "cmake"       "cmake --version"
check_tool "docker"      "docker --version"  # provided by docker-in-docker feature
check_file "/usr/local/lib/libsymengine.a"
check_dir  "/opt/wasi-sdk"
check_dir  "/opt/symengine-wasm/lib"

if [ "$MISSING" -gt 0 ]; then
    log_warn "$MISSING tool(s) missing — the Docker image may be outdated."
    log_warn "Try rebuilding: docker build -f .devcontainer/Dockerfile -t locus-dev ."
fi

# =============================================================================
# 2. Git submodules (defensive — initializeCommand should handle this)
# =============================================================================

log_info "Ensuring git submodules are initialized..."
git submodule update --init --recursive

# =============================================================================
# 3. Cargo fetch (mostly no-op — registry is cached in image)
# =============================================================================

log_info "Fetching Cargo registry (catches any new deps since image build)..."
cargo fetch --quiet

# =============================================================================
# 4. Factory Python venv
# =============================================================================

if [ -d /opt/factory-venv ] && [ -f "$REPO_ROOT/factory/backend/requirements.txt" ]; then
    log_info "Copying pre-built factory venv..."
    cp -a /opt/factory-venv "$REPO_ROOT/factory/backend/venv"
    # Install any new deps added since the image was built
    "$REPO_ROOT/factory/backend/venv/bin/pip" install --quiet \
        -r "$REPO_ROOT/factory/backend/requirements.txt" 2>/dev/null || true
    log_success "Factory venv ready at factory/backend/venv"
elif [ -f "$REPO_ROOT/factory/backend/requirements.txt" ]; then
    log_info "Pre-built venv not found, creating fresh..."
    python3 -m venv "$REPO_ROOT/factory/backend/venv"
    "$REPO_ROOT/factory/backend/venv/bin/pip" install --quiet \
        -r "$REPO_ROOT/factory/backend/requirements.txt"
    log_success "Factory venv ready at factory/backend/venv"
fi

# =============================================================================
# 5. Create .env files from examples if missing
# =============================================================================

if [ -f "$REPO_ROOT/.env.example" ] && [ ! -f "$REPO_ROOT/.env" ]; then
    cp "$REPO_ROOT/.env.example" "$REPO_ROOT/.env"
    log_success "Created .env from .env.example"
else
    log_info ".env already exists or no .env.example found"
fi

if [ -f "$REPO_ROOT/factory/backend/.env.example" ] && [ ! -f "$REPO_ROOT/factory/backend/.env" ]; then
    cp "$REPO_ROOT/factory/backend/.env.example" "$REPO_ROOT/factory/backend/.env"
    log_success "Created factory/backend/.env from .env.example"
else
    log_info "factory/backend/.env already exists or no .env.example found"
fi

# =============================================================================
# Summary
# =============================================================================

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Locus Development Environment Ready!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Pre-built artifacts:"
echo "  Native SymEngine: /usr/local/lib/libsymengine.a"
echo "  WASM SymEngine:   /opt/symengine-wasm/lib/"
echo "  WASI SDK:         /opt/wasi-sdk/"
echo "  Cargo target:     /opt/cargo-target/"
echo ""
echo "To start development:"
echo "  ./dev.sh              # Start backend + frontend"
echo ""
echo "Factory:"
echo "  cd factory && ./start.sh"
echo ""
