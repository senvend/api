#!/bin/bash

set -euo pipefail

GIT_SHA="66b503dd1631bc2eaf79db37cde74b01567c8224"
GIT_REPO="https://github.com/neoeinstein/protoc-gen-prost.git"

install(){
  cargo install --locked --git "$GIT_REPO" --rev "$GIT_SHA" "$@"
}

install protoc-gen-prost
install protoc-gen-prost-crate
install protoc-gen-prost-serde
install protoc-gen-tonic
