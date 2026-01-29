# Build stage
FROM rust:1.75-bookworm AS builder

WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/rust-article-recorder /app/rust-article-recorder
COPY .env.example /app/.env

EXPOSE 3000

CMD ["./rust-article-recorder"]
