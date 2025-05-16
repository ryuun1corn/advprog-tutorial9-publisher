# Use the official Rust image
FROM rust:1.87.0-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy full source and rebuild
COPY . .
RUN cargo build --release

# ---

# Final image
FROM debian:bookworm-slim

# Install minimal deps for running
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/publisher /usr/local/bin/app

# Copy .env if needed (optional for dev)
# COPY .env .env

ENV RUST_LOG=info
ENV RABBITMQ_URL=amqp://guest:guest@localhost:5672

CMD ["app"]
