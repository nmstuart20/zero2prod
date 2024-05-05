FROM rust:1.77.2
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN  apt update && apt install lld clang -y 
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./target/release/zero2prod" ]