# MCP examples (developers / CI)

These binaries call the **same Rust helpers** the MCP server uses (`ops`, `assets`, `logs`). They are for **local smoke and CI**, not a substitute for Cursor MCP.

**Cursor agents** must use `CallMcpTool` on the `casper-nctl-2-docker` MCP server — see `.cursor/rules/nctl-use-mcp.mdc`.

```bash
export NCTL_DOCKER_ROOT=/path/to/casper-nctl-2-docker

cargo run --manifest-path mcp/Cargo.toml --example list_profiles
cargo run --manifest-path mcp/Cargo.toml --example endpoints -- 2.2
cargo run --manifest-path mcp/Cargo.toml --example faucet_info
cargo run --manifest-path mcp/Cargo.toml --example status -- 2.2
cargo run --manifest-path mcp/Cargo.toml --example start_log -- 2.2 40
cargo run --manifest-path mcp/Cargo.toml --example start_faucet -- 2.2
cargo run --manifest-path mcp/Cargo.toml --example stop -- 2.2
cargo run --manifest-path mcp/Cargo.toml --example roundtrip -- 2.2
```

`start_*` / `stop` / `roundtrip` need Docker and a tagged local image (e.g. `casper-nctl-2-docker-2.2:latest`).
