# Multi-stage build for Leptos app
FROM rust:latest as builder

# Install cargo-leptos and dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Add WebAssembly target for Leptos frontend
RUN rustup target add wasm32-unknown-unknown

# Install cargo-leptos (without --locked to avoid version conflicts)
RUN cargo install cargo-leptos

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src
COPY style ./style
COPY public ./public
COPY migrations ./migrations

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
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"

# Expose the port
EXPOSE 3000

# Run the application
CMD ["/app/shrtnr"]
