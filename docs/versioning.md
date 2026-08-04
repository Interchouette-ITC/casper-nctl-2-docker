# Versioning

| What         | Tags                                                         | Registry                                 |
| ------------ | ------------------------------------------------------------ | ---------------------------------------- |
| NCTL testnet | `stable`, `2.2`, `latest` (same image); also `2.1`, `dev`, … | `interchouette/casper-nctl-2-docker`     |
| MCP sidecar  | `2.2` + `latest` (release); `dev` (tip)                      | `interchouette/casper-nctl-2-docker-mcp` |

NCTL profile tags follow Casper node/client/sidecar pins (see [Profiles](profiles.md)). MCP **image** tags (`:2.2`, `:latest`, `:dev`) are the publish line; the MCP **crate** version in `mcp/Cargo.toml` is independent semver (currently `0.2.2`).

## CI rebuild policy

Do **not** rebuild every historical family on each merge to `dev`. Floating and active family tags move; legacy families stay on Hub until you refresh them on purpose.

| Trigger                                      | Rebuilds                                                         |
| -------------------------------------------- | ---------------------------------------------------------------- |
| Push to `dev` (non-docs paths)               | `:dev`, `:2.2` (+ `:stable` + `:latest`)                         |
| Daily cron                                   | `:dev` only                                                      |
| Actions → Run workflow (`workflow_dispatch`) | Parked `:2.1`, `:2.0`, `:1.5.8` / `:1.6`                         |

Legacy Hub tags are not removed; they simply are not rebuilt automatically.

### Adding a future family (e.g. `2.3`)

1. Add a compose profile and `make build 2.3` pins (see [Profiles](profiles.md)).
2. Copy `.github/workflows/docker-build-push-2.2.yml` → `docker-build-push-2.3.yml`, point tags at `:2.3`, and move the `stable` / `latest` multi-tags onto that workflow.
3. Keep older active lines on `push:dev` only while you still want them refreshed every merge; otherwise switch them to `workflow_dispatch` (same pattern as `:2.0` / `:1.5.8`).
4. Do **not** add a matrix that rebuilds all previous version tags on every push.

When changing MCP tools or transport:

1. Bump `mcp/Cargo.toml` and `#[mcp_server(version = …)]`
2. Align `MCP_TAG` / compose image tag in the Makefile and `docker-compose.yml`
3. Push Hub/GHCR tags via the MCP image workflows
4. Update root `CHANGELOG.md`

See [MCP for agents](mcp.md).
