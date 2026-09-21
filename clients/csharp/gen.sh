#!/bin/bash

set -euo pipefail

# run from this directory so the relative paths below are stable
cd "$(dirname "$0")"

# buf's protoc_builtin plugins (csharp) shell out to the protoc on PATH;
# use the pinned one so the generated code is reproducible across machines
. ../../scripts/download-protoc.sh

# grpc_csharp_plugin is used as a buf "local" plugin, see buf.gen.yaml
. ../../scripts/download-grpc-csharp-plugin.sh

../../scripts/bufw generate
