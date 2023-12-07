FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && apt-get install -y build-essential \
    openssl libssl-dev \
    pkg-config \
    zlib1g-dev \
    protobuf-compiler
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json --bin controller --release
COPY . .
RUN cargo build --bin controller --release

FROM debian:stable-slim AS runtime
RUN apt-get update
RUN apt-get install libssl-dev -y
WORKDIR /app
COPY --from=builder /app/target/release/controller /usr/local/bin
ENTRYPOINT ["/usr/local/bin/controller"]