# Assets and logs

Host directory `./assets` is bind-mounted into the NCTL container:

| Path | Contents |
| --- | --- |
| `assets/faucet/` | Faucet keys (`public_key_hex`, `public_key.pem`, `secret_key.pem`) |
| `assets/users/` | `user-1` … plus accounts/chainspec helpers |
| `assets/chainspec/` | Chainspec-related files |
| `assets/nodes/node-N/` | `keys`, `logs`, `config`, `storage`, `bin` |
| `assets/logs/` | Aggregated `stdout.log`, `sidecar-stdout.log` |

Files may be **root-owned** after the container runs. Empty node log dirs before the first successful start are normal.

## MCP helpers

- `nctl_assets_summary`, `nctl_faucet_info`, `nctl_list_users`, `nctl_list_nodes`
- `nctl_read_public_key` (`faucet` / `user-N` / `node-N`) — **never** returns `secret_key.pem`
- `nctl_logs_list`, `nctl_logs`, `nctl_logs_grep`

Faucet **CSPR transfers** are not exposed via MCP in v1 (use NCTL / `casper-client` manually).
