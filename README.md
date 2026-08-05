# Casper NCTL 2 Docker

Local multi-node [Casper](https://docs.casper.network/) testnet in Docker ([NCTL](https://docs.casper.network/)), plus a Rust **MCP** server so tools and agents can start/stop the network, read faucet/user/node keys, and inspect logs.

Images:

- NCTL: [`interchouette/casper-nctl-2-docker`](https://hub.docker.com/r/interchouette/casper-nctl-2-docker) · `ghcr.io/interchouette-itc/casper-nctl-2-docker`
- MCP: [`interchouette/casper-nctl-2-docker-mcp`](https://hub.docker.com/r/interchouette/casper-nctl-2-docker-mcp) · `ghcr.io/interchouette-itc/casper-nctl-2-docker-mcp`

Assets under `./assets` are **testnet-only**. Do not use them on mainnet.

## MCP

Control and debug the local testnet over MCP (stdio or Streamable HTTP on port **8790**).

Pull and run (no Rust toolchain required):

```bash
docker pull interchouette/casper-nctl-2-docker-mcp:2.2
docker run --rm -d --name casper-nctl-2-docker-mcp \
  -p 8790:8790 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v "$PWD":/workspace \
  -e NCTL_DOCKER_ROOT=/workspace \
  -e NCTL_HOST_ROOT="$PWD" \
  interchouette/casper-nctl-2-docker-mcp:2.2
```

From a clone:

```sh
make start-all 2.2      # NCTL + MCP at http://127.0.0.1:8790/mcp
make stop-all 2.2
make mcp-http           # MCP only (pulls Hub image)
make run-mcp            # stdio on the host (needs Rust)
```

Cursor: `"url": "http://127.0.0.1:8790/mcp"` — see [`mcp/mcp.json.example`](mcp/mcp.json.example) and [`docs/mcp.md`](docs/mcp.md).

`make start` starts **NCTL only**. Use `start-all` / `mcp-http` when you want MCP.

## Quick start (image)

```bash
docker pull interchouette/casper-nctl-2-docker:stable
docker run --rm -it interchouette/casper-nctl-2-docker:2.2
```

## Quick start (clone)

```sh
make start 2.2          # NCTL only
make start-all 2.2      # NCTL + MCP
make stop-all 2.2
```

## Profiles

Default: **`stable`**.

| Profile          | Node   | Client | Sidecar |
| ---------------- | ------ | ------ | ------- |
| `1.5.8` / `1.6`  | v1.5.8 | v2.0.0 | —       |
| `stable` / `2.2` | v2.2.2 | v5.0.1 | v2.1.0  |
| `2.0`            | v2.0.4 | v5.0.0 | v2.0.0  |
| `2.1`            | v2.1.2 | v5.0.0 | v2.0.0  |
| `dev`            | dev    | dev    | dev     |

Published tags: `1.5.8`, `1.6`, `2.0`, `2.1`, `stable`, `2.2`, `latest`, `dev`.

## Make

```sh
make build 2.2
make start 2.2
make start-log 2.2
make build-start 2.2
make stop 2.2
make start-all 2.2
make stop-all 2.2
make mcp-http
make mcp-http-stop
make run-mcp
make run-mcp-http
```

## Ports

| Range / port  | Role                  |
| ------------- | --------------------- |
| `11101-11105` | JSON-RPC              |
| `14101-14105` | REST                  |
| `18101-18105` | SSE                   |
| `25101-25105` | Node sidecar (2.x)    |
| `28101-28105` | Additional            |
| `11100`       | CORS proxy (optional) |
| `8790`        | MCP HTTP (`/mcp`)     |

Up check: container `casper-nctl-2-docker-<profile>`; RPC `http://127.0.0.1:11101/rpc`; MCP `http://127.0.0.1:8790/mcp`.

## Volumes

Host `./assets` → faucet, users, chainspec, nodes (logs under `assets/nodes` and `assets/logs`).

## CORS

Optional proxy on **11100** for browser apps calling NCTL RPC:

```sh
docker compose -f docker/docker-compose.yml --profile 2.2 up -d
docker compose -f docker/docker-compose.yml --profile cors-anywhere up -d
```

## Docs

[`docs/`](docs/): [getting started](docs/getting-started.md) · [MCP](docs/mcp.md) · [ports](docs/ports-and-rpc.md) · [assets & logs](docs/assets-and-logs.md) · [profiles](docs/profiles.md) · [versioning](docs/versioning.md)

## License

[Apache-2.0](LICENSE). Copyright Interchouette.
