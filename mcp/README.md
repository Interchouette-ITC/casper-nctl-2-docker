# MCP server for casper-nctl-2-docker

Rust **rmcp** server (`casper-nctl-2-docker-mcp`) to control and debug a local NCTL testnet from Cursor.

## Transports

| Mode | Command | Use |
| --- | --- | --- |
| **stdio** | `make run-mcp` / `casper-nctl-2-docker-mcp` | Local Cursor spawn |
| **HTTP** | `make mcp-http` or `make start-all <profile>` | Streamable HTTP on **8790** → `http://127.0.0.1:8790/mcp` |

Existing `make start <profile>` stays **NCTL only**. Use `make start-all <profile>` for NCTL + MCP.

See [docs/mcp.md](../docs/mcp.md) and [mcp.json.example](mcp.json.example).

## Tests & examples

```bash
cd mcp
cargo test --all-targets
cargo build --examples
```

Examples call the same helpers as the MCP tools (`ops` / `assets` / `logs`) for humans and CI. See [examples/README.md](examples/README.md).
