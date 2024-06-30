# Builder stage
FROM rust:1.77.2-slim AS builder
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN  apt update && apt install lld clang -y 
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install open SSL
# Install ca-certs - this is needed to verify TLS certs when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./zero2prod" ]