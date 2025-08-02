FROM home-builder:latest AS builder
WORKDIR /work
COPY . .
RUN ls -lah
RUN ghc --version
RUN --mount=type=cache,target=/root/.stack stack install \
    --system-ghc \
    --compiler=ghc-${GHC_VERSION} \
    --local-bin-path=bin

FROM debian:bookworm

RUN apt-get update && apt-get install -y postgresql-common \
    && /usr/share/postgresql-common/pgdg/apt.postgresql.org.sh -y \
    && apt-get update && apt-get install -y libpq-dev

WORKDIR /app
COPY --from=builder /work/bin/home-api-server .
CMD [ "/app/home-api-server" ]
