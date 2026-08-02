# Versioning

| What | Version | Where |
| --- | --- | --- |
| NCTL testnet image | Profile tags (`stable`, `2.2`, `dev`, …) | Docker Hub / GHCR |
| MCP sidecar | Semver in `mcp/Cargo.toml` (currently `2.2.2`) | MCP handshake + image `casper-nctl-2-docker-mcp:<version>` |

Profile tags follow Casper node/client/sidecar pins (see [Profiles](profiles.md)). Shipping is image tags, not GitHub Releases.

When changing MCP tools or transport:

1. Bump `mcp/Cargo.toml` `version`
2. Match `#[mcp_server(version = "…")]` in `mcp/src/server.rs`
3. Tag the MCP image (`casper-nctl-2-docker-mcp:<version>` and `:latest`)
4. Update root `CHANGELOG.md`

See [MCP for agents](mcp.md).
