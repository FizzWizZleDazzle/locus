# Multi-stage Dockerfile for Locus backend + frontend
# For Kubernetes deployment

# ============================================
# Stage 1: Build Backend
# ============================================
FROM rust:1.93.0-slim AS backend-builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libgmp-dev \
    cmake \
    g++ \
    git \
    && rm -rf /var/lib/apt/lists/*

# Build and install SymEngine to /usr/local/lib
RUN git clone https://github.com/symengine/symengine.git /tmp/symengine && \
    cd /tmp/symengine && \
    mkdir build && cd build && \
    cmake -DCMAKE_BUILD_TYPE=Release \
          -DBUILD_SHARED_LIBS=OFF \
          -DCMAKE_INSTALL_PREFIX=/usr/local \
          .. && \
    make -j$(nproc) && \
    make install && \
    cd / && rm -rf /tmp/symengine

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build backend in release mode
RUN cargo build --release -p locus-backend

# ============================================
# Stage 2: Runtime
# ============================================
FROM debian:trixie-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libgmp10 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 locus && \
    mkdir -p /app/dist && \
    chown -R locus:locus /app

WORKDIR /app

# Copy backend binary from builder
COPY --from=backend-builder /build/target/release/locus-backend ./locus-backend

# Copy migrations
COPY crates/backend/migrations ./migrations

# Switch to non-root user
USER locus

# Expose port
EXPOSE 28743

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:28743/health || exit 1

# Run backend (migrations run automatically on startup)
CMD ["./locus-backend"]
