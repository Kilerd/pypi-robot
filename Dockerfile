FROM clux/muslrust:stable as builder

WORKDIR /app
COPY . /app
RUN cargo build --release
FROM alpine:latest

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pypirobot /application/pypirobot

EXPOSE 8000

WORKDIR /application

ENV TELEGRAM_BOT_TOKEN TELEGRAM_WHITE_LIST
ENV RUST_LOG pypirobot=INFO

CMD ["./pypirobot"]