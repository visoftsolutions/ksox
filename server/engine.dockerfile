FROM rust:1-alpine AS chef
# Use apk for package management in Alpine
RUN apk add --no-cache build-base libressl-dev protoc protobuf-dev
RUN cargo install cargo-chef

FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
# sqlx ensure offline mode
ARG SQLX_OFFLINE
ENV SQLX_OFFLINE ${SQLX_OFFLINE}
RUN cargo build --release -p engine

# We do not need the Rust toolchain to run the binary!
FROM alpine AS runtime
COPY --from=builder /app/target/release/engine /usr/local/bin
ENTRYPOINT [ "engine" ]
