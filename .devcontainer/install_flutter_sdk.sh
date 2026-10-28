#!/usr/bin/env bash
# Download and install a version-pinned Flutter source checkout.

set -euo pipefail

if [[ $# -ne 3 ]]; then
    echo "Usage: install_flutter_sdk.sh <version> <git-revision> <absolute-install-path>" >&2
    exit 2
fi

version="$1"
expected_revision="$2"
install_path="${3%/}"

if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Flutter version must be a stable x.y.z version: $version" >&2
    exit 2
fi

if [[ ! "$expected_revision" =~ ^[0-9a-f]{40}$ ]]; then
    echo "Flutter Git revision must be a 40-character lowercase SHA: $expected_revision" >&2
    exit 2
fi

if [[ "$install_path" != /* ]]; then
    echo "Flutter install path must be absolute: $install_path" >&2
    exit 2
fi

if [[ -x "$install_path/bin/flutter" ]]; then
    current_revision="$(git -C "$install_path" rev-parse HEAD 2>/dev/null || true)"
    if [[ "$current_revision" == "$expected_revision" ]]; then
        echo "Flutter $version is already installed at $install_path"
        exit 0
    fi
fi

if [[ -e "$install_path" ]]; then
    echo "Flutter install path already exists and does not match the requested revision: $install_path" >&2
    exit 1
fi

install_parent="$(dirname "$install_path")"
mkdir -p "$install_parent"
temporary_directory="$(mktemp -d "${TMPDIR:-/tmp}/flutter-install.XXXXXXXX")"
trap 'rm -rf -- "$temporary_directory"' EXIT

archive_path="$temporary_directory/flutter.tar.gz"
source_path="$temporary_directory/source"
mkdir "$source_path"

curl --fail --location --proto '=https' --tlsv1.2 --silent --show-error \
    "https://codeload.github.com/flutter/flutter/tar.gz/refs/tags/$version" \
    --output "$archive_path"
tar -xzf "$archive_path" --strip-components=1 -C "$source_path"
if [[ ! -x "$source_path/bin/flutter" ]]; then
    echo "Flutter source archive did not contain an executable bin/flutter." >&2
    exit 1
fi

git -C "$source_path" init --initial-branch=stable
git -C "$source_path" remote add origin https://github.com/flutter/flutter.git
git -C "$source_path" config core.autocrlf false
git -C "$source_path" fetch --depth=1 --filter=blob:none origin \
    "refs/tags/$version:refs/tags/$version"

actual_revision="$(git -C "$source_path" rev-parse "refs/tags/$version^{commit}")"
if [[ "$actual_revision" != "$expected_revision" ]]; then
    echo "Flutter $version resolved to $actual_revision; expected $expected_revision" >&2
    exit 1
fi

git -C "$source_path" update-ref refs/heads/stable "$actual_revision"
git -C "$source_path" read-tree "$actual_revision"
mv -- "$source_path" "$install_path"

echo "Installed Flutter $version ($actual_revision) at $install_path"
