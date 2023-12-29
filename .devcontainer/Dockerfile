# syntax=docker/dockerfile:1

FROM rust:1.75-bookworm as base
# alternative: mcr.microsoft.com/devcontainers/rust:bookworm

SHELL ["bash", "-c"]

## Install linker and utilities
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install fzf mold clang \
    # Remove imagemagick due to https://security-tracker.debian.org/tracker/CVE-2019-10131
    && apt-get purge -y imagemagick imagemagick-6-common \
    && apt-get autoremove -y && apt-get clean -y
# Alternatives: use the `lld` instead of `mold` linker
# Other packages that may be needed: wget

## Update Rust if needed (note: `rustup update stable` is required for Rust 1.74 -> Rust 1.75 somehow)
RUN rustup update stable && rustup default stable

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
ARG MDBOOK_VERSION="0.4.36"
RUN cargo binstall --version ${MDBOOK_VERSION} --no-confirm mdbook
## Alternatively
# RUN <<EOF
# set -e
# wget -c https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz -O - | sudo tar -xvz -C /usr/local/bin
# EOF

### Install mdbook-hide and keeper (note the workaround)
# RUN --mount=type=cache,target=/usr/local/cargo/registry/ \
#      cargo install --force mdbook-hide; \
#      cargo install --force mdbook-keeper --git https://github.com/tfpk/mdbook-keeper.git

### Install cargo check tools
#     cargo install --locked cargo-deny || true
#     cargo install --locked cargo-outdated || true
#     cargo install --locked cargo-udeps || true
#     cargo install --locked cargo-audit || true
#     cargo install --locked cargo-pants || true

VOLUME /usr/local/cargo/registry/
# See .cargo/config.toml
VOLUME /cargo-target-rust_howto/


## --------------------------------
## Stage to use for development
FROM base as development

## Update the working directory to match the workspace folder that VS Code is mounting to the container
## (see devcontainer.json)
ARG WORK_DIR
WORKDIR ${WORK_DIR}

## For `mdbook serve --open`
EXPOSE 3000

# Build stub project required by mdbook-keeper
# Print toolchain info
# Sleep to keep the container running
CMD cargo fmt --all; cargo build; rustup check; sleep infinity

## --------------------------------
## Stage for Continuous Integration
FROM base as ci

WORKDIR /build
COPY ./deps .
COPY ./src .
COPY ./theme .
COPY ./xmpl .
COPY book.toml Cargo.lock Cargo.toml deny.toml justfile ./

ENV CARGO_TARGET_DIR = ./target/

# Rust code lint (fails if not formatted properly)
RUN cargo fmt --all --check
RUN cargo clippy --workspace -- --deny warnings

## Check dependencies
#     cargo deny check
#     cargo outdated --exit-code 1
#     cargo udeps
#     rm -rf ~/.cargo/advisory-db
#     cargo audit
#     cargo pants

## Check all
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
RUN cargo check --workspace --all-targets --locked

## Build all
RUN cargo build --workspace --all-targets --locked --release

## Test all
RUN cargo test --workspace --all-targets --locked

## Test the examples embedded in the markdown
RUN mdbook test

## Build the book
RUN mdbook build
