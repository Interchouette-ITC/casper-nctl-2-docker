# Changelog

All notable changes to **casper-nctl-2-docker** packaging and the **MCP sidecar** are listed here.

Image tags (`stable`, `2.2`, `dev`, …) are shared by the NCTL and MCP images — see [docs/versioning.md](docs/versioning.md).

## Unreleased

### Changed

- Profile / Hub tag `dev`: pin node/client/sidecar to `v2.2.2` / `v5.0.1` / `v2.1.0` (same as `2.2`) until casper-node branch `dev` moves past package version `2.2.0`.

## [2.2] - 2026-08-02

### Added

- MCP sidecar image `interchouette/casper-nctl-2-docker-mcp` (`:2.2`, `:latest`, `:dev`): stdio + Streamable HTTP on port **8790** (`/mcp`).
- Make targets: `start-all`, `stop-all`, `mcp-http`, `mcp-http-stop`, `run-mcp`, `run-mcp-http`, MCP image build/push helpers.
- MCP tools for compose/Hub lifecycle, faucet/assets inspection, and log tail/grep (secrets never returned).
- Docs (`docs/`, MkDocs) and Hub rename to `casper-nctl-2-docker`.
