# Casper NCTL 2 Docker

Casper NCTL 2 Docker provides containerized environments for running Casper NCTL 1.x and 2.x, supporting different branches and configurations for stable, development, and version-specific releases.

Source: https://github.com/gRoussac/casper-nctl-2-docker

## Available Tags

**1.5.8** – Legacy Casper 1.x

```
BRANCH_NODE=v1.5.8
BRANCH_CLIENT=v2.0.0
```

**stable** (default) – Current recommended release

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

**2.2**

```
BRANCH_NODE=v2.2.2
BRANCH_CLIENT=v5.0.1
BRANCH_SIDECAR=v2.1.0
```

**dev** – Tracks latest development branches for node, client, and sidecar

```
BRANCH_NODE=dev
BRANCH_CLIENT=dev
BRANCH_SIDECAR=dev
```

## Running a Container

```bash
docker run --rm -it interchouette/casper-nctl:2.2
```

Detached:

```bash
docker run -d --name casper-nctl-stable interchouette/casper-nctl:stable
```

Replace `2.2` / `stable` with `1.5.8`, `2.0`, `2.1`, or `dev` as needed.

## Ports

`11101-11105`, `14101-14105`, `18101-18105`, `25101-25105`, `28101-28105`

## Volumes

Host `./assets` maps to NCTL logs, faucet, users, chainspec, and node data.

## CORS proxy

Optional CORS proxy on port `11100` via the `cors-anywhere` compose profile:

```bash
docker compose --profile 2.2 up -d
docker compose --profile cors-anywhere up -d
```
