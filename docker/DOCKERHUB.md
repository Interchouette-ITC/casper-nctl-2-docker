# Casper NCTL 2 Docker

Containerized Casper NCTL for local testnets (1.x and 2.x), with an optional CORS proxy for browser apps.

Source: https://github.com/gRoussac/casper-nctl-2-docker

Docker Hub: [`interchouette/casper-nctl`](https://hub.docker.com/r/interchouette/casper-nctl)  
Legacy mirror: [`gregoshop/casper-nctl`](https://hub.docker.com/r/gregoshop/casper-nctl)

## Available Tags

**1.5.8** – Legacy Casper 1.x

```
BRANCH_NODE=v1.5.8
BRANCH_CLIENT=v2.0.0
```

**stable** (default) – Current recommended release (same as 2.2)

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

**2.0** / **2.1** – Earlier 2.x pins (build locally via compose profiles)

**dev** – Latest development branches for node, client, and sidecar

```
BRANCH_NODE=dev
BRANCH_CLIENT=dev
BRANCH_SIDECAR=dev
```

## Running a Container

```bash
docker pull interchouette/casper-nctl:stable
docker run --rm -it interchouette/casper-nctl:2.2
```

Detached:

```bash
docker run -d --name casper-nctl-stable interchouette/casper-nctl:stable
```

## Ports

`11101-11105`, `14101-14105`, `18101-18105`, `25101-25105`, `28101-28105`

## Volumes

Host `./assets` maps to NCTL faucet, users, chainspec, and node data (including logs). Testnet-only keys.

## CORS proxy

Optional CORS proxy on port `11100` for browser apps calling NCTL RPC:

```bash
docker compose --profile 2.2 up -d
docker compose --profile cors-anywhere up -d
```
