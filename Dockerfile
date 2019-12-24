FROM clux/muslrust:nightly-2019-12-21 as builder

WORKDIR /app

RUN USER=root cargo new pypirobot
WORKDIR /app/pypirobot

COPY --from=rust /app/Cargo.toml ./
COPY --from=rust /app/Cargo.lock ./

RUN echo 'fn main() { println!("Dummy") }' > ./src/main.rs

RUN cargo build --release

RUN rm -r target/x86_64-unknown-linux-musl/release/.fingerprint/pypirobot-*

COPY --from=rust /app/src src/
COPY --from=rust /app/templates templates/

RUN cargo build --release --frozen --bin pypirobot


FROM alpine:latest

COPY --from=builder /app/pypirobot/target/x86_64-unknown-linux-musl/release/pypirobot /application/pypirobot

EXPOSE 8000

ENV DATABASE_URL postgres://root@postgres/resource
ENV TELEGRAM_BOT_SECRET_KEY TELEGRAM_BOT_SECRET_KEY
ENV TELEGRAM_WHITE_LIST TELEGRAM_WHITE_LIST

WORKDIR /application

ENV RUST_LOG pypirobot=INFO

CMD ["./pypirobot"]