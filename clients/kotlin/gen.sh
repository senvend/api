#!/bin/bash

set -euo pipefail

# run from this directory so the relative paths below are stable
cd "$(dirname "$0")"

# buf's protoc_builtin plugins (java, kotlin) shell out to the protoc on PATH;
# use the pinned one so the generated code is reproducible across machines
. ../../scripts/download-protoc.sh

# grpc-java and grpc-kotlin are used as buf "local" plugins, see buf.gen.yaml
./gradlew downloadGrpcPlugins

../../scripts/bufw generate
./gradlew --refresh-dependencies
