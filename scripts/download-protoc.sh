#!/bin/bash

set -euo pipefail

# protoc wrapper: downloads, verifies and installs the pinned protoc into the user cache (like
# bufw) and puts it first on PATH so buf's protoc_builtin plugins produce reproducible output.

# only allow sourcing this script
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  echo "This script should be sourced, not executed"
  exit 1
fi

# renovate: datasource=github-releases depName=protocolbuffers/protobuf extractVersion=v(?<version>.*)
PROTOC_VERSION=36.0
echo "PROTOC_VERSION=${PROTOC_VERSION}"

# protoc's name for the platform, and the sha256 of its release zip, from the release's asset
# digests; update them together with PROTOC_VERSION
case "$(uname -s)-$(uname -m)" in
  Darwin-arm64)
    PROTOC_PLATFORM="osx-aarch_64"
    PROTOC_SHA256="b6bc4afdcb880124bf342851d05155b6e3d9b6e661236d87b9c614250d26ae00"
    ;;
  Darwin-x86_64)
    PROTOC_PLATFORM="osx-x86_64"
    PROTOC_SHA256="2847d952ecd1c466769ae3ca319c9cd34c3613542eba335dc9b02c49537f6c70"
    ;;
  Linux-aarch64)
    PROTOC_PLATFORM="linux-aarch_64"
    PROTOC_SHA256="4a00ec5e256d20a3deadd9e77d56da0ac04c72367c3c959f6d08e110a368400a"
    ;;
  Linux-x86_64)
    PROTOC_PLATFORM="linux-x86_64"
    PROTOC_SHA256="bc8211ce760bd43ee21ddc145d6d9dbaeeabae205267a79d9054a240e367d4b4"
    ;;
  *)
    echo "download-protoc.sh: no protoc pinned for $(uname -s)-$(uname -m)" >&2
    return 1
    ;;
esac

PROTOC_CACHE="${XDG_CACHE_HOME:-$HOME/.cache}/senvend-protoc"
PROTOC_DIR="${PROTOC_CACHE}/${PROTOC_VERSION}"
PROTOC="${PROTOC_DIR}/bin/protoc"

# export protoc path to override system protoc
export PATH="${PROTOC_DIR}/bin:${PATH}"

# the version the cached protoc reports, empty when there is none or it does not run
function protoc_installed_version() {
  { "${PROTOC}" --version 2>/dev/null || true; } | cut -d' ' -f2
}

. "$(dirname "${BASH_SOURCE[0]}")/sha256.sh"

# downloads and verifies the zip in a directory of its own, then moves the unpacked protoc into
# place in one rename, so a failed or concurrent run never leaves a half-installed one behind
function download_protoc() {
  local url="https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-${PROTOC_PLATFORM}.zip"
  echo "DOWNLOAD_URL=${url}"
  mkdir -p "${PROTOC_CACHE}"
  local staging
  staging="$(mktemp -d "${PROTOC_CACHE}/.download.XXXXXX")"
  if ! curl -fsSL -o "${staging}/protoc.zip" "${url}"; then
    rm -rf "${staging}"
    echo "download-protoc.sh: downloading ${url} failed" >&2
    return 1
  fi
  if [ "$(file_sha256 "${staging}/protoc.zip")" != "${PROTOC_SHA256}" ]; then
    rm -rf "${staging}"
    echo "download-protoc.sh: checksum mismatch for protoc ${PROTOC_VERSION} ${PROTOC_PLATFORM}" >&2
    return 1
  fi
  unzip -q "${staging}/protoc.zip" -d "${staging}/protoc"
  # another run may have installed it meanwhile; else replace whatever is there
  if [ "$(protoc_installed_version)" != "${PROTOC_VERSION}" ]; then
    rm -rf "${PROTOC_DIR}"
    mv "${staging}/protoc" "${PROTOC_DIR}"
  fi
  rm -rf "${staging}"
}

# a cached protoc that is missing, does not run or reports another version is replaced
INSTALLED_VERSION="$(protoc_installed_version)"
if [ "${INSTALLED_VERSION}" == "${PROTOC_VERSION}" ]; then
  echo "protoc ${PROTOC_VERSION} is already installed"
else
  echo "installing protoc ${PROTOC_VERSION} (found: ${INSTALLED_VERSION:-none})"
  download_protoc
  INSTALLED_VERSION="$(protoc_installed_version)"
  if [ "${INSTALLED_VERSION}" != "${PROTOC_VERSION}" ]; then
    echo "download-protoc.sh: the installed protoc reports '${INSTALLED_VERSION}', not ${PROTOC_VERSION}" >&2
    return 1
  fi
  echo "protoc ${PROTOC_VERSION} was successfully installed"
fi
