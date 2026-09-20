FROM rust:1.90-slim AS build

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

    WORKDIR /app

COPY . .

RUN cargo build --release -p qctidy-server

FROM debian:trixie-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/qctidy-server /usr/local/bin/qctidy-server

ENV API_HOST=0.0.0.0
ENV API_PORT=3000

EXPOSE 3000

CMD ["qctidy-server"]
