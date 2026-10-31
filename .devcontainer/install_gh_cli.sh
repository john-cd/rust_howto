#!/usr/bin/env bash
# Install a checksum-pinned GitHub CLI release for Linux.

set -euo pipefail

: "${GH_CLI_VERSION:?Set GH_CLI_VERSION from .devcontainer/.env}"
: "${GH_CLI_LINUX_AMD64_SHA256:?Set the GitHub CLI x64 checksum from .devcontainer/.env}"
: "${GH_CLI_LINUX_ARM64_SHA256:?Set the GitHub CLI ARM64 checksum from .devcontainer/.env}"

if [[ ! "$GH_CLI_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    printf 'GitHub CLI version must be a stable x.y.z release: %s\n' "$GH_CLI_VERSION" >&2
    exit 2
fi

case "$(uname -m)" in
    amd64|x86_64)
        archive_arch=amd64
        checksum="$GH_CLI_LINUX_AMD64_SHA256"
        ;;
    arm64|aarch64)
        archive_arch=arm64
        checksum="$GH_CLI_LINUX_ARM64_SHA256"
        ;;
    *)
        printf 'Unsupported GitHub CLI architecture: %s\n' "$(uname -m)" >&2
        exit 2
        ;;
esac

install_root="${1:-/usr/local}"
archive_name="gh_${GH_CLI_VERSION}_linux_${archive_arch}.tar.gz"
archive="$(mktemp)"
stage_directory="$(mktemp -d)"
trap 'rm -f -- "$archive"; rm -rf -- "$stage_directory"' EXIT

curl --fail --location --proto '=https' --tlsv1.2 --retry 3 \
    "https://github.com/cli/cli/releases/download/v${GH_CLI_VERSION}/${archive_name}" \
    --output "$archive"
printf '%s  %s\n' "$checksum" "$archive" | sha256sum --check --status

tar -xzf "$archive" -C "$stage_directory"
install -D -m 0755 \
    "$stage_directory/gh_${GH_CLI_VERSION}_linux_${archive_arch}/bin/gh" \
    "$install_root/bin/gh"

"$install_root/bin/gh" --version
