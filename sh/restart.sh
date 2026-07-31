#!/bin/bash
# NCTL activate uses unset optionals; nctl-* are aliases. Do not use set -e/u here:
# bind-mounted assets make teardown `rm` fail ("Device or resource busy"), and set -e
# aborts setup mid-source (pop_var_context).
set -m

if [ -n "${1:-}" ] && [ -f "$1/activate" ]; then
    # shellcheck source=/dev/null
    source "$1/activate"
elif [ -f "/app/casper-nctl/activate" ]; then
    # shellcheck source=/dev/null
    source /app/casper-nctl/activate
else
    echo "Error: Activation script not found"
    exit 1
fi

shopt -s expand_aliases

sed -i 's/^allow_request_speculative_exec = false/allow_request_speculative_exec = true/' ./casper-node/resources/local/config.toml
sed -i 's/262_144/450_000/g' ./casper-node/resources/local/chainspec.toml.in
sed -i 's/^qps_limit = 110/qps_limit = 220/' ./casper-node/resources/local/config.toml
sed -i 's/^qps_limit = 1$/qps_limit = 100/' ./casper-sidecar/resources/example_configs/default_rpc_only_config.toml

nctl-assets-setup
find "${NCTL}/assets/" -name 'secret_key.pem' -exec chmod go+r {} \;
nctl-start

exec tail -f "${NCTL}/assets/net-1/nodes/node-1/logs/stderr.log"
