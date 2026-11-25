# shellcheck shell=bash
# Sourced by gen.sh and public/gen.sh, under pipefail so a failing stage isn't masked by its
# prefix pipe: `stage LABEL DIR COMMAND...` runs COMMAND in DIR in the background, each line it
# prints prefixed with [LABEL]; `wait_stages` waits for them all and fails if any did.

stage_pids=()
stage_names=()

function stage() {
  local label="$1" dir="$2"
  shift 2
  (cd "${dir}" && "$@") 2>&1 | sed -u "s/^/$(printf '%-8s ' "[${label}]")/" &
  stage_pids+=("$!")
  stage_names+=("${dir}/${1#./}")
}

function wait_stages() {
  local fail=0 i
  for i in "${!stage_pids[@]}"; do
    wait "${stage_pids[$i]}" || { echo "${stage_names[$i]} failed" >&2; fail=1; }
  done
  return "${fail}"
}
