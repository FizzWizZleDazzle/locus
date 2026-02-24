FROM ubuntu:25.04

# Install basic development tools
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    wget \
    ca-certificates \
    software-properties-common \
    apt-transport-https \
    cmake \
    pkg-config \
    libssl-dev \
    libgmp-dev \
    g++ \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Docker CLI (for docker compose)
RUN apt-get update && apt-get install -y \
    docker.io \
    && rm -rf /var/lib/apt/lists/*

# Install Python
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Install Emscripten SDK
RUN cd /tmp && \
    git clone https://github.com/emscripten-core/emsdk.git && \
    cd emsdk && \
    ./emsdk install latest && \
    ./emsdk activate latest && \
    cd / && rm -rf /tmp/emsdk
ENV EMSDK="/emsdk"
ENV PATH="${EMSDK}:${EMSDK}/emscripten/main:${PATH}"

# Install WASI SDK
RUN cd /tmp && \
    curl -sL https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-21/wasi-sdk-21.0-linux.tar.gz -o wasi-sdk.tar.gz && \
    tar -xzf wasi-sdk.tar.gz && \
    mv wasi-sdk-21.0 /opt/wasi-sdk && \
    rm wasi-sdk.tar.gz
ENV WASI_SDK_PATH="/opt/wasi-sdk"

WORKDIR /workspaces/locus

# Keep container running
CMD ["/bin/bash"]
