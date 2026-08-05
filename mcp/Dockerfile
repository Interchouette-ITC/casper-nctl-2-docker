# casper-nctl-2-docker-mcp — Streamable HTTP on :8791
# Build context: mcp/

ARG RUST_IMAGE=rust:slim-bookworm
ARG RUNTIME_IMAGE=debian:bookworm-slim

FROM ${RUST_IMAGE} AS builder

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates pkg-config binutils \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY README.md ./

RUN cargo build --release \
  && strip -s target/release/casper-nctl-2-docker-mcp \
  && cp target/release/casper-nctl-2-docker-mcp /casper-nctl-2-docker-mcp

FROM ${RUNTIME_IMAGE} AS runtime

ARG DOCKER_STATIC_VERSION=27.3.1
ARG COMPOSE_VERSION=2.29.7

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates curl make \
  && curl -fsSL "https://download.docker.com/linux/static/stable/x86_64/docker-${DOCKER_STATIC_VERSION}.tgz" \
    | tar -xz -C /tmp \
  && mv /tmp/docker/docker /usr/local/bin/docker \
  && rm -rf /tmp/docker \
  && mkdir -p /usr/local/lib/docker/cli-plugins \
  && curl -fsSL "https://github.com/docker/compose/releases/download/v${COMPOSE_VERSION}/docker-compose-linux-x86_64" \
    -o /usr/local/lib/docker/cli-plugins/docker-compose \
  && chmod +x /usr/local/bin/docker /usr/local/lib/docker/cli-plugins/docker-compose \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /casper-nctl-2-docker-mcp /usr/local/bin/casper-nctl-2-docker-mcp

ENV NCTL_MCP_HTTP=1 \
    NCTL_MCP_ADDR=0.0.0.0:8791 \
    NCTL_DOCKER_ROOT=/workspace \
    DOCKER_HOST=unix:///var/run/docker.sock \
    SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

EXPOSE 8791

WORKDIR /workspace
CMD ["casper-nctl-2-docker-mcp", "--http", "--listen", "0.0.0.0:8791"]
