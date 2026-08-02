# Casper NCTL 2 Docker

Local Casper testnet in Docker via [NCTL](https://docs.casper.network/) (Node Control Tool). Optional CORS proxy for browser apps, and an optional **Rust MCP** sidecar (**v2.2.2**) so Cursor (or other agents) can inspect faucet/users/nodes/logs and control the compose lifecycle.

**Product name:** `casper-nctl-2-docker` (not `casper-nctl`). NCTL images use profile tags (`2.2`, `stable`, …); the MCP sidecar is versioned separately as **`2.2.2`** — see [docs/versioning.md](docs/versioning.md) and [CHANGELOG.md](CHANGELOG.md).

Images:

- Docker Hub: [`interchouette/casper-nctl-2-docker`](https://hub.docker.com/r/interchouette/casper-nctl-2-docker)
- GHCR: `ghcr.io/interchouette-itc/casper-nctl-2-docker`
- Legacy Hub name (deprecated): `interchouette/casper-nctl` / `gregoshop/casper-nctl`

NCTL assets under `./assets` are **testnet-only** (including keys). Never use them on mainnet.

## Quick start (image only)

```bash
docker pull interchouette/casper-nctl-2-docker:stable
docker run --rm -it interchouette/casper-nctl-2-docker:2.2
```

## Quick start (clone + Make)

```sh
make start 2.2          # NCTL only (unchanged)
make start-all 2.2      # NCTL + MCP HTTP on :8788
make stop-all 2.2
```

After `start-all`, point Cursor at `http://127.0.0.1:8788/mcp` (see [docs/mcp.md](docs/mcp.md) and [mcp/mcp.json.example](mcp/mcp.json.example)).

## Documentation

Guides live in [`docs/`](docs/) (GitHub Pages when enabled): [getting started](docs/getting-started.md), [ports & RPC](docs/ports-and-rpc.md), [assets & logs](docs/assets-and-logs.md), [MCP](docs/mcp.md), [versioning](docs/versioning.md).

## Profiles

Default profile is **`stable`**.

| Profile          | Node   | Client | Sidecar |
| ---------------- | ------ | ------ | ------- |
| `1.5.8`          | v1.5.8 | v2.0.0 | —       |
| `stable` / `2.2` | v2.2.2 | v5.0.1 | v2.1.0  |
| `2.0`            | v2.0.4 | v5.0.0 | v2.0.0  |
| `2.1`            | v2.1.2 | v5.0.0 | v2.0.0  |
| `dev`            | dev    | dev    | dev     |

Published tags: `1.5.8`, `stable`, `2.2`, `dev`.

## Make targets

NCTL (unchanged):

```sh
make build 2.2
make start 2.2
make start-log 2.2
make build-start 2.2
make stop 2.2
```

NCTL + MCP, or MCP alone:

```sh
make start-all 2.2      # NCTL + MCP :8788
make stop-all 2.2
make mcp-http           # MCP sidecar only
make mcp-http-stop
make run-mcp            # host stdio MCP (Rust)
make run-mcp-http       # host HTTP MCP on 127.0.0.1:8788
```

## Ports

| Range / port | Role |
| --- | --- |
| `11101-11105` | Node JSON-RPC |
| `14101-14105` | REST |
| `18101-18105` | SSE |
| `25101-25105` | Sidecar (2.x) |
| `28101-28105` | Additional node ports |
| `11100` | CORS proxy (optional profile) |
| `8788` | MCP Streamable HTTP (`/mcp`) when using `start-all` / `mcp-http` |

How to know it is up: `docker ps` shows `casper-nctl-2-docker-<profile>`; POST `info_get_status` to `http://127.0.0.1:11101/rpc`; with MCP, use `http://127.0.0.1:8788/mcp`.

## Volumes

Host `./assets` maps to NCTL **faucet**, **users**, **chainspec**, and **nodes** (logs under `assets/nodes` and `assets/logs`).

## CORS proxy

Browser apps that call NCTL RPC can use the `cors-anywhere` profile on port **11100**:

```sh
docker compose --profile 2.2 up -d
docker compose --profile cors-anywhere up -d
```

## License

[Apache-2.0](LICENSE). Copyright Interchouette.
