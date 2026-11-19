#!/usr/bin/env bash

# TODO Reviewed ??-??-2026.

# Install a checksum-pinned ONNX Runtime CPU build for the `ort` examples.

set -euo pipefail

: "${ONNXRUNTIME_VERSION:?Set ONNXRUNTIME_VERSION from .devcontainer/.env}"
: "${ONNXRUNTIME_NUGET_SHA512:?Set ONNXRUNTIME_NUGET_SHA512 from .devcontainer/.env}"

install_root="${1:-/opt/onnxruntime}"
runtime_arch="${ONNXRUNTIME_ARCH:-$(uname -m)}"

case "$runtime_arch" in
    amd64|x86_64)
        archive_arch="x64"
        package_arch="x64"
        ;;
    arm64|aarch64)
        archive_arch="aarch64"
        package_arch="arm64"
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
package="$stage_directory/Microsoft.ML.OnnxRuntime.$ONNXRUNTIME_VERSION.nupkg"

curl --fail --location --proto '=https' --tlsv1.2 --retry 3 \
    "https://www.nuget.org/api/v2/package/Microsoft.ML.OnnxRuntime/$ONNXRUNTIME_VERSION" \
    --output "$package"
printf '%s  %s\n' "$ONNXRUNTIME_NUGET_SHA512" "$package" | sha512sum --check

unzip -q "$package" "runtimes/linux-$package_arch/native/*.so" -d "$stage_directory/package"
native_directory="$stage_directory/package/runtimes/linux-$package_arch/native"
test -f "$native_directory/libonnxruntime.so"
test -f "$native_directory/libonnxruntime_providers_shared.so"
mkdir -p "$stage_directory/$archive_name/lib"
cp -a -- "$native_directory"/*.so "$stage_directory/$archive_name/lib/"
mv -- "$stage_directory/$archive_name" "$runtime_directory"
ln -sfn -- "$runtime_directory" "$install_root/current"

printf 'Installed ONNX Runtime at %s\n' "$runtime_directory"
