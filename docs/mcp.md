# MCP for agents

Rust binary `casper-nctl-2-docker-mcp` **v2.2.2** (mcpkit), tvscreener-style dual transport.

**Version:** MCP sidecar semver is **`2.2.2`** (smiles at node pin `v2.2.2` / profile `2.2`; not locked to it). See [Versioning](versioning.md). Changelog: repository root `CHANGELOG.md`.

| Mode | How | Cursor |
| --- | --- | --- |
| **HTTP** (Docker) | `make start-all <profile>` or `make mcp-http` | `"url": "http://127.0.0.1:8788/mcp"` |
| **stdio** (host) | `make run-mcp` / `casper-nctl-2-docker-mcp` | command spawn (see example) |

Plain `make start <profile>` remains **NCTL only**. Use `start-all` when you want MCP too.

## Make matrix

| Command | Effect |
| --- | --- |
| `make start 2.2` | NCTL only |
| `make stop 2.2` | Stop NCTL profile |
| `make start-all 2.2` | NCTL + MCP on **8788** |
| `make stop-all 2.2` | Stop NCTL + MCP |
| `make mcp-http` | MCP sidecar only |
| `make mcp-http-stop` | Stop MCP sidecar |
| `make run-mcp` | Host **stdio** MCP |
| `make run-mcp-http` | Host HTTP on `127.0.0.1:8788` |

```bash
casper-nctl-2-docker-mcp                         # stdio
casper-nctl-2-docker-mcp --http                  # 0.0.0.0:8788
casper-nctl-2-docker-mcp --http --listen 127.0.0.1:8788
```

Example Cursor config: copy snippets from `mcp/mcp.json.example` in the repository root.

## Tool catalog

### Lifecycle (Make parity)

MCP drives **Docker** via Make/compose (or Hub `docker run`). It does not run NCTL as a host binary.

| Tool | Make equivalent | Notes |
| --- | --- | --- |
| `nctl_build` | `make build` | Compose image build |
| `nctl_build_no_cache` | `make build-no-cache` | Fresh build |
| `nctl_start` | `make start` | Compose `up -d` |
| `nctl_start_log` | `make start-log` | Detached + log tail (no TTY hang) |
| `nctl_build_start` | `make build-start` | Build then `up -d` |
| `nctl_build_start_log` | `make build-start-log` | No-cache build + start + log tail |
| `nctl_stop` | `make stop` | Compose down |
| `nctl_start_all` | `make start-all` | NCTL + MCP `:8788` |
| `nctl_stop_all` | `make stop-all` | |
| `nctl_start_docker` | `make start-docker` | Hub image `docker run` **detached** (Make uses `-it`) |
| `nctl_stop_docker` | — | Remove Hub-run container |
| `nctl_status` | — | Compose + hub-run + MCP + RPC |
| `nctl_endpoints` | — | URLs |
| `nctl_cors_start` | cors profile | Port 11100 |
| `nctl_list_profiles` | — | Profiles + parity map |

**Compose vs Hub:** `nctl_start*` = compose project (clone workflow). `nctl_start_docker` = published image without compose.

### Assets (faucet first-class)

| Tool | Behavior |
| --- | --- |
| `nctl_assets_summary` | faucet/users/nodes/chainspec/logs presence |
| `nctl_faucet_info` | Faucet `public_key_hex` only (no secrets; no CSPR transfer in v1) |
| `nctl_list_nodes` | `node-N` dirs and keys/logs/storage/config |
| `nctl_list_users` | `user-N` (+ optional public hex) |
| `nctl_read_public_key` | `faucet` \| `user-N` \| `node-N` |
| `nctl_read_chainspec` | List/read size-capped text under `assets/` (refuses secrets) |

### Logs

| Tool | Behavior |
| --- | --- |
| `nctl_logs_list` | Files under `assets/logs` and `nodes/*/logs` |
| `nctl_logs` | Tail: `docker` \| `assets_stdout` \| `sidecar` \| `node` |
| `nctl_logs_grep` | Case-insensitive grep (capped matches) |

**Safety:** never returns `secret_key.pem`; paths confined under `assets/`; log payloads capped.

MCP is a **slim Rust sidecar image**, not baked into the heavy NCTL image.

## Enable GitHub Pages

After `.github/workflows/docs-pages.yml` is on `dev`, set the repo Pages source to **GitHub Actions** once under Settings → Pages.
