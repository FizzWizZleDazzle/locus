#!/bin/bash
#
# Locus Development Container Setup
# Runs automatically when launching GitHub Codespaces
#

set -e

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $*"; }
log_success() { echo -e "${GREEN}[OK]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$REPO_ROOT"

# =============================================================================
# Install System Dependencies
# =============================================================================

log_info "Installing system dependencies..."
sudo apt-get update
sudo apt-get install -y \
    pkg-config \
    libssl-dev \
    libgmp-dev \
    cmake \
    g++ \
    git \
    curl \
    npm \
    python3-pip

# =============================================================================
# Install Emscripten (if not already installed)
# =============================================================================

if ! command -v emcc &> /dev/null; then
    log_info "Installing Emscripten SDK..."
    cd /tmp
    git clone https://github.com/emscripten-core/emsdk.git
    cd emsdk
    ./emsdk install latest
    ./emsdk activate latest
    cd "$REPO_ROOT"
    log_success "Emscripten installed"
else
    log_success "Emscripten already installed"
fi

# =============================================================================
# Install WASI SDK (if not already at /opt/wasi-sdk)
# =============================================================================

if [ ! -d /opt/wasi-sdk ]; then
    log_info "Installing WASI SDK..."
    cd /tmp
    curl -sL https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-21/wasi-sdk-21.0-linux.tar.gz -o wasi-sdk.tar.gz
    tar -xzf wasi-sdk.tar.gz
    sudo mv wasi-sdk-21.0 /opt/wasi-sdk
    rm wasi-sdk.tar.gz
    log_success "WASI SDK installed at /opt/wasi-sdk"
else
    log_success "WASI SDK already installed"
fi

# =============================================================================
# Build SymEngine WASM library
# =============================================================================

if [ ! -f "$REPO_ROOT/symengine.js/dist/wasm-unknown/lib/libsymengine.a" ]; then
    log_info "Building SymEngine WASM library..."
    cd "$REPO_ROOT/symengine.js"
    make wasm-unknown
    log_success "SymEngine WASM library built"
else
    log_success "SymEngine WASM library already built"
fi

# =============================================================================
# Build native SymEngine (optional, for ./dev.sh)
# =============================================================================

if ! command -v symengine &> /dev/null && [ ! -f /usr/local/lib/libsymengine.a ]; then
    log_info "Building native SymEngine (this will take a few minutes)..."
    cd /tmp
    git clone https://github.com/symengine/symengine.git symengine-native
    cd symengine-native
    mkdir -p build && cd build
    cmake -DCMAKE_BUILD_TYPE=Release \
          -DBUILD_SHARED_LIBS=OFF \
          -DCMAKE_INSTALL_PREFIX=/usr/local \
          .. 2>/dev/null || {
        log_error "CMake 3.18+ required for native symengine, skipping..."
        log_info "You can still use the frontend - it uses WASM SymEngine"
    }
    if [ -f Makefile ]; then
        make -j$(nproc)
        sudo make install
        log_success "Native SymEngine installed"
    fi
    cd / && rm -rf /tmp/symengine-native
else
    log_success "SymEngine native already installed or not needed"
fi

# =============================================================================
# Install Rust tools
# =============================================================================

log_info "Installing Rust tools..."
rustup target add wasm32-unknown-unknown 2>/dev/null || true
cargo install trunk cargo-watch --quiet 2>/dev/null || log_error "Failed to install Rust tools (may already exist)"
log_success "Rust tools ready"

# =============================================================================
# Build workspace
# =============================================================================

log_info "Building Locus workspace (this may take a few minutes)..."
export SYMENGINE_LIB_DIR="$REPO_ROOT/symengine.js/dist/wasm-unknown/lib"
cargo build --workspace --quiet 2>&1 | grep -E "error|warning" || true
log_success "Workspace built"

# =============================================================================
# Summary
# =============================================================================

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Locus Development Environment Ready!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "To start development:"
echo "  ./dev.sh              # Start backend + frontend"
echo ""
echo "Or use Docker:"
echo "  docker compose -f docker-compose.dev.yml up"
echo ""
