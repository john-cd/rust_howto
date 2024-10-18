# syntax=docker/dockerfile:1

# Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/
# See also https://docs.docker.com/language/rust/

ARG RUST_IMAGE_LABEL
FROM rust:${RUST_IMAGE_LABEL} as base
## Alternative: mcr.microsoft.com/devcontainers/rust:bookworm
## Repo for the Rust images: https://github.com/rust-lang/docker-rust/tree/master
## The full Rust image depends on https://github.com/docker-library/buildpack-deps

## TODO consider switching to slim or Alpine image. Need to solve openssl install
## https://stackoverflow.com/questions/70561544/rust-openssl-could-not-find-directory-of-openssl-installation
## https://www.cyberciti.biz/faq/perl-warning-setting-locale-failed-in-debian-ubuntu/
# ENV LC_CTYPE=en_US.UTF-8
# ENV LC_ALL=en_US.UTF-8

SHELL ["bash", "-c"]

## Install linker and other utilities
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends fzf mold clang \
                                                  aspell aspell-en \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
## Alternatively, use `lld` instead of the `mold` linker
## Configuration in .cargo/config.toml
## If using the `slim` Rust image, you may need to install: curl pkg-config libssl-dev

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
RUN <<EOF
    set -e
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall --no-confirm just \
    && cargo binstall --no-confirm mdbook-linkcheck \
    && cargo binstall --no-confirm mdbook-private \
    && cargo binstall --no-confirm mdbook-indexing \
    && cargo binstall --no-confirm mdbook-keeper \
    && cargo binstall --no-confirm --secure cargo-nextest
    # mdbook not installed
    if [ ! hash mdbook 2>/dev/null ]; then
        cargo binstall --version ${MDBOOK_VERSION} --no-confirm mdbook
    fi
    # mdbook installed but old version - force the installation of the new version
    if [[ $(mdbook -V) != "mdbook v${MDBOOK_VERSION}" ]]; then
        cargo binstall --version ${MDBOOK_VERSION} --no-confirm --force mdbook
    fi
EOF

# Optional:
#   && cargo binstall --no-confirm toml2json

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
# cargo binstall --no-confirm mdbook-open-on-gh

## TODO Install cargo check tools
# RUN cargo install --locked cargo-deny || true
# RUN cargo install --locked cargo-outdated || true
# RUN cargo install --locked cargo-udeps || true
# RUN cargo install --locked cargo-audit || true
# RUN cargo install --locked cargo-pants || true

## Install the mdbook-utils tool that is a companion to the book.
RUN cargo install --locked mdbook-utils

WORKDIR /code

## --------------------------------
## Stage for development
FROM base as development

## Additional packages for developement only
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get install -y firefox-esr
# locales xdg-utils desktop-file-utils \
# nano \

# RUN cargo binstall --no-confirm ripgrep

## https://code.visualstudio.com/remote/advancedcontainers/persist-bash-history
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/command_history/.bash_history" \
    && echo "$SNIPPET" >> "/root/.bashrc"

## Git config
ARG EMAIL
ARG GIT_AUTHOR_NAME

RUN git config --global user.email ${EMAIL} && git config --global user.name ${GIT_AUTHOR_NAME}

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

## Make sure the start scripts have permission to execute
RUN chmod +x ./.devcontainer/ci.sh; chmod +x ./.devcontainer/spellcheck.sh

ENTRYPOINT [ "./.devcontainer/ci.sh" ]
