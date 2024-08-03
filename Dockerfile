FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

# Install essential build tools and dependencies
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    llvm \
    libssl-dev \
    clang \
    gcc-multilib \
    nasm \
    cmake \
    curl \
    git \
    wget \
    unzip \
    pkg-config \
    vim \
    gdb \
    python3-pip && \
    apt-get autoremove -y && \
    apt-get clean

# Install Node.js (LTS Version) and npm
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# # Ensure the cargo environment is correctly set up
# RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force && \
#     avm install latest && \
#     avm use latest

# Install Solana CLI tools
# RUN sh -c "$(curl -sSfL https://release.solana.com/v1.18.15/install)"
# ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"

# Install Yarn
# RUN npm install -g yarn

# Install Firebase (if needed, can be removed if not required)
# RUN npm install -g firebase-tools

# Configure Solana CLI and generate a new keypair
# RUN solana-keygen new --no-passphrase -o /root/.config/solana/id.json && \
    # solana config set --url http://localhost:8899

# Set environment variables for Anchor
# ENV ANCHOR_WALLET=/root/.config/solana/id.json
# ENV ANCHOR_PROVIDER_URL=http://localhost:8899

# Reset DEBIAN_FRONTEND environment variable
ENV DEBIAN_FRONTEND=

# Set the working directory
WORKDIR /workspace

# Create entry point
CMD ["/bin/bash"]