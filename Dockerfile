# Multi-stage build for Leptos app with Tailwind assets
FROM rust:latest as builder

# Install build tooling, Node.js (for Tailwind) and cargo-leptos dependencies
ARG NODE_MAJOR=20
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    curl \
    gnupg \
    && curl -fsSL https://deb.nodesource.com/setup_${NODE_MAJOR}.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Add WebAssembly target for Leptos frontend
RUN rustup target add wasm32-unknown-unknown

# Install cargo-leptos (without --locked to avoid version conflicts)
RUN cargo install cargo-leptos

# Create app directory
WORKDIR /app

# Install frontend build dependencies for Tailwind
COPY package*.json ./
RUN npm ci

# Copy manifests and sources
COPY Cargo.toml ./
COPY tailwind.config.cjs postcss.config.cjs ./
COPY style ./style
COPY public ./public
COPY migrations ./migrations
COPY src ./src

# Build Tailwind styles before the Rust build
RUN npm run tailwind:build

# Keep wasm-bindgen CLI aligned with the version expected by cargo-leptos
ENV LEPTOS_WASM_BINDGEN_VERSION=0.2.105

# Build the application
RUN cargo leptos build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    postgresql-client \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built application from builder
COPY --from=builder /app/target/release/shrtnr /app/shrtnr
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/migrations /app/migrations

# Set environment variables
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"

# Expose the port
EXPOSE 8080

# Run the application
CMD ["/app/shrtnr"]
