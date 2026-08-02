# Troubleshooting

**First build is very slow** — The NCTL image compiles Casper node/client/sidecar. Prefer `docker pull interchouette/casper-nctl-2-docker:stable` or a published tag when you only need to run.

**Device or resource busy on stop** — Bind-mounted `assets/` can make teardown `rm` noisy inside NCTL; usually harmless. Stop with `make stop <profile>` and retry.

**RPC not ready yet** — Nodes need time after `nctl-start`. Retry `info_get_status` on `11101`.

**MCP not reachable on 8788** — Plain `make start` does not start MCP. Use `make start-all <profile>` or `make mcp-http`. Check `docker ps` for `casper-nctl-2-docker-mcp`.

**Permission denied on assets** — Container may write root-owned files under `./assets`. Use `sudo` to inspect or fix ownership locally if needed.

**Logs empty** — Before the first successful network setup, `assets/nodes/node-*/logs` may be empty; `assets/logs/stdout.log` appears after runs.

**Reset assets** — Not exposed via MCP. Stop the network, then clean `./assets` manually if you need a fresh NCTL setup.
