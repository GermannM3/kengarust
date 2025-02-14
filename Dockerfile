FROM rust:1.81-slim as builder

WORKDIR /app
COPY . .
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev protobuf-compiler && \
    cargo build --release --features autonomous

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3
COPY --from=builder /app/target/release/kengarust /usr/local/bin/
ENTRYPOINT ["kengarust"]
