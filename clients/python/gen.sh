#!/bin/bash

set -euo pipefail

# run from this directory so the relative paths below are stable
cd "$(dirname "$0")"

# A virtualenv records an absolute interpreter path in every console script it
# installs, so a checkout that is moved or renamed leaves them pointing at an
# interpreter that is no longer there. protoc reports that as a plugin which
# does not exist, which is misleading: the plugin is present, its interpreter is
# not. Syncing reconciles packages rather than paths and leaves such scripts
# alone, so rewrite them when the recorded interpreter has gone. Never empty:
# bash before 4.4 rejects an empty array's expansion under `set -u`.
sync_args=(--all-groups)
plugin=".venv/bin/protoc-gen-python_betterproto2"
if [ -f "${plugin}" ]; then
  interpreter="$(sed -n '1s/^#!//p' "${plugin}")"
  [ -x "${interpreter}" ] || sync_args+=(--reinstall)
fi

uv sync --locked "${sync_args[@]}"
# in the venv: the plugin formats its output with the venv's ruff
uv run --locked ../../scripts/bufw generate

# handwritten helpers shared between the sync and async packages
cp shared/*.py packages/senvend-api/src/senvend_api/
cp shared/*.py packages/senvend-api-async/src/senvend_api_async/
