# Casper NCTL 2 Docker

Containerized [Casper NCTL](https://docs.casper.network/) (Node Control Tool) for local testnets. Optional CORS proxy for browser apps talking to NCTL RPC.

Images:

- Docker Hub: [`interchouette/casper-nctl`](https://hub.docker.com/r/interchouette/casper-nctl)
- Docker Hub (legacy): [`gregoshop/casper-nctl`](https://hub.docker.com/r/gregoshop/casper-nctl)
- GHCR: `ghcr.io/interchouette-itc/casper-nctl`

```bash
docker pull interchouette/casper-nctl:stable
docker run --rm -it interchouette/casper-nctl:2.2
```

NCTL assets under `./assets` are **testnet-only** (including keys).

Agents controlling NCTL via MCP must use Cursor `CallMcpTool` (see [`agent-rules/nctl-use-mcp.mdc`](agent-rules/nctl-use-mcp.mdc)); copy into `.cursor/rules/` for Cursor.

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

## Usage

```sh
make build 2.2
make start-log 2.2
make build-start-log 2.2
make stop 2.2
```

Replace `2.2` with `stable`, `dev`, `1.5.8`, `2.0`, or `2.1` as needed.

```sh
make build-no-cache stable
make start stable
make build-start stable
```

## Ports

`11101-11105`, `14101-14105`, `18101-18105`, `25101-25105`, `28101-28105`

## Volumes

Host `./assets` maps to NCTL faucet, users, chainspec, and nodes (including node logs under `assets/nodes`).

## CORS proxy

Browser apps that call NCTL RPC can use the `cors-anywhere` profile on port **11100**:

```sh
docker compose --profile 2.2 up -d
docker compose --profile cors-anywhere up -d
```

## License

[Apache-2.0](LICENSE). Copyright Interchouette.
