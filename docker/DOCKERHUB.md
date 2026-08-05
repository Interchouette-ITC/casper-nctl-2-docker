# Casper NCTL 2 Docker

Containerized Casper NCTL for local testnets (1.x and 2.x), with an optional CORS proxy and an optional **Rust MCP** HTTP sidecar for AI agents.

**Product:** `casper-nctl-2-docker`

Source: https://github.com/gRoussac/casper-nctl-2-docker

Docker Hub: [`interchouette/casper-nctl-2-docker`](https://hub.docker.com/r/interchouette/casper-nctl-2-docker)
GHCR: `ghcr.io/interchouette-itc/casper-nctl-2-docker`
Legacy Hub names (archived / no longer published): `interchouette/casper-nctl`, `gregoshop/casper-nctl`

## Available Tags

**1.5.8** / **1.6** – Legacy Casper 1.x (`1.6` is an alias of `1.5.8`)

```
BRANCH_NODE=v1.5.8
BRANCH_CLIENT=v2.0.0
```

**stable** (default) – Current recommended release (same as `2.2` / `latest`)

```
BRANCH_NODE=v2.2.2
BRANCH_CLIENT=v5.0.1
BRANCH_SIDECAR=v2.1.0
```

**2.2**

```
BRANCH_NODE=v2.2.2
BRANCH_CLIENT=v5.0.1
BRANCH_SIDECAR=v2.1.0
```

**2.0**

```
BRANCH_NODE=v2.0.4
BRANCH_CLIENT=v5.0.0
BRANCH_SIDECAR=v2.0.0
```

**2.1**

```
BRANCH_NODE=v2.1.2
BRANCH_CLIENT=v5.0.0
BRANCH_SIDECAR=v2.0.0
```

**dev** – Latest development branches for node, client, and sidecar

```
BRANCH_NODE=dev
BRANCH_CLIENT=dev
BRANCH_SIDECAR=dev
```

## Running a Container

```bash
docker pull interchouette/casper-nctl-2-docker:stable
docker run --rm -it interchouette/casper-nctl-2-docker:2.2
```

Detached:

```bash
docker run -d --name casper-nctl-2-docker-stable interchouette/casper-nctl-2-docker:stable
```

From a git clone, Make keeps **NCTL-only** vs **NCTL+MCP** separate:

```bash
make start 2.2        # NCTL only
make start-all 2.2    # NCTL + MCP at http://127.0.0.1:8791/mcp
```

## Ports

`11101-11105` (RPC), `14101-14105` (REST), `18101-18105` (SSE), `25101-25105` (sidecar), `28101-28105`, optional CORS `11100`, optional MCP `8791`.

## Volumes

Host `./assets` maps to NCTL faucet, users, chainspec, and node data (including logs). Testnet-only keys.

## CORS proxy

Optional CORS proxy on port `11100` for browser apps calling NCTL RPC:

```bash
docker compose --profile 2.2 up -d
docker compose --profile cors-anywhere up -d
```

## MCP (agents)

Image [`interchouette/casper-nctl-2-docker-mcp`](https://hub.docker.com/r/interchouette/casper-nctl-2-docker-mcp) (`:2.2` / `:latest`): Streamable HTTP on **8791** → `/mcp`. Not inside the NCTL image. See repo `docs/mcp.md`.
