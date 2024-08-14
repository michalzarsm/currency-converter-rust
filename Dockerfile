FROM rust:1.76.0-slim-bullseye as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev

WORKDIR /usr/src/currency-converter

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/currency-converter /usr/local/bin/currency-converter

CMD ["currency-converter"]