# Stage 1: Build dependencies and binary
FROM rust:1.75-slim-bookworm AS builder

WORKDIR /usr/src/taskmaneger

# Pre-build dependencies to cache them in Docker layer
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Copy real source code
COPY src ./src

# Rebuild the real binary (we touch main.rs and lib.rs to ensure cargo rebuilds)
RUN touch src/main.rs src/lib.rs && \
    cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install certificates and update packages
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary
COPY --from=builder /usr/src/taskmaneger/target/release/task_maneger /app/taskmanager

# Ensure user permissions and run-time setup
RUN useradd -ms /bin/bash appuser && chown -R appuser:appuser /app
USER appuser

# By default, run the interactive CLI
CMD ["/app/taskmanager"]
