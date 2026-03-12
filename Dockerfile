# Stage 1: Builder
FROM rust:1.85-bookworm AS builder

WORKDIR /usr/src/app

# Copy the Rust project
COPY geodesic_engine ./geodesic_engine

# Build the project in release mode
WORKDIR /usr/src/app/geodesic_engine
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install minimal dependencies (if needed by dynamic linking, usually libc)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/geodesic_engine/target/release/geodesic_engine /usr/local/bin/geodesic_engine

# Expose the default RESP port
EXPOSE 6379

# Create a directory for the database file
RUN mkdir -p /data
WORKDIR /data

# Run the server
# Default args: port 6379, db at ./geodesic.db
# Note: The server binds to 0.0.0.0 by default.
ENTRYPOINT ["geodesic_engine"]
CMD ["--port", "6379", "--db-path", "/data/geodesic.db"]
