FROM clux/muslrust:nightly-2019-12-21 as builder

WORKDIR /app
COPY . /app
RUN cargo build --release
FROM alpine:latest

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pypirobot /application/pypirobot

EXPOSE 8000

ENV DATABASE_URL postgres://root@postgres/resource
ENV TELEGRAM_BOT_SECRET_KEY TELEGRAM_BOT_SECRET_KEY
ENV TELEGRAM_WHITE_LIST TELEGRAM_WHITE_LIST

WORKDIR /application

ENV RUST_LOG pypirobot=INFO

CMD ["./pypirobot"]