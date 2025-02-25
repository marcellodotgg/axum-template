# Start with the official Rust image for building the project
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

RUN cargo install sqlx-cli --locked

# Copy the project files
COPY . .

# Build the project in release mode
RUN cargo sqlx migrate run
RUN cargo build --release

# Use a minimal base image for running the application
FROM debian:bookworm-slim
RUN apt update && apt install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/APP_NAME .
COPY .env .
COPY data.db .

# Expose any necessary ports (change as needed)
EXPOSE 8080

# Set the entrypoint to run the compiled binary
ENTRYPOINT ["/app/APP_NAME"]
