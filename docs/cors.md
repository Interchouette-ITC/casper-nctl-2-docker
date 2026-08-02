# CORS proxy

Browser apps that call NCTL RPC from another origin can hit the optional `cors-anywhere` service on port **11100**.

```sh
docker compose --profile 2.2 up -d
docker compose --profile cors-anywhere up -d
```

Or ask the MCP tool `nctl_cors_start` once the MCP server is running.
