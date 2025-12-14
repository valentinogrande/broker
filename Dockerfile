FROM rust:1.91 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y pkg-config libssl-dev curl openssl

WORKDIR /app

RUN mkdir -p ./uploads/files ./uploads/profile_pictures

RUN mkdir -p /etc/broker

COPY master.key.v /etc/broker

COPY --from=builder /app/target/release/broker /usr/local/bin/broker

CMD ["broker"]
