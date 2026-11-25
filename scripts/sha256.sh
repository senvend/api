# shellcheck shell=sh
# Sourced by bufw and download-protoc.sh: `file_sha256 FILE` prints the hex SHA-256 of FILE.
file_sha256() {
  if command -v sha256sum >/dev/null; then
    set -- "$(sha256sum "$1")"
  else
    set -- "$(shasum -a 256 "$1")"
  fi
  echo "${1%% *}"
}
