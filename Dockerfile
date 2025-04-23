# Build stage
FROM rust:1.85-slim-bullseye AS builder

WORKDIR /usr/src/app

COPY ./src ./src
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/payment-gateway /app/payment-gateway

CMD [ "/app/payment-gateway" ]