FROM debian:sid-slim AS base
RUN apt-get update && \
    apt-get install -y libc6-dev && \
    rm -rf /var/lib/apt/lists/*

FROM rust:1.99 AS builder
WORKDIR /usr/src/home-assistant-exporter
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/home-assistant-exporter/target \
    cargo install --path .

FROM base
COPY --from=builder \
    /usr/local/cargo/bin/home-assistant-exporter \
    /usr/local/bin/home-assistant-exporter
CMD ["home-assistant-exporter", "/etc/home/config/home-assistant.yml"]
