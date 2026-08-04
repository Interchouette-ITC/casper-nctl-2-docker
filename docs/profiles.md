# Profiles and tags

Compose **profiles** match image **tags** for published builds.

| Profile          | Node   | Client | Sidecar | Published tag             |
| ---------------- | ------ | ------ | ------- | ------------------------- |
| `1.5.8` / `1.6`  | v1.5.8 | v2.0.0 | —       | `1.5.8`, `1.6`            |
| `stable` / `2.2` | v2.2.2 | v5.0.1 | v2.1.0  | `stable`, `2.2`, `latest` |
| `2.0`            | v2.0.4 | v5.0.0 | v2.0.0  | `2.0`                     |
| `2.1`            | v2.1.2 | v5.0.0 | v2.0.0  | `2.1`                     |
| `dev`            | dev    | dev    | dev     | `dev`                     |

```sh
make start stable
make start 2.2
make start-all dev    # NCTL + MCP
```

Separate compose profiles:

- `cors-anywhere` — browser CORS proxy on `11100`
- `mcp` — Rust MCP HTTP sidecar on `8788` (via `make mcp-http` / `make start-all`)
