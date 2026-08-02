# Changelog

All notable changes to **casper-nctl-2-docker** packaging and the **MCP sidecar** are listed here.

NCTL image profile tags (`stable`, `2.2`, …) continue to track Casper pins separately — see [docs/versioning.md](docs/versioning.md).

## [2.2.2] - 2026-08-02

### Added

- **MCP sidecar** `casper-nctl-2-docker-mcp` **v2.2.2** (Rust / mcpkit): stdio + Streamable HTTP on port **8788** (`/mcp`).
- Make targets: `start-all`, `stop-all`, `mcp-http`, `mcp-http-stop`, `run-mcp`, `run-mcp-http` (plain `make start` stays NCTL-only).
- MCP tools for compose/Hub lifecycle parity, faucet/assets inspection, and log tail/grep (secrets never returned).
- Docs site (`docs/`, MkDocs) including MCP and versioning guides; GitHub Pages workflow.
- Product image rename to `interchouette/casper-nctl-2-docker` (legacy `casper-nctl` documented as deprecated).
