# syntax=docker/dockerfile:1

# Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/
# See also https://docs.docker.com/language/rust/

ARG RUST_VERSION=1.75.0
FROM rust:${RUST_VERSION}-bookworm as base
# alternative: mcr.microsoft.com/devcontainers/rust:bookworm

SHELL ["bash", "-c"]

## Install linker and other utilities
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install fzf mold clang \
    # Remove imagemagick due to https://security-tracker.debian.org/tracker/CVE-2019-10131
    && apt-get purge -y imagemagick imagemagick-6-common \
    && apt-get autoremove -y && apt-get clean -y
# Alternatively, use `lld` instead of the `mold` linker
# Configuration in .cargo/config.toml

## Update Rust if needed (note: `rustup update stable && rustup default stable` is required for Rust 1.74 -> Rust 1.75 somehow)
RUN rustup update

## Install fmt and clippy if needed
RUN rustup component add clippy
RUN rustup component add rustfmt

## Install cargo binstall (binary install)
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

## Install just from the binary release
RUN cargo binstall --no-confirm just
## Alternatively
# # Prereqs to install Just with apt: https://just.systems/man/en/chapter_4.html
# RUN <<EOF
#   set -e
#   wget -qO - 'https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub' | gpg --dearmor | sudo tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null
#   echo "deb [arch=all,$(dpkg --print-architecture) signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr $(lsb_release -cs)" | sudo tee /etc/apt/sources.list.d/prebuilt-mpr.list
#   sudo apt update
#   apt-get -y install just
# EOF

## Install mdbook from the binary release
ARG MDBOOK_VERSION=0.4.36
RUN cargo binstall --version ${MDBOOK_VERSION} --no-confirm mdbook
## Alternatively
# RUN <<EOF
# set -e
# wget -c https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz -O - | sudo tar -xvz -C /usr/local/bin
# EOF

### Install mdbook-hide and keeper (note the workaround to get the unreleased keeper)
# RUN --mount=type=cache,target=/usr/local/cargo/registry/ \
#      cargo install --force mdbook-hide; \
#      cargo install --force mdbook-keeper --git https://github.com/tfpk/mdbook-keeper.git

### Install cargo check tools
# RUN cargo install --locked cargo-deny || true
# RUN cargo install --locked cargo-outdated || true
# RUN cargo install --locked cargo-udeps || true
# RUN cargo install --locked cargo-audit || true
# RUN cargo install --locked cargo-pants || true


## --------------------------------
## Stage to use for development
FROM base as development

## Dev Container typically mounts the project root to /workspace/<project_name>/ in the container.
## There is no way to specify Docker's option -w in `devcontainer.json`.
## Therefore we pass the path as a build arg and use it to set the working directory in the container.
## (see devcontainer.json)
ARG WORK_DIR
WORKDIR ${WORK_DIR}

## `devcontainer.json` mounts the following two folders to named volumes for caching purposes:
## - /usr/local/cargo/registry/
## - /cargo-target-rust_howto/
## The latter is the compile target directory which is overriden in `.cargo/config.toml`

## For `mdbook serve --open`
EXPOSE 3000

## Sleep to keep the container running.
CMD sleep infinity

## --------------------------------
## Stage for Continuous Integration
FROM base as ci

WORKDIR /build
COPY . .

ENV CARGO_TARGET_DIR=./target/

## Checks the Rust code formatting
## Fails if not formatted properly
RUN cargo fmt --all --check

## Lint all examples
## - Elevate clippy warnings to errors, which will in turn fail the build.
## - `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
RUN cargo clippy --workspace --all-targets -- --deny warnings

## Check dependencies
# RUN cargo deny check \
#     && cargo outdated --exit-code 1 \
#     && cargo udeps \
#     && rm -rf ~/.cargo/advisory-db \
#     && cargo audit \
#     && cargo pants

## Check all examples
RUN cargo check --workspace --all-targets --locked

## Make sure all examples compile
RUN cargo build --workspace --all-targets --locked
## --release is not necessary; we don't execute any of them later

## Test all examples
RUN cargo test --workspace --all-targets --locked

## Test the examples embedded in the markdown
RUN mdbook test

## Build the book and copy into ./book
RUN mdbook build
