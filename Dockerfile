# Stage 1: Build the application
FROM rust:1.75 AS builder

# Create a new empty shell project
RUN USER=root cargo new --bin exam_gateway
WORKDIR /exam_gateway

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create the final image
FROM debian:bookworm-slim

# Set the RUST_LOG environment variable
ENV DOCKER_ENV=true

# Copy the build artifact from the builder stage
COPY --from=builder /sentinela_exam_gateway/target/release/exam_gateway /usr/local/bin/exam_gateway

# Expose the port the app runs on
EXPOSE 8080

# Run the application
CMD ["exam_gateway"]