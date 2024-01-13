# syntax=docker/dockerfile:1

# Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/
# See also https://docs.docker.com/language/rust/

ARG RUST_IMAGE_LABEL
FROM rust:${RUST_IMAGE_LABEL} as base
# alternative: mcr.microsoft.com/devcontainers/rust:bookworm

SHELL ["bash", "-c"]

## Install linker and other utilities
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends fzf mold clang curl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Alternatively, use `lld` instead of the `mold` linker
# Configuration in .cargo/config.toml

## Update Rust if needed (note: `rustup update stable && rustup default stable` is required for Rust 1.74 -> Rust 1.75 somehow)
RUN rustup update

## Install clippy if needed
RUN rustup component add clippy

## Install nightly fmt
RUN rustup toolchain install nightly \
    && rustup component add rustfmt --toolchain nightly

ARG MDBOOK_VERSION

## Install cargo binstall (binary install)
## Install just from the binary release
## Install mdbook from the binary release
## Install ripgrep
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall --no-confirm just \
    && cargo binstall --version ${MDBOOK_VERSION} --no-confirm mdbook \
    && cargo binstall --no-confirm mdbook-linkcheck \
    && cargo binstall --no-confirm ripgrep \
    && cargo binstall --no-confirm --secure cargo-nextest

## Alternative just install
# # Prereqs to install Just with apt: https://just.systems/man/en/chapter_4.html
# RUN <<EOF
#   set -e
#   wget -qO - 'https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub' | gpg --dearmor | sudo tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null
#   echo "deb [arch=all,$(dpkg --print-architecture) signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr $(lsb_release -cs)" | sudo tee /etc/apt/sources.list.d/prebuilt-mpr.list
#   sudo apt update
#   apt-get -y install just
# EOF

## Alternative mdbook install
# RUN <<EOF
# set -e
# wget -c https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz -O - | sudo tar -xvz -C /usr/local/bin
# EOF

## Optional: install mdbook-hide and keeper (note the workaround to get the unreleased keeper)
# RUN --mount=type=cache,target=/usr/local/cargo/registry/ \
#      cargo install --force mdbook-hide; \
#      cargo install --force mdbook-keeper --git https://github.com/tfpk/mdbook-keeper.git

## Optional: adds an "Edit this file on GitHub" link on the bottom of every page, linking directly to the source file.
## TODO https://lib.rs/crates/mdbook-open-on-gh

## TODO Install cargo check tools
# RUN cargo install --locked cargo-deny || true
# RUN cargo install --locked cargo-outdated || true
# RUN cargo install --locked cargo-udeps || true
# RUN cargo install --locked cargo-audit || true
# RUN cargo install --locked cargo-pants || true

WORKDIR /code

## --------------------------------
## Stage to use for development
FROM base as development

## https://code.visualstudio.com/remote/advancedcontainers/persist-bash-history
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/command_history/.bash_history" \
    && echo "$SNIPPET" >> "/root/.bashrc"

## For `mdbook serve --open`
EXPOSE 3000

## https://code.visualstudio.com/remote/advancedcontainers/start-processes
COPY --chmod=755 .devcontainer/dev.sh ./.devcontainer/
ENTRYPOINT [ "./.devcontainer/dev.sh" ]

## Sleep to keep the container running.
## The command is passed to `dev.sh`.
CMD [ "sleep", "infinity" ]

## --------------------------------
## Stage for Continuous Integration
FROM base as ci

## Copy the code in the container (in WORKDIR), excluding what is in .dockerignore
COPY . .

ENV CARGO_TARGET_DIR=./target/

## Make sure the start script has permission to execute
RUN chmod +x ./.devcontainer/ci.sh

ENTRYPOINT [ "./.devcontainer/ci.sh" ]
