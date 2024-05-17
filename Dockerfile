# Builder stage
FROM rust:1.78.0 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app


RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*

# Get the binary
COPY --from=builder /app/target/release/jaas jaas

# The config files
COPY configuration configuration

# Set correct env var telling the app what environment its on
ENV APP_ENVIRONMENT production

# Run the binary
ENTRYPOINT ["./jaas"]
