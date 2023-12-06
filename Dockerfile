FROM cgr.dev/chainguard/static
WORKDIR /app
COPY ./target/x86_64-unknown-linux-musl/release/controller /app/
EXPOSE 8080
ENTRYPOINT ["/app/controller"]
