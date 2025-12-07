#!/bin/bash

set -euo pipefail

./install-protoc-plugins.sh
buf generate
cargo fmt
