#!/bin/bash

set -euo pipefail

uv sync --all-groups
buf generate
