#!/bin/bash

set -euo pipefail

# grpc_csharp_plugin wrapper: downloads the pinned plugin from the Grpc.Tools nuget package into
# the user cache (like bufw) and puts it on PATH, so buf can run it as a local plugin.

# only allow sourcing this script
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  echo "This script should be sourced, not executed"
  exit 1
fi

# renovate: datasource=nuget depName=Grpc.Tools
GRPC_TOOLS_VERSION=2.83.0
echo "GRPC_TOOLS_VERSION=${GRPC_TOOLS_VERSION}"

PLUGIN_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/senvend-grpc-csharp/${GRPC_TOOLS_VERSION}"
PLUGIN="${PLUGIN_DIR}/grpc_csharp_plugin"

# export plugin path so buf finds it as a local plugin
export PATH="${PLUGIN_DIR}:${PATH}"

function download_grpc_csharp_plugin() {
    # get architecture
    ARCH=$(uname -m)

    # only x86_64 and aarch64 are supported atm
    case "${ARCH}" in
      x86_64) NUGET_ARCH="linux_x64" ;;
      aarch64) NUGET_ARCH="linux_arm64" ;;
      *) echo "Unsupported architecture: ${ARCH}"; exit 1 ;;
    esac

    echo "ARCH=${ARCH}"

    # generate download link
    DOWNLOAD_URL="https://api.nuget.org/v3-flatcontainer/grpc.tools/${GRPC_TOOLS_VERSION}/grpc.tools.${GRPC_TOOLS_VERSION}.nupkg"
    echo "DOWNLOAD_URL=${DOWNLOAD_URL}"

    NUPKG_FILE="${PLUGIN_DIR}/grpc.tools.nupkg"

    # download the nuget package to the cache directory
    mkdir -p "${PLUGIN_DIR}"
    curl -L -o "${NUPKG_FILE}" "${DOWNLOAD_URL}"

    # a nupkg is a zip - extract just the plugin, without its directory structure
    unzip -j -o "${NUPKG_FILE}" "tools/${NUGET_ARCH}/grpc_csharp_plugin" -d "${PLUGIN_DIR}"
    chmod +x "${PLUGIN}"
    rm "${NUPKG_FILE}"

    echo "grpc_csharp_plugin ${GRPC_TOOLS_VERSION} was successfully installed"
}

# check if the pinned plugin is already installed
if [ -x "${PLUGIN}" ]; then
  echo "grpc_csharp_plugin ${GRPC_TOOLS_VERSION} is already installed"
else
  download_grpc_csharp_plugin
fi
