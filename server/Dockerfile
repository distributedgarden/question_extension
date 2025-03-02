# Build stage
FROM rust:1.85-slim as builder

WORKDIR /usr/src/app

# Install OpenSSL development packages and pkg-config
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
RUN USER=root cargo new --bin question_extension
WORKDIR /usr/src/app/question_extension

# Copy over manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release
RUN rm -f ./target/release/deps/question_extension*
RUN cargo build --release

# Production stage
FROM debian:bookworm-slim

# Set working directory
WORKDIR /app

# Install dependencies with OpenSSL 3
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary and additional files
COPY --from=builder /usr/src/app/question_extension/target/release/question_extension /app/question_extension
COPY ./.env.example /app/.env.example

# Create a non-root user to run the application
RUN useradd -m appuser
RUN chown -R appuser:appuser /app
USER appuser

# Set environment variables
ENV RUST_LOG=info

# Expose the port
EXPOSE 5000

# Run the binary
CMD ["./question_extension"]