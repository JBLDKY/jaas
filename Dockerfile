# Builder stage
FROM lukemathwalker/cargo-chef:latest-rust-1.78.0 as chef

WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

# Build project dependencies
RUN cargo chef cook --release --recipe-path recipe.json


COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin jaas

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app


RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	# cleanup
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
