# Getting started

## What you get

A multi-node **Casper local testnet** (NCTL) packaged as Docker images, with host-mounted `./assets` for faucet keys, users, chainspec, and node logs.

Keys under `./assets` are **testnet-only**. Do not reuse them elsewhere.

## Option A — pull and run

```bash
docker pull interchouette/casper-nctl-2-docker:stable
docker run --rm -it interchouette/casper-nctl-2-docker:2.2
```

This starts NCTL inside the container. For bind-mounted assets and Make workflows, use a clone (option B).

## Option B — clone + Make

```bash
git clone https://github.com/gRoussac/casper-nctl-2-docker.git
cd casper-nctl-2-docker
make start 2.2          # NCTL only (existing behavior)
# or
make start-all 2.2      # NCTL + MCP HTTP on :8790
```

First build of a profile can take a long time (compiles node/client/sidecar). Prefer published tags when you only need to run.

## Is it up?

1. `docker ps` shows `casper-nctl-2-docker-<profile>`.
2. RPC: POST `info_get_status` to `http://127.0.0.1:11101/rpc`.
3. If you used `start-all`, MCP is at `http://127.0.0.1:8790/mcp`.

## Next

- [Profiles](profiles.md) · [Ports](ports-and-rpc.md) · [Assets & logs](assets-and-logs.md) · [MCP](mcp.md)
