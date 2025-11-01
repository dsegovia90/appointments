# Multi-stage Dockerfile for Loco.rs with Nuxt frontend
# Using cargo-chef and sccache for optimal Rust build caching
ARG CARGO_PACKAGE_NAME=appointments-cli

# Stage 1: Base image with build tools
FROM rust:1.91-slim AS rust-base

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-chef and sccache for optimal caching
RUN cargo install cargo-chef --version ^0.1
RUN cargo install sccache --version ^0.7

# Set sccache environment variables
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/sccache

# Stage 2: Plan dependencies with cargo-chef
FROM rust-base AS rust-planner
WORKDIR /app

# Copy all source files for dependency analysis
COPY Cargo.toml Cargo.lock ./
COPY migration ./migration
COPY src ./src

# Generate the cargo-chef recipe file
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies
FROM rust-base AS rust-deps
WORKDIR /app

# Copy the recipe from the planner stage
COPY --from=rust-planner /app/recipe.json recipe.json

# Build dependencies - this is the layer that gets cached
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

# Stage 4: Build the application
FROM rust-base AS rust-builder
ARG CARGO_PACKAGE_NAME
WORKDIR /app

# Copy the built dependencies from the deps stage
COPY --from=rust-deps /app/target target
COPY --from=rust-deps /usr/local/cargo /usr/local/cargo

# Copy source code
COPY Cargo.toml Cargo.lock ./
COPY migration ./migration
COPY src ./src

# Build the application binary
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release --bin ${CARGO_PACKAGE_NAME}

# Stage 5: Build Node.js frontend
FROM oven/bun:1.3.0 AS bun-builder

# Set working directory
WORKDIR /app/frontend

# Copy package files for dependency caching
COPY frontend/package.json frontend/bun.lock ./

# Install dependencies with cache mount
RUN bun install --frozen-lockfile

# Copy frontend source code
COPY frontend/ ./

# Build the Nuxt application
RUN bun run generate

# Stage 6: Runtime image
FROM debian:trixie-slim
ARG CARGO_PACKAGE_NAME

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy the built Rust binary
COPY --from=rust-builder /app/target/release/${CARGO_PACKAGE_NAME} ./${CARGO_PACKAGE_NAME}

# Copy the built frontend
COPY --from=bun-builder /app/frontend/.output ./.output

# Copy configuration files
COPY config/ ./config/

# Copy and setup entrypoint script
COPY docker-entrypoint.sh ./docker-entrypoint.sh
# Change ownership and make binary executable
RUN chmod +x ${CARGO_PACKAGE_NAME} && \
    chmod +x docker-entrypoint.sh


# Expose port (adjust if your app uses a different port)
EXPOSE 5150

# Set environment variables
ENV NODE_ENV=production
ENV LOCO_ENV=production
ENV CARGO_PACKAGE_NAME=${CARGO_PACKAGE_NAME}

# Use entrypoint with exec form CMD (prevents JSONArgsRecommended warning)
ENTRYPOINT ["./docker-entrypoint.sh"]
CMD ["start", "-e", "production"]
