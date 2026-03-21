FROM rust:1.87.0-slim AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/services-backend/ crates/services-backend/
RUN cargo build -p locus-services-backend --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*
RUN useradd -m -u 1000 locus
USER locus
COPY --from=builder /app/target/release/locus-services-backend /usr/local/bin/
EXPOSE 8090
HEALTHCHECK --interval=30s --timeout=5s CMD curl -f http://localhost:8090/health || exit 1
CMD ["locus-services-backend"]
