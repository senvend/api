#!/bin/bash

set -euo pipefail

(cd clients/rust && ./gen.sh)
(cd clients/python && ./gen.sh)
