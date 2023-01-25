FROM rust AS chef 
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN apt-get update
RUN apt-get install -y protobuf-compiler
RUN cargo install cargo-chef  
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/worker /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/worker" ]
