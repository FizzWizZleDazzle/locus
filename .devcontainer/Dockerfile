# =============================================================================
# Locus Development Container
# =============================================================================
# Bakes all tools and precompiled libraries into the image so Codespace
# startup is near-instant. Built and pushed to GHCR by CI.
#
# Local build (from repo root, after `git submodule update --init --recursive`):
#   docker build -f .devcontainer/Dockerfile -t locus-dev .
# =============================================================================

# ---- Base stage: system deps + toolchains ----
FROM ubuntu:24.04 AS base
ENV DEBIAN_FRONTEND=noninteractive

# 1. System packages
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    libgmp-dev \
    g++ \
    clang \
    llvm \
    git \
    curl \
    wget \
    ca-certificates \
    python3 \
    python3-pip \
    python3-venv \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# 2. Create non-root user (--force handles pre-existing GID/UID)
RUN groupadd --gid 1000 --force vscode \
    && useradd --uid 1000 --gid 1000 -m vscode -s /bin/bash || true \
    && echo "vscode ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/vscode

# 3. Rust toolchain (installed as vscode so ~/.cargo is in /home/vscode)
USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    --default-toolchain stable
ENV PATH="/home/vscode/.cargo/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown \
    && cargo install trunk cargo-watch cargo-chef --locked
USER root

# 4. Node.js LTS + TypeScript
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/* \
    && npm install -g typescript

# 5. WASI SDK 25 (arch-aware)
RUN ARCH="$(uname -m)" \
    && case "$ARCH" in \
        x86_64)  WASI_ARCH="x86_64" ;; \
        aarch64) WASI_ARCH="aarch64" ;; \
        arm64)   WASI_ARCH="aarch64" ;; \
        *)       echo "Unsupported arch: $ARCH" && exit 1 ;; \
    esac \
    && curl -fsSL "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-25/wasi-sdk-25.0-${WASI_ARCH}-linux.tar.gz" \
       -o /tmp/wasi-sdk.tar.gz \
    && tar -xzf /tmp/wasi-sdk.tar.gz -C /opt \
    && mv /opt/wasi-sdk-25.0-* /opt/wasi-sdk \
    && rm /tmp/wasi-sdk.tar.gz
ENV WASI_SDK_PATH="/opt/wasi-sdk"

# 6. Native SymEngine (for backend)
RUN cd /tmp \
    && git clone --depth 1 https://github.com/symengine/symengine.git \
    && cd symengine \
    && mkdir build && cd build \
    && cmake -DCMAKE_BUILD_TYPE=Release \
             -DBUILD_SHARED_LIBS=OFF \
             -DBUILD_TESTS=OFF \
             -DBUILD_BENCHMARKS=OFF \
             -DCMAKE_INSTALL_PREFIX=/usr/local \
             .. \
    && make -j"$(nproc)" \
    && make install \
    && cd / && rm -rf /tmp/symengine

# 7. WASM SymEngine (for frontend)
#    Uses symengine.js/build_wasm.sh from the repo (submodule must be checked out).
#    CI does `actions/checkout` with `submodules: recursive` so these files exist.
COPY symengine.js/build_wasm.sh /tmp/symengine-wasm-build/build_wasm.sh
COPY symengine.js/cmake/ /tmp/symengine-wasm-build/cmake/
RUN cd /tmp/symengine-wasm-build \
    && chmod +x build_wasm.sh \
    && ./build_wasm.sh --arch=unknown --install-deps \
    && mkdir -p /opt/symengine-wasm/lib \
    && cp dist/wasm-unknown/lib/*.a /opt/symengine-wasm/lib/ \
    && cd / && rm -rf /tmp/symengine-wasm-build
ENV SYMENGINE_LIB_DIR="/opt/symengine-wasm/lib"

# 8. Factory Python deps (pre-built venv)
COPY factory/backend/requirements.txt /tmp/factory-requirements.txt
RUN python3 -m venv /opt/factory-venv \
    && /opt/factory-venv/bin/pip install --no-cache-dir -r /tmp/factory-requirements.txt \
    && rm /tmp/factory-requirements.txt

# ---- Planner stage: generate cargo-chef recipe from workspace manifests ----
FROM base AS planner
USER vscode
WORKDIR /workspaces/locus
COPY --chown=vscode:vscode Cargo.toml Cargo.lock ./
COPY --chown=vscode:vscode crates/common/Cargo.toml crates/common/Cargo.toml
COPY --chown=vscode:vscode crates/backend/Cargo.toml crates/backend/Cargo.toml
COPY --chown=vscode:vscode crates/frontend/Cargo.toml crates/frontend/Cargo.toml
COPY --chown=vscode:vscode crates/common/build.rs crates/common/build.rs
# cargo-chef needs stub source files to satisfy Cargo's manifest validation
RUN mkdir -p crates/common/src crates/backend/src crates/frontend/src \
    && touch crates/common/src/lib.rs crates/backend/src/main.rs crates/frontend/src/main.rs \
    && cargo chef prepare --recipe-path /tmp/recipe.json

# ---- Final stage: cook (pre-build) all dependency crates ----
FROM base AS final

# Create cargo target dir owned by vscode
RUN mkdir -p /opt/cargo-target && chown vscode:vscode /opt/cargo-target

COPY --from=planner /tmp/recipe.json /tmp/recipe.json
ENV CARGO_TARGET_DIR=/opt/cargo-target
USER vscode
WORKDIR /workspaces/locus

# Cook native deps (axum, tokio, serde, sqlx, etc.)
# The `|| true` allows the build to succeed even if some crates fail
# (e.g. build.rs that expect runtime resources). The important thing
# is that the bulk of dependency crates are compiled and cached.
RUN cargo chef cook --recipe-path /tmp/recipe.json || true

# Cook WASM deps (leptos, wasm-bindgen, gloo, etc.)
RUN cargo chef cook --recipe-path /tmp/recipe.json --target wasm32-unknown-unknown || true

# Clean up workspace stubs and recipe, keep compiled artifacts
USER root
RUN rm -rf /workspaces/locus/* /tmp/recipe.json

USER vscode
WORKDIR /workspaces/locus
CMD ["/bin/bash"]
