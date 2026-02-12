# Multi-stage Dockerfile for Locus backend + frontend
# For Kubernetes deployment

# ============================================
# Stage 1: Build Backend
# ============================================
FROM rust:1.83-slim AS backend-builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libgmp-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build backend in release mode
RUN cargo build --release -p locus-backend

# ============================================
# Stage 2: Build Frontend
# ============================================
FROM rust:1.83-slim AS frontend-builder

# Install dependencies for trunk and wasm
RUN apt-get update && apt-get install -y \
    curl \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install trunk
RUN cargo install --locked trunk

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Copy symengine.js submodule
COPY symengine.js ./symengine.js

# Build frontend
WORKDIR /build/crates/frontend
RUN trunk build --release

# ============================================
# Stage 3: Runtime
# ============================================
FROM debian:bookworm-slim

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

# Copy frontend static assets from builder
COPY --from=frontend-builder /build/crates/frontend/dist ./dist

# Copy migrations
COPY crates/backend/migrations ./migrations

# Switch to non-root user
USER locus

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run backend (migrations run automatically on startup)
CMD ["./locus-backend"]
