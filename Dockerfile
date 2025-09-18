# Stage 1: Build the application
FROM rust:1.88 AS builder

# Create a new empty shell project
RUN USER=root cargo new --bin sentinela_exam_receiver
WORKDIR /sentinela_exam_receiver

# Copy the source code
COPY src ./src
COPY .env .env
COPY Cargo.toml Cargo.lock ./

# Build the application
RUN cargo build --release

# Stage 2: Create the final image
FROM debian:bookworm-slim

# Create a non-root user to run the application
RUN useradd -m appuser

# Install libss13 (needed for OpenSSL3)
#RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*
RUN apt-get update && \
    apt-get install -y --no-install-recommends libssl3 ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

# Set the environment variable
ENV DOCKER_ENV=true

# Copy the build artifact from the builder stage
COPY --from=builder /sentinela_exam_receiver/target/release/sentinela_exam_receiver /usr/local/bin/sentinela_exam_receiver/sentinela_exam_receiver
COPY --from=builder /sentinela_exam_receiver/.env /usr/local/bin/sentinela_exam_receiver/.env

# Set permissions and ownership
RUN chown appuser:appuser /usr/local/bin/sentinela_exam_receiver/.env

# Switch to the non-root user
USER appuser

# Expose the port the app runs on
EXPOSE 8080

# Run the application
CMD ["sentinela_exam_receiver"]