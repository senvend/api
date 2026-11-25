#!/bin/bash

set -euo pipefail

# run from this directory so the relative paths below are stable
cd "$(dirname "$0")"

./install-protoc-plugins.sh
../../scripts/bufw generate
cargo fmt
