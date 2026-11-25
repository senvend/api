#!/bin/bash

# pipefail so a failing client gen isn't masked by its sed prefix pipe
set -euo pipefail

# run from this directory so the relative paths below are stable
cd "$(dirname "$0")"

# fetch the pinned buf into the user cache first: the client gens below run in parallel and
# would otherwise all download it into the same file on a cold cache. The clients that use
# protoc (kotlin, csharp) fetch it themselves; its download is safe to run in parallel.
scripts/bufw --version >/dev/null

# the client gens are independent of each other and write disjoint trees
. scripts/stages.sh
stage rust clients/rust ./gen.sh
stage python clients/python ./gen.sh
stage kotlin clients/kotlin ./gen.sh
stage csharp clients/csharp ./gen.sh
wait_stages
