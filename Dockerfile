FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json --bin controller --release
COPY . .
RUN cargo build --bin controller --release

FROM gcr.io/distroless/cc-debian12 AS runtime
COPY --from=builder /app/target/release/controller ./
ENTRYPOINT ["/controller"]