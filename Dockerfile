# Stage 1: Build the application
FROM rust:1.88 AS builder

ARG BUCKET_NAME
ENV BUCKET_NAME=$BUCKET_NAME

ARG GOOGLE_APPLICATION_CREDENTIALS
ENV GOOGLE_APPLICATION_CREDENTIALS=$GOOGLE_APPLICATION_CREDENTIALS

# Create a new empty shell project
RUN USER=root cargo new --bin sentinela_exam_gateway
WORKDIR /sentinela_exam_gateway

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create the final image
FROM debian:bookworm-slim

# Install libss13 (needed for OpenSSL3)
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

# Set the RUST_LOG environment variable
ENV DOCKER_ENV=true

# Copy the build artifact from the builder stage
COPY --from=builder /sentinela_exam_gateway/target/release/sentinela_exam_gateway /usr/local/bin/sentinela_exam_gateway

# Expose the port the app runs on
EXPOSE 8080

# Run the application
CMD ["sentinela_exam_gateway"]