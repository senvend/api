#!/bin/bash

set -euo pipefail

./install-protoc-plugins.sh
../../scripts/bufw generate
cargo fmt
