FROM rust:1.90-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release -p qsimplify-server

FROM debian:trixie-slim
COPY --from=build /app/target/release/qsimplify-server /usr/local/bin/qsimplify-server
CMD ["qsimplify-server"]
