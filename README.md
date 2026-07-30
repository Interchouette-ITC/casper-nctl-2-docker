## Casper NCTL 2 Docker

**Casper NCTL 2 Docker** provides containerized environments for running **Casper NCTL** (Node Control Tool) versions 1.x and 2.x. This setup supports different branches and configurations for stable, development, and version-specific releases of Casper Node and Client.

---

### What is this repository for?

This repository contains Docker images and Docker Compose services that simplify the process of running the **Casper network's NCTL** (Node Control Tool). NCTL is a tool used for managing **Casper testnets**. The images are configured to work with different branches and versions, including stable, development, and version-specific releases.

---

### Key Features:

- **Stable Version:**
  Uses **Casper Node v1.5.8** and **Casper Client v2.0.0** for a stable environment.

- **2.0 Version:**
  Uses **Casper Node v2.0.4**, **Client v5.0.0**, and **Sidecar v2.0.0**.

- **2.1 Version:**
  Uses **Casper Node v2.1.2**, **Client v5.0.0**, and **Sidecar v2.0.0**.

- **2.2 Version:**
  Uses **Casper Node v2.2.2**, **Client v5.0.1**, and **Sidecar v2.1.0**.

- **Development Version:**
  Tracks the latest development versions of **Casper Node, Client**, and **Sidecar**.

---

## Usage

You can choose between `stable`, `dev`, `2.0`, `2.1`, or `2.2`.
If no profile is specified, **`stable` is used by default**.

### Build the container

```sh
make build 2.2
```

### Start the container & Log

```sh
make start-log 2.2
```

### Build & Start & Log in one step

```sh
make build-start-log 2.2
```

To replace `2.2` with another version, use `stable`, `dev`, `2.0`, or `2.1`.

---

## Available Tags & Profiles

Casper NCTL 2 Docker supports five configurations:

### Stable (default)

- Uses **v1.5.8** for the node and **v2.0.0** for the client.

```yaml
- BRANCH_NODE=v1.5.8
- BRANCH_CLIENT=v2.0.0
```

### 2.0 (Specific Release)

- Uses **v2.0.4** for the node, **v5.0.0** for the client, and **v2.0.0** for the sidecar.

```yaml
- BRANCH_NODE=v2.0.4
- BRANCH_CLIENT=v5.0.0
- BRANCH_SIDECAR=v2.0.0
```

### 2.1 (Specific Release)

- Uses **v2.1.2** for the node, **v5.0.0** for the client, and **v2.0.0** for the sidecar.

```yaml
- BRANCH_NODE=v2.1.2
- BRANCH_CLIENT=v5.0.0
- BRANCH_SIDECAR=v2.0.0
```

### 2.2 (Specific Release)

- Uses **v2.2.2** for the node, **v5.0.1** for the client, and **v2.1.0** for the sidecar.

```yaml
- BRANCH_NODE=v2.2.2
- BRANCH_CLIENT=v5.0.1
- BRANCH_SIDECAR=v2.1.0
```

### Dev (Latest Development Builds)

- Tracks the latest development versions of **node, client, and sidecar**.

```yaml
- BRANCH_NODE=dev
- BRANCH_CLIENT=dev
- BRANCH_SIDECAR=dev
```

---

## Services Overview

This project provides multiple Docker Compose services:

### casper-nctl-2-docker (Base Service)

- Provides the core infrastructure for running a Casper NCTL node.
- Used by `stable`, `2.0`, `2.1`, `2.2`, and `dev` profiles.
- **Exposes Ports:**

> 11101-11105, 14101-14105, 18101-18105, 25101-25105, 28101-28105

- **Maps Volumes for Persistence:**
- Logs
- Faucet
- Users
- Chainspec
- Node Data

### CORS Proxy (Optional)

- Runs a CORS proxy on **port 11100**.
- Required for front-end applications making cross-origin requests.
- To enable, use the `cors-anywhere` profile.

---

## Running the Containers

### Building a Specific Version

```sh
make build 2.2
```

or build without cache:

```sh
make build-no-cache 2.2
```

### Starting a Specific Version

```sh
make start dev
```

### Building & Starting a Specific Version

```sh
make build-start dev
```

### View Logs

```sh
make start-log dev
```

### Stopping the Container

```sh
make stop dev
```

---

## Docker-Compose Services

### casper-nctl-2-docker

This is the base service for running a Casper NCTL instance.

- **Image:** `casper-nctl-2-docker:latest`
- **Ports:**
  - 11101-11105, 14101-14105, 18101-18105, 25101-25105, 28101-28105
- **Volumes:**
  - Logs, faucet, users, chainspec, and node data for persistence

### casper-nctl-2-docker-stable

Provides the stable environment for Casper Node and Client.

- **Image:** `casper-nctl-2-docker-stable:latest`
- **Build Args:**
  - `BRANCH_NODE=v1.5.8`
  - `BRANCH_CLIENT=v2.0.0`
- **Profiles:**
  - `stable`

### casper-nctl-2-docker-2.0

Provides the 2.0 environment for Casper Node and Client.

- **Image:** `casper-nctl-2-docker-2.0:latest`
- **Build Args:**
  - `BRANCH_NODE=v2.0.4`
  - `BRANCH_CLIENT=v5.0.0`
  - `BRANCH_SIDECAR=v2.0.0`
- **Profiles:**
  - `2.0`

### casper-nctl-2-docker-2.1

Provides the 2.1 environment for Casper Node and Client.

- **Image:** `casper-nctl-2-docker-2.1:latest`
- **Build Args:**
  - `BRANCH_NODE=v2.1.2`
  - `BRANCH_CLIENT=v5.0.0`
  - `BRANCH_SIDECAR=v2.0.0`
- **Profiles:**
  - `2.1`

### casper-nctl-2-docker-2.2

Provides the 2.2 environment for Casper Node and Client.

- **Image:** `casper-nctl-2-docker-2.2:latest`
- **Build Args:**
  - `BRANCH_NODE=v2.2.2`
  - `BRANCH_CLIENT=v5.0.1`
  - `BRANCH_SIDECAR=v2.1.0`
- **Profiles:**
  - `2.2`

### casper-nctl-2-docker-dev

Tracks the latest development versions of the node, client, and sidecar.

- **Image:** `casper-nctl-2-docker-dev:latest`
- **Build Args:**
  - `BRANCH_NODE=dev`
  - `BRANCH_CLIENT=dev`
  - `BRANCH_SIDECAR=dev`
- **Profiles:**
  - `dev`

### cors-anywhere

Runs a CORS proxy on **port 11100**.

- **Image:** Custom build using `cors-anywhere.Dockerfile`
- **Environment Variables:**
  - `PORT=11100`
- **Ports:**
  - `11100:11100`

If you want to spin up everything at once (nodes 2.2 + CORS proxy):

```sh
docker compose --profile 2.2 up -d
docker compose --profile cors-anywhere up -d
```
