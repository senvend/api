#!/bin/bash

set -euo pipefail

uv sync --all-groups
uv run ../../scripts/bufw generate

# handwritten helpers shared between the sync and async packages
cp shared/*.py packages/senvend-api/src/senvend_api/
cp shared/*.py packages/senvend-api-async/src/senvend_api_async/
