#!/bin/bash

set -euo pipefail

# protoc wrapper: downloads the pinned protoc version into the user cache (like bufw) and
# puts it first on PATH so buf's protoc_builtin plugins produce reproducible output.

# only allow sourcing this script
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  echo "This script should be sourced, not executed"
  exit 1
fi

# renovate: datasource=github-releases depName=protocolbuffers/protobuf extractVersion=v(?<version>.*)
PROTOC_VERSION=36.0
echo "PROTOC_VERSION=${PROTOC_VERSION}"

PROTOC_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/senvend-protoc/${PROTOC_VERSION}"
PROTOC="${PROTOC_DIR}/bin/protoc"

# export protoc path to override system protoc
export PATH="${PROTOC_DIR}/bin:${PATH}"

function download_protoc() {
    # get architecture
    ARCH=$(uname -m)

    # only x86_64 and aarch64 are supported atm
    if [ "${ARCH}" != "x86_64" ] && [ "${ARCH}" != "aarch64" ]; then
      echo "Unsupported architecture: ${ARCH}"
      exit 1
    fi

    # replace aarch64 with aarch_64
    if [ "${ARCH}" == "aarch64" ]; then
      ARCH="aarch_64"
    fi

    echo "ARCH=${ARCH}"

    # generate download link
    DOWNLOAD_URL="https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-${ARCH}.zip"
    echo "DOWNLOAD_URL=${DOWNLOAD_URL}"

    ZIP_FILE="${PROTOC_DIR}/protoc.zip"

    # download protoc to the cache directory
    mkdir -p "${PROTOC_DIR}"
    # wget -O ${ZIP_FILE} "${DOWNLOAD_URL}"
    curl -L -o "${ZIP_FILE}" "${DOWNLOAD_URL}"

    # unzip protoc
    unzip -o "${ZIP_FILE}" -d "${PROTOC_DIR}"
    rm "${ZIP_FILE}"

    echo "protoc ${PROTOC_VERSION} was successfully installed"
}

# check if correct protoc version is already installed
if [ -f "${PROTOC}" ]; then
  INSTALLED_VERSION=$(${PROTOC} --version | cut -d' ' -f2)
  if [ "${INSTALLED_VERSION}" == "${PROTOC_VERSION}" ]; then
    echo "protoc ${PROTOC_VERSION} is already installed"
  else
    echo "wrong protoc version installed: ${INSTALLED_VERSION}"
    rm -rf "${PROTOC_DIR}"
    download_protoc
  fi
else
  download_protoc
fi
