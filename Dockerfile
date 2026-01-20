# Multi-stage build for Rust bot
FROM rust:latest as builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml .
COPY Cargo.lock* ./
COPY crates/ ./crates/
COPY bins/ ./bins/

# Build the bot
RUN cargo build --release --bin arb-daemon

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/arb-daemon /usr/local/bin/arb-daemon

# Copy config
COPY config/ ./config/

# Expose API port
EXPOSE 8080

# Run the bot
CMD ["arb-daemon"]
