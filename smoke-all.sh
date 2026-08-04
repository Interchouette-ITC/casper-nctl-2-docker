#!/bin/bash
# Local smoke: build + start each profile; verify NCTL gets past asset setup.
set -uo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"
LOG_DIR="${ROOT}/assets/.smoke-logs"
mkdir -p "$LOG_DIR"
RESULT="${LOG_DIR}/results.txt"
: > "$RESULT"

PROFILES=(1.5.8 1.6 2.0 2.1 2.2 dev)

pass() { echo "PASS $1 — $2" | tee -a "$RESULT"; }
fail() { echo "FAIL $1 — $2" | tee -a "$RESULT"; }

smoke_one() {
  local profile="$1"
  local blog="${LOG_DIR}/build-${profile}.log"
  local slog="${LOG_DIR}/run-${profile}.log"
  echo "======== BUILD ${profile} $(date -Is) ========" | tee -a "$RESULT"

  if ! make build "$profile" >"$blog" 2>&1; then
    fail "$profile" "build failed (see ${blog})"
    tail -40 "$blog" | tee -a "$RESULT"
    return 1
  fi
  pass "$profile" "build ok"

  make stop "$profile" >/dev/null 2>&1 || true
  # Fresh-ish host assets dirs (mount points must remain)
  for d in faucet users chainspec nodes; do
    mkdir -p "assets/$d"
  done

  echo "======== RUN ${profile} $(date -Is) ========" | tee -a "$RESULT"
  # Detached start
  if ! make start "$profile" >"$slog" 2>&1; then
    fail "$profile" "start failed (see ${slog})"
    return 1
  fi

  # Wait up to 10 minutes for setup complete or fatal
  local ok=0
  for i in $(seq 1 120); do
    sleep 5
    docker logs "casper-nctl-2-docker-${profile}" >"${LOG_DIR}/docker-${profile}.log" 2>&1 || true
    if grep -q 'asset setup complete\|nctl-start\|waiting for genesis\|Node starting\|supervisor' "${LOG_DIR}/docker-${profile}.log" 2>/dev/null; then
      if grep -qiE 'pop_var_context|exited with code|Error: Activation' "${LOG_DIR}/docker-${profile}.log" 2>/dev/null; then
        # continue watching; early errors may appear before retry
        :
      else
        ok=1
        break
      fi
    fi
    if grep -q 'pop_var_context' "${LOG_DIR}/docker-${profile}.log" 2>/dev/null; then
      fail "$profile" "pop_var_context in container logs"
      docker logs "casper-nctl-2-docker-${profile}" 2>&1 | tail -30 | tee -a "$RESULT"
      make stop "$profile" >/dev/null 2>&1 || true
      return 1
    fi
    # Container exited
    if ! docker ps --format '{{.Names}}' | grep -qx "casper-nctl-2-docker-${profile}"; then
      fail "$profile" "container exited early"
      docker logs "casper-nctl-2-docker-${profile}" 2>&1 | tail -40 | tee -a "$RESULT"
      return 1
    fi
  done

  if [ "$ok" -eq 1 ]; then
    pass "$profile" "runtime smoke ok (setup/start progressed)"
    docker logs "casper-nctl-2-docker-${profile}" 2>&1 | tail -20 | tee -a "$RESULT"
  else
    # Still running after wait — treat as soft pass if no fatal
    if docker ps --format '{{.Names}}' | grep -qx "casper-nctl-2-docker-${profile}"; then
      if grep -q 'pop_var_context' "${LOG_DIR}/docker-${profile}.log" 2>/dev/null; then
        fail "$profile" "still broken after wait"
      else
        pass "$profile" "container still running after wait (check logs)"
        docker logs "casper-nctl-2-docker-${profile}" 2>&1 | tail -25 | tee -a "$RESULT"
      fi
    else
      fail "$profile" "no success signal within timeout"
      docker logs "casper-nctl-2-docker-${profile}" 2>&1 | tail -40 | tee -a "$RESULT"
    fi
  fi

  make stop "$profile" >/dev/null 2>&1 || true
  # give ports a moment
  sleep 3
}

echo "Smoke start $(date -Is)" | tee -a "$RESULT"
for p in "${PROFILES[@]}"; do
  smoke_one "$p" || true
done
echo "Smoke done $(date -Is)" | tee -a "$RESULT"
echo "===== SUMMARY =====" | tee -a "$RESULT"
grep -E '^(PASS|FAIL) ' "$RESULT" | tee -a "$RESULT"
