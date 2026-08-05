# Ports and RPC

| Port / range | Role |
| --- | --- |
| `11101-11105` | JSON-RPC (nodes 1–5) |
| `14101-14105` | REST |
| `18101-18105` | SSE |
| `25101-25105` | Sidecar RPC (2.x) |
| `28101-28105` | Additional |
| `11100` | CORS proxy (`cors-anywhere` profile) |
| `8791` | MCP Streamable HTTP (`/mcp`) |

Typical node-1 RPC:

```bash
curl -s -X POST http://127.0.0.1:11101/rpc \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"info_get_status","params":[]}'
```

MCP (after `make start-all` or `make mcp-http`):

`http://127.0.0.1:8791/mcp`
