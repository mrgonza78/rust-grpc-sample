# ---- Build Stage ----
FROM rustlang/rust:nightly as builder

WORKDIR /app

# Install build dependencies for static linking and protoc
RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev protobuf-compiler

# Add musl target
RUN rustup target add x86_64-unknown-linux-musl

# Copy source and build
COPY . .

# Build for musl target
RUN cargo build --release --target x86_64-unknown-linux-musl

# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Install only necessary runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the statically built binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/grpc_sample /app/server

# Cloud Run expects the app to listen on $PORT (default 8080)
ENV PORT=8080

# Expose the port (for documentation; Cloud Run maps ports automatically)
EXPOSE 8080

# Start the server
CMD ["/app/server"]
