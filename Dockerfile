FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release --features autonomous

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3
COPY --from=builder /app/target/release/kengarust /usr/local/bin

CMD ["kengarust", "--autonomous"]
