#!/usr/bin/env bash
set -euo pipefail

# Directly install on Ubuntu all required dependencies to compile the book's code.
#
# Essentially a copy of the Dockerfile.

MDBOOK_VERSION="0.4.43"
PANDOC_VERSION="3.6"

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
    diffutils \
    file \
    gcc \
    libasound2-dev \
    libayatana-appindicator3-dev \
    libc6-dev \
    libclang-dev \
    libglu1-mesa-dev \
    libgtk-4-dev \
    libopencv-dev \
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
    pkg-config \
    protobuf-compiler \
    python3 \
    wget \
    xorg-dev \
    jq \
    fzf \
&& sudo apt-get clean -y
&& rm -rf /var/lib/apt/lists/*

# Install Pandoc binary
wget -q "https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-1-amd64.deb" -O /tmp/pandoc.deb \
    && sudo dpkg -i /tmp/pandoc.deb \
    && rm /tmp/pandoc.deb

# Rust install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup update && rustup component add clippy rustfmt
rustup toolchain install nightly \
    && rustup component add rustfmt clippy --toolchain nightly

## binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

cargo binstall --no-confirm --secure \
    cargo-nextest \
    mdbook@${MDBOOK_VERSION} \
    mdbook-linkcheck \
    mdbook-private \
    mdbook-pandoc \
    tectonic \
    sccache \
    just \
    cargo-deny \
    lychee \
    bacon \
    ripgrep

cargo install --force mdbook-indexing
cargo install --locked mdbook-utils

# GitHub
curl -sS https://webi.sh/gh | sh

cargo binstall --no-confirm --secure kani-verifier

git config --global user.email "John CD" && git config --global user.name john-cd@users.noreply.github.com
