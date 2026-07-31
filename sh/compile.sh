#!/bin/bash
# No -u: NCTL activate references unset vars (e.g. NCTL_CASPER_HOME).
set -eo pipefail

if [ -n "${1:-}" ]; then
    # shellcheck source=/dev/null
    source "$1/activate"
else
    # shellcheck source=/dev/null
    source /app/casper-nctl/activate
fi

# NCTL registers nctl-* as aliases; enable them in non-interactive shells.
shopt -s expand_aliases

nctl-compile
echo "nctl-compile complete."
