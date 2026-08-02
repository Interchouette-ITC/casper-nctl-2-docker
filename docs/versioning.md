# Versioning

`casper-nctl-2-docker` uses **two independent version lines**. Do not conflate them.

| What | Version | Where it shows up |
| --- | --- | --- |
| NCTL testnet image | Profile Docker tags (`stable`, `2.2`, `dev`, …) | Docker Hub / GHCR |
| MCP sidecar | Semver (currently **`2.2.2`**) | `mcp/Cargo.toml`, MCP handshake, image `casper-nctl-2-docker-mcp:2.2.2` |

## NCTL image tags

Profile tags track **Casper node/client/sidecar pins** (e.g. profile `2.2` / `stable` builds with `BRANCH_NODE=v2.2.2`). See [Profiles](profiles.md).

There is no separate GitHub Release for the packaging repo; shipping is image tags.

## MCP sidecar (`2.2.2`)

First public MCP cut is **`2.2.2`**, smiling at the recommended node pin `v2.2.2` / profile `2.2`. That is a product cue, **not** a hard lock — MCP works with every compose profile.

Later MCP bumps are independent of Casper pins (for example MCP `2.3.0` while still driving a `2.2` network).

Bump MCP when tools or transport behavior change:

1. `mcp/Cargo.toml` `version`
2. `#[mcp_server(version = "…")]` in `mcp/src/server.rs` (must match)
3. Compose / CI image tags (`casper-nctl-2-docker-mcp:<version>` and `:latest`)
4. Root [`CHANGELOG.md`](../CHANGELOG.md)

See [MCP for agents](mcp.md).
