#!/usr/bin/env bash
set -euo pipefail

# Directly install on Ubuntu all required dependencies to compile the book's code.
#
# Essentially a copy of the Dockerfile.

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
    libudev-dev \
    libvulkan-dev \
    libwayland-dev \
    libwebkit2gtk-4.1-dev \
    libxdo-dev \
    libxkbcommon-dev \
    libxkbcommon-x11-dev \
    llvm \
    m4 \
    make \
    pkg-config \
    protobuf-compiler \
    python3 \
    python3.12-dev \
    systemd \
    wget \
    xorg-dev \
&& sudo apt-get clean -y

sudo apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && sudo apt-get install -y jq fzf \
    && sudo apt-get clean

# Rust install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update && rustup component add clippy rustfmt
rustup toolchain install nightly \
    && rustup component add rustfmt clippy --toolchain nightly

## binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

cargo binstall --no-confirm --secure cargo-nextest

# mdbook
cargo binstall --version "0.4.35" --no-confirm --force mdbook

cargo binstall --no-confirm mdbook-linkcheck
cargo binstall --no-confirm mdbook-private
cargo binstall --no-confirm mdbook-indexing

cargo binstall --no-confirm just
cargo binstall --no-confirm cargo-deny
cargo binstall --no-confirm lychee

cargo install --locked mdbook-utils

# GitHub
curl -sS https://webi.sh/gh | sh

cargo install --locked kani-verifier && \
    cargo install --locked bacon && \
    cargo binstall --no-confirm ripgrep

git config --global user.email "John CD" && git config --global user.name john-cd@users.noreply.github.com
