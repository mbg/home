FROM debian:sid-slim AS base
RUN apt-get update && \
    apt-get install -y libc6-dev && \
    rm -rf /var/lib/apt/lists/*

FROM rust:1.99 AS builder
WORKDIR /usr/src/home-assistant-exporter
COPY . .
RUN cargo install --path .

FROM base
COPY --from=builder \
    /usr/local/cargo/bin/home-assistant-exporter \
    /usr/local/bin/home-assistant-exporter
CMD ["home-assistant-exporter"]
