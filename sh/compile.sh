#!/bin/bash
set -euo pipefail

if [ -n "${1:-}" ]; then
    # shellcheck source=/dev/null
    source "$1/activate"
else
    # shellcheck source=/dev/null
    source /app/casper-nctl/activate
fi

nctl-compile
echo "nctl-compile complete."
