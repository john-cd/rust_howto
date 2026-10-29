#!/usr/bin/env bash
# Install a checksum-pinned ONNX Runtime CPU build for the `ort` examples.

set -euo pipefail

: "${ONNXRUNTIME_VERSION:?Set ONNXRUNTIME_VERSION from .devcontainer/.env}"

install_root="${1:-/opt/onnxruntime}"
runtime_arch="${ONNXRUNTIME_ARCH:-$(uname -m)}"

case "$runtime_arch" in
    amd64|x86_64)
        archive_arch="x64"
        checksum="$ONNXRUNTIME_LINUX_X64_SHA256"
        ;;
    arm64|aarch64)
        archive_arch="aarch64"
        checksum="$ONNXRUNTIME_LINUX_AARCH64_SHA256"
        ;;
    *)
        printf 'Unsupported ONNX Runtime architecture: %s\n' "$runtime_arch" >&2
        exit 2
        ;;
esac

archive_name="onnxruntime-linux-$archive_arch-$ONNXRUNTIME_VERSION"
runtime_directory="$install_root/$archive_name"

if [[ -f "$runtime_directory/lib/libonnxruntime.so" ]]; then
    ln -sfn -- "$runtime_directory" "$install_root/current"
    printf 'ONNX Runtime already installed at %s\n' "$runtime_directory"
    exit 0
fi

if [[ -e "$runtime_directory" ]]; then
    printf 'Refusing to overwrite incomplete ONNX Runtime directory: %s\n' "$runtime_directory" >&2
    exit 1
fi

mkdir -p "$install_root"
stage_directory="$(mktemp -d "$install_root/.onnxruntime.XXXXXX")"
trap 'rm -rf -- "$stage_directory"' EXIT
archive="$stage_directory/$archive_name.tgz"

curl --fail --location --proto '=https' --tlsv1.2 --retry 3 \
    "https://github.com/microsoft/onnxruntime/releases/download/v$ONNXRUNTIME_VERSION/$archive_name.tgz" \
    --output "$archive"
printf '%s  %s\n' "$checksum" "$archive" | sha256sum --check

tar -xzf "$archive" -C "$stage_directory"
test -f "$stage_directory/$archive_name/lib/libonnxruntime.so"
mv -- "$stage_directory/$archive_name" "$runtime_directory"
ln -sfn -- "$runtime_directory" "$install_root/current"

printf 'Installed ONNX Runtime at %s\n' "$runtime_directory"
