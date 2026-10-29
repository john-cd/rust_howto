#!/usr/bin/env bash
# Install a checksum-pinned Oracle Instant Client Basic Light and SDK pair.

set -euo pipefail

: "${ORACLE_CLIENT_VERSION:?Set ORACLE_CLIENT_VERSION from .devcontainer/.env}"
: "${ORACLE_CLIENT_DOWNLOAD_ID:?Set ORACLE_CLIENT_DOWNLOAD_ID from .devcontainer/.env}"

install_root="${1:-/opt/oracle}"
IFS=. read -r client_major client_minor _ <<< "$ORACLE_CLIENT_VERSION"
client_directory_name="instantclient_${client_major}_${client_minor}"
client_directory="$install_root/$client_directory_name"
client_arch="${ORACLE_CLIENT_ARCH:-$(uname -m)}"

case "$client_arch" in
    amd64|x86_64)
        archive_arch="x64"
        basic_checksum="$ORACLE_CLIENT_LINUX_AMD64_BASICLITE_SHA256"
        sdk_checksum="$ORACLE_CLIENT_LINUX_AMD64_SDK_SHA256"
        ;;
    arm64|aarch64)
        archive_arch="arm64"
        basic_checksum="$ORACLE_CLIENT_LINUX_ARM64_BASICLITE_SHA256"
        sdk_checksum="$ORACLE_CLIENT_LINUX_ARM64_SDK_SHA256"
        ;;
    *)
        printf 'Unsupported Oracle Instant Client architecture: %s\n' "$client_arch" >&2
        exit 2
        ;;
esac

configure_libaio_compatibility() {
    local system_library

    if [[ -e "$client_directory/libaio.so.1" ]]; then
        return
    fi

    if [[ -L "$client_directory/libaio.so.1" ]]; then
        printf 'Found a dangling libaio compatibility link: %s\n' "$client_directory/libaio.so.1" >&2
        exit 1
    fi

    if ldconfig -p | awk '$1 == "libaio.so.1" { found = 1 } END { exit !found }'; then
        return
    fi

    system_library="$(ldconfig -p | awk '$1 == "libaio.so.1t64" { print $NF; exit }')"
    if [[ -z "$system_library" ]]; then
        printf 'Oracle Instant Client requires libaio.so.1, but no compatible libaio library was found.\n' >&2
        exit 1
    fi

    ln -s "$system_library" "$client_directory/libaio.so.1"
}

if [[ -f "$client_directory/libclntsh.so" && -f "$client_directory/sdk/include/oci.h" ]]; then
    configure_libaio_compatibility
    printf 'Oracle Instant Client already installed at %s\n' "$client_directory"
    exit 0
fi

if [[ -e "$client_directory" ]]; then
    printf 'Refusing to overwrite incomplete Oracle Instant Client directory: %s\n' "$client_directory" >&2
    exit 1
fi

mkdir -p "$install_root"
stage_directory="$(mktemp -d "$install_root/.oracle-client.XXXXXX")"
trap 'rm -rf -- "$stage_directory"' EXIT
basic_archive="$stage_directory/basiclite.zip"
sdk_archive="$stage_directory/sdk.zip"
download_base="https://download.oracle.com/otn_software/linux/instantclient/$ORACLE_CLIENT_DOWNLOAD_ID"

curl --fail --location --proto '=https' --tlsv1.2 --retry 3 \
    "$download_base/instantclient-basiclite-linux.$archive_arch-$ORACLE_CLIENT_VERSION.zip" \
    --output "$basic_archive"
printf '%s  %s\n' "$basic_checksum" "$basic_archive" | sha256sum --check

curl --fail --location --proto '=https' --tlsv1.2 --retry 3 \
    "$download_base/instantclient-sdk-linux.$archive_arch-$ORACLE_CLIENT_VERSION.zip" \
    --output "$sdk_archive"
printf '%s  %s\n' "$sdk_checksum" "$sdk_archive" | sha256sum --check

unzip -q "$basic_archive" -d "$stage_directory"
unzip -oq "$sdk_archive" -d "$stage_directory"
test -f "$stage_directory/$client_directory_name/libclntsh.so"
test -f "$stage_directory/$client_directory_name/sdk/include/oci.h"
mv -- "$stage_directory/$client_directory_name" "$client_directory"
configure_libaio_compatibility

printf 'Installed Oracle Instant Client at %s\n' "$client_directory"
