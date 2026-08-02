# Versioning

| What | Tags | Registry |
| --- | --- | --- |
| NCTL testnet | `stable`, `2.2`, `latest` (same image); also `2.1`, `dev`, … | `interchouette/casper-nctl-2-docker` |
| MCP sidecar | `2.2` + `latest` (release); `dev` (tip) | `interchouette/casper-nctl-2-docker-mcp` |

NCTL profile tags follow Casper node/client/sidecar pins (see [Profiles](profiles.md)). MCP **image** tags (`:2.2`, `:latest`, `:dev`) are the publish line; the MCP **crate** version in `mcp/Cargo.toml` is independent semver (currently `0.2.2`).

When changing MCP tools or transport:

1. Bump `mcp/Cargo.toml` and `#[mcp_server(version = …)]`
2. Align `MCP_TAG` / compose image tag in the Makefile and `docker-compose.yml`
3. Push Hub/GHCR tags via the MCP image workflows
4. Update root `CHANGELOG.md`

See [MCP for agents](mcp.md).
