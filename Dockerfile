# Сборка приложения
# https://hub.docker.com/_/rust/tags
FROM rust:1.88.0-slim-bookworm AS builder

WORKDIR /app
# Install build dependencies if needed
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy only files needed for dependency resolution first (for better caching)
COPY Cargo.toml ./

# Create dummy src to build dependencies
RUN mkdir -p src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/deps/thread_api* target/release/thread_api*

# Copy actual source code and rebuild
COPY .sqlx ./.sqlx
COPY migrations ./migrations
COPY src ./src
COPY config ./config
COPY static ./static

# Build application
RUN cargo build --release && \
    strip target/release/thread_api

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies only
RUN apt-get update && \
    apt-get install -y ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/thread_api /app/
COPY --from=builder /app/config /app/config
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/static /app/static

# Create a non-root user to run the application
RUN useradd -m appuser && \
    chown -R appuser:appuser /app

USER appuser

EXPOSE 5000

CMD ["./thread_api"]