#!/usr/bin/env bash
# Purpose: Install the operating-system and Rust tools required by the development container.
# Usage: Run with bash .devcontainer/install_deps.sh from the repository root; no arguments are required.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

# Directly install on Ubuntu all required dependencies to compile the book's code.
#
# Essentially a copy of the Dockerfile.

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# Keep Linux developer installs aligned with the versions used by Docker and CI.
set -a
# shellcheck source=.env
source "$repo_root/.devcontainer/.env"
set +a
RUSTFMT_TOOLCHAIN="${RUSTFMT_TOOLCHAIN:-$(sed -n 's/^channel *= *"\([^"]*\)".*/\1/p' "$repo_root/rustfmt-toolchain.toml")}"

# GTK4 and pkg-config are required for the Rust GUI examples that depend on gtk4/relm4.
sudo apt-get update \
&& export DEBIAN_FRONTEND=noninteractive \
&& sudo apt-get install -y --no-install-recommends \
    aspell \
    aspell-en \
    build-essential \
    capnproto \
    clang \
    cmake \
    curl \
    default-jdk \
    diffutils \
    file \
    gcc \
    libasound2-dev \
    libayatana-appindicator3-dev \
    libc6-dev \
    libclang-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    libglu1-mesa-dev \
    libgraphite2-dev \
    libharfbuzz-dev \
    libgtk-4-dev \
    libicu-dev \
    libopencv-dev \
    libpng-dev \
    libpython3-dev \
    librocksdb-dev \
    librsvg2-dev \
    libssl-dev \
    libsystemd-dev \
    libudev-dev \
    libvulkan-dev \
    libwayland-dev \
    libwebkit2gtk-4.1-dev \
    libxdo-dev \
    libxkbcommon-dev \
    libxkbcommon-x11-dev \
    libz-dev \
    mold \
    llvm \
    m4 \
    make \
    nasm \
    ninja-build \
    pkg-config \
    protobuf-compiler \
    python3 \
    python-is-python3 \
    python3-venv \
    ruby-dev \
    wget \
    xorg-dev \
    jq \
    fzf \
&& sudo apt-get clean -y

# Install Pandoc binary
wget -q "https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-1-amd64.deb" -O /tmp/pandoc.deb \
    && sudo dpkg -i /tmp/pandoc.deb \
    && rm /tmp/pandoc.deb

# Rust install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup update && rustup component add clippy rustfmt
rustup toolchain install "$RUSTFMT_TOOLCHAIN" \
    && rustup component add rustfmt clippy --toolchain "$RUSTFMT_TOOLCHAIN"

## binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

cargo binstall --no-confirm --secure --version "$CARGO_NEXTEST_VERSION" cargo-nextest
cargo binstall --no-confirm --secure --version "$MATURIN_VERSION" maturin
cargo binstall --no-confirm --secure --version "$MDBOOK_VERSION" mdbook
cargo binstall --no-confirm --secure --version "$MDBOOK_LINKCHECK_VERSION" mdbook-linkcheck
cargo binstall --no-confirm --secure --version "$MDBOOK_PANDOC_VERSION" mdbook-pandoc
cargo binstall --no-confirm --secure --version "$JUST_VERSION" just
cargo binstall --no-confirm --secure --force --version "$CARGO_FMT_TOML_VERSION" cargo-fmt-toml
cargo binstall --no-confirm --secure --force --version "$DPRINT_VERSION" dprint
cargo binstall --no-confirm --secure --force --version "$RUMDL_VERSION" rumdl
cargo binstall --no-confirm --secure sccache cargo-deny lychee bacon ripgrep

if [[ "$(uname -m)" == "x86_64" ]]; then
    tectonic_archive="$(mktemp)"
    curl --fail --location --proto '=https' --tlsv1.2 \
        "https://github.com/tectonic-typesetting/tectonic/releases/download/tectonic%40${TECTONIC_VERSION}/tectonic-${TECTONIC_VERSION}-x86_64-unknown-linux-gnu.tar.gz" \
        --output "$tectonic_archive"
    echo "${TECTONIC_LINUX_SHA256}  ${tectonic_archive}" | sha256sum --check
    tar -xzf "$tectonic_archive" -C "$HOME/.cargo/bin" tectonic
    chmod +x "$HOME/.cargo/bin/tectonic"
    rm -f "$tectonic_archive"
else
    cargo binstall --no-confirm --secure --version "$TECTONIC_VERSION" tectonic
fi

cargo install --locked --force --version "$MDBOOK_INDEXING_VERSION" mdbook-indexing
cargo install --git https://github.com/john-cd/mdbook-utils --rev "$MDBOOK_UTILS_REV" --locked --force mdbook-utils

# GitHub
curl -sS https://webi.sh/gh | sh

cargo binstall --no-confirm --secure kani-verifier
tectonic --version
cargo fmt-toml --version
dprint --version
rumdl --version
