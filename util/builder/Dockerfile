ARG GHC_VERSION=9.12
FROM haskell:${GHC_VERSION} AS builder

ARG GHC_VERSION
ENV GHC_VERSION=${GHC_VERSION}

RUN apt-get update && apt-get install -y postgresql-common \
    && /usr/share/postgresql-common/pgdg/apt.postgresql.org.sh -y \
    && apt-get update && apt-get install -y libpq-dev

WORKDIR /work
COPY stack.yaml stack.yaml.lock /work/
COPY common/hpack/lib.yaml /work/common/hpack/lib.yaml
COPY lib/core/package.yaml /work/lib/core/package.yaml
COPY lib/db/package.yaml /work/lib/db/package.yaml
COPY lib/api/package.yaml /work/lib/api/package.yaml
COPY lib/api-handlers/package.yaml /work/lib/api-handlers/package.yaml
COPY app/home-api-server/package.yaml /work/app/home-api-server/package.yaml

RUN ls -lah
RUN ghc --version
RUN --mount=type=cache,target=/root/.stack stack build \
    --only-dependencies \
    --system-ghc \
    --compiler=ghc-${GHC_VERSION} \
    --local-bin-path=bin
