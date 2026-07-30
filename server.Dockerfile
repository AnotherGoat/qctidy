FROM rust:1.90-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release -p qctidy-server

FROM debian:trixie-slim
COPY --from=build /app/target/release/qctidy-server /usr/local/bin/qctidy-server
CMD ["qctidy-server"]
