#!/usr/bin/env bash
set -euo pipefail

# Directly install on Ubuntu all required dependencies to compile the book's code.
#
# Essentially a copy of the Dockerfile.

MDBOOK_VERSION="0.4.35"

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
&& sudo apt-get clean -y \

rustup update && rustup component add clippy rustfmt

curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

cargo binstall --no-confirm --secure cargo-nextest

# mdbook not installed
if [ ! hash mdbook 2>/dev/null ]; then
    cargo binstall --version ${MDBOOK_VERSION} --no-confirm mdbook
fi
# mdbook installed but old version - force the installation of the new version
if [[ $(mdbook -V) != "mdbook v${MDBOOK_VERSION}" ]]; then
    cargo binstall --version ${MDBOOK_VERSION} --no-confirm --force mdbook
fi

cargo binstall --no-confirm mdbook-linkcheck
cargo binstall --no-confirm mdbook-private
cargo binstall --no-confirm mdbook-indexing

cargo binstall --no-confirm just
cargo binstall --no-confirm cargo-deny
cargo binstall --no-confirm lychee

cargo install --locked mdbook-utils

rustup toolchain install nightly \
    && rustup component add rustfmt clippy --toolchain nightly

sudo apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && sudo apt-get install -y jq fzf \
    && sudo apt-get clean

curl -sS https://webi.sh/gh | sh

cargo install --locked kani-verifier && \
    cargo install --locked bacon && \
    cargo binstall --no-confirm ripgrep

#EMAIL=nobody@users.noreply.github.com
#GIT_AUTHOR_NAME=nemo
#git config --global user.email ${EMAIL} && git config --global user.name ${GIT_AUTHOR_NAME}
