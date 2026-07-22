# syntax=docker/dockerfile:1-labs
# Steps to run the image:
# docker build -f server.Dockerfile -t qsimplify-server .
# docker run --rm -p 3000:3000 qsimplify-server

FROM rust:1.90-alpine3.22 AS builder

ARG UID=1000
ARG GID=1000

# Install Graphviz, TLS, and other build dependencies
RUN apk add --no-cache \
    build-base=0.5-r3 \
    openssl-dev=3.5.7-r0 \
    openssl-libs-static=3.5.7-r0 \
    graphviz-dev=12.2.1-r0 \
    pkgconf=2.4.3-r0

WORKDIR /app

# Copy Cargo configuration files
COPY Cargo.toml Cargo.lock ./

# Remove unneeded workspace members
RUN sed -i \
    -e '/"benchmarks"/d' \
    -e '/"crates\/cli"/d' \
    -e '/"crates\/qiskit"/d' \
    -e '/"crates\/tui"/d' \
    -e '/^crossterm = /d' \
    -e '/^fuzzy-matcher = /d' \
    -e '/^pyo3 = /d' \
    -e '/^ratatui = /d' \
    Cargo.toml

# Copy Cargo configuration files for workspace crates
COPY --parents \
    crates/analyzer/Cargo.toml \
    crates/codegen/Cargo.toml \
    crates/converter/Cargo.toml \
    crates/estimator/Cargo.toml \
    crates/facade/Cargo.toml \
    crates/library/Cargo.toml \
    crates/ports/Cargo.toml \
    crates/presenter/Cargo.toml \
    crates/server/Cargo.toml \
    ./

# Add dummy source files for better layer caching, then build the dependencies in release mode
RUN mkdir -p \
        crates/analyzer/src \
        crates/codegen/src \
        crates/converter/src \
        crates/estimator/src \
        crates/facade/src \
        crates/library/src \
        crates/ports/src \
        crates/presenter/src \
        crates/server/src && \
    printf '' > crates/analyzer/src/lib.rs && \
    printf '' > crates/codegen/src/lib.rs && \
    printf '' > crates/converter/src/lib.rs && \
    printf '' > crates/estimator/src/lib.rs && \
    printf '' > crates/facade/src/lib.rs && \
    printf '' > crates/library/src/lib.rs && \
    printf '' > crates/ports/src/lib.rs && \
    printf '' > crates/presenter/src/lib.rs && \
    printf 'fn main() {}\n' > crates/server/src/main.rs && \
    cargo build --release -p qsimplify-server

# Copy the actual source files
COPY --parents \
    crates/analyzer/src \
    crates/codegen/src \
    crates/converter/src \
    crates/estimator/src \
    crates/facade/src \
    crates/library/src \
    crates/ports/src \
    crates/presenter/src \
    crates/server/src \
    ./

# Remove previously-built dummy artifacts, then build in release mode and change the owner of the built binary
RUN cargo clean --release \
        -p qsimplify \
        -p qsimplify-analyzer \
        -p qsimplify-codegen \
        -p qsimplify-converter \
        -p qsimplify-estimator \
        -p qsimplify-facade \
        -p qsimplify-ports \
        -p qsimplify-presenter \
        -p qsimplify-server && \
    cargo build --release -p qsimplify-server && \
    cp target/release/qsimplify-server /usr/local/bin/ && \
    chown "$UID:$GID" /usr/local/bin/qsimplify-server

FROM alpine:3.22 AS runtime

ARG UID=1000
ARG GID=1000

# Create non-root user
RUN addgroup -g "$GID" appgroup && \
    adduser -D -u "$UID" -G appgroup appuser

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates=20260611-r0 \
    graphviz=12.2.1-r0 \
    openssl=3.5.7-r0

# Copy the binary built in the previous stage
COPY --from=builder /usr/local/bin/qsimplify-server /usr/local/bin/qsimplify-server

ENV API_HOST=0.0.0.0
ENV API_PORT=3000

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget -q -O /dev/null "http://127.0.0.1:${API_PORT}/health" || exit 1

USER appuser

CMD ["qsimplify-server"]
