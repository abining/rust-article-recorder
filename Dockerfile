# Build stage
FROM rust:1.88-bookworm AS builder

WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release

# Run stage
FROM debian:bookworm-slim

WORKDIR /app


# Copy the binary from the builder stage
COPY --from=builder /app/target/release/rust-article-recorder /app/rust-article-recorder
COPY .env.example /app/.env

EXPOSE 3000

CMD ["./rust-article-recorder"]
