# syntax=docker/dockerfile:1-labs

# Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/
# See also https://docs.docker.com/language/rust/

# The default value is overriden by the build arg passed by compose.yaml,
# itself set from the RUST_IMAGE_LABEL env. variable in `.devcontainer/.env` file
ARG RUST_IMAGE_LABEL=invalid

FROM rust:${RUST_IMAGE_LABEL} AS base
## Alternative: mcr.microsoft.com/devcontainers/rust:bookworm
## Repo for the Rust images: https://github.com/rust-lang/docker-rust/tree/master
## The full Rust image depends on https://github.com/docker-library/buildpack-deps

SHELL ["bash", "-c"]

## Install
## `aspell*` required for spell checking
## `systemd` required for one example using `journalctl`
## `gcc pkg-config libc6-dev libssl-dev` required for https://docs.rs/crate/lychee/latest
## `wget` required for `cmake` install
## `cmake`
## `libopencv-dev` `clang` `libclang-dev` for `opencv`
## `llvm` for `llvm-config` required for `clang-sys`
## for Tauri: `libwebkit2gtk-4.1-dev` `build-essential` `curl` `wget` `file` `libxdo-dev` `libssl-dev` `libayatana-appindicator3-dev` `librsvg2-dev`
## `python3` for `pyo3`
## `libgtk-4-dev` `build-essential` for `relm4` and `gtk4`
## `clang` `libwayland-dev` `libxkbcommon-x11-dev` `libvulkan-dev` for `xilem`
## `cmake build-essential libudev-dev xorg-dev libglu1-mesa-dev libasound2-dev libxkbcommon-dev` for openrr
## Note: the following caching should only be useful for development. Removed to save space on the GitHub Action runner
## --mount=type=cache,target=/var/cache/apt,sharing=locked \
## --mount=type=cache,target=/var/lib/apt,sharing=locked \
RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends \
    aspell \
    aspell-en \
    build-essential \
    capnproto \
    clang \
    cmake \
    curl \
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
    pkg-config \
    protobuf-compiler \
    python3 \
    python3.11-dev \
    systemd \
    wget \
    xorg-dev \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/* \
    && wget https://github.com/Kitware/CMake/releases/download/v3.31.0-rc3/cmake-3.31.0-rc3-linux-x86_64.sh \
    -q -O /tmp/cmake-install.sh \
    && chmod u+x /tmp/cmake-install.sh \
    && mkdir /opt/cmake-3.31.0-rc3/ \
    && /tmp/cmake-install.sh --skip-license --prefix=/opt/cmake-3.31.0-rc3/ \
    && rm /tmp/cmake-install.sh \
    && ln -s /opt/cmake-3.31.0-rc3/bin/* /usr/local/bin

## Optionally install linker and other utilities
# mold clang \
## Alternatively, use `lld` instead of the `mold` linker
## See configuration in .cargo/config.toml
## If using the `slim` Rust image, you may need to install: curl pkg-config libssl-dev

## Update Rust if needed
## Install clippy
## Install stable rustfmt (used by `rust-analyzer` e.g. when using VS code) and clippy
RUN rustup update && rustup component add clippy rustfmt

# The default value is overriden by the build arg passed by compose.yaml,
# itself set from the RUST_IMAGE_LABEL env. variable in `.devcontainer/.env` file
ARG MDBOOK_VERSION=invalid

## Install cargo binstall (binary install)
## Install nextest
## Install mdbook from the binary release
## Install mdbook preprocessors
## Note: you can cache the registry (not the bin folder) Only useful in development
## Disabled to save space on the GitHub Action runner
## --mount=type=cache,target=/usr/local/cargo/registry \
## See https://doc.rust-lang.org/cargo/guide/cargo-home.html
RUN <<EOF
    set -e
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

    ## TODO P3 CONSIDER: adds an "Edit this file on GitHub" link on the bottom of every page, linking directly to the source file.
    ## See https://lib.rs/crates/mdbook-open-on-gh
    # cargo binstall --no-confirm mdbook-open-on-gh

    # ------------------------------
    ## Technically the following section is only required for development
    ## but we want to cache these layers before `rustup nightly`,
    ## which changes very often.

    ## Install `just` (from the binary release)
    ## Install cargo plugins
    ## Install the link checker `lychee`
    cargo binstall --no-confirm just
    cargo binstall --no-confirm cargo-deny
    cargo binstall --no-confirm lychee
EOF

# ------------------------------

## Install the mdbook-utils tool that is a companion to the book.
## Installed second to last, because the tool is still in development
RUN cargo install --locked mdbook-utils

## Install nightly fmt - last, since it changes very often
## Rust edition 2024 preparation: make the `nightly` toolchain the default until release of Rust 1.85
RUN rustup toolchain install nightly \
    && rustup component add rustfmt clippy --toolchain nightly \
    && rustup default nightly

WORKDIR /code

## --------------------------------
## Stage for development
FROM base AS development

## Additional packages for development only
## You can cache with
## --mount=type=cache,target=/var/cache/apt,sharing=locked \
## --mount=type=cache,target=/var/lib/apt,sharing=locked \
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y jq fzf
# `fzf` is a fast fuzzy search used by `just`
## Other optional tools:
# firefox-esr \
# locales xdg-utils desktop-file-utils \
# nano \
# pipx \

## Install the GitHub CLI
RUN curl -sS https://webi.sh/gh | sh

## Install `pre-commit` (requires python / pipx)
## NOTE: `pre-commit` has been replaced by `cargo husky`
# RUN pipx install pre-commit \
#     && pipx ensurepath

## Install optional cargo plugins
## Install bacon tool
RUN <<EOF
cargo install --locked kani-verifier
cargo install --locked bacon
EOF
#  cargo install --locked cargo-outdated || true
#  cargo install --locked cargo-udeps || true
#  cargo install --locked cargo-audit || true
#  cargo install --locked cargo-pants || true

## Install optional tools: ripgrep
RUN cargo binstall --no-confirm ripgrep
## Optionally, install tool to convert TOML to JSON
#   && cargo binstall --no-confirm toml2json \

## `rustup` completions
## Command history - https://code.visualstudio.com/remote/advancedcontainers/persist-bash-history
RUN <<EOF
    set -e
    mkdir -p ~/.local/share/bash-completion/completions
    rustup completions bash > ~/.local/share/bash-completion/completions/rustup

    SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/command_history/.bash_history"
    echo "$SNIPPET" >> "/root/.bashrc"
EOF

## Git config
# The default values are overriden by the build arg passed by compose.override.yaml,
# itself set from the RUST_IMAGE_LABEL env. variable in `.devcontainer/.env` file
ARG EMAIL=nobody@users.noreply.github.com
ARG GIT_AUTHOR_NAME=nemo

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
## Stage for Continuous Integration (book building)
FROM base AS ci

## TODO P4 consider switching to slim or Alpine image for CI. Need to solve openssl install
## https://stackoverflow.com/questions/70561544/rust-openssl-could-not-find-directory-of-openssl-installation
## https://www.cyberciti.biz/faq/perl-warning-setting-locale-failed-in-debian-ubuntu/
# ENV LC_CTYPE=en_US.UTF-8
# ENV LC_ALL=en_US.UTF-8

ENV CARGO_TARGET_DIR=./target/

## The following is a bit of a hack to try to speed up CI: create separate docker layers for the dependencies and the code / examples
## Use `just push_ci` to build the CI image locally, push it to DockerHub; GitHub Actions will pull this image and use it as cache.

# TODO P2 reconsider when we can reliably use cached docker images in GitHub Actions; slow and takes space

## Copy all the Cargo.toml / lock (and config.toml files) of the workspace
## The --parents flag preserves parent directories for the source entries.
## NOTE: [lib] or [[bin]] targets must be defined in all Cargo.toml, since the code is not present at this stage and autodiscovery won't work.
#COPY --parents .cargo/ Cargo.toml Cargo.lock crates/about/Cargo.toml crates/categories/*/Cargo.toml \
#    crates/language/Cargo.toml crates/other/Cargo.toml crates/standard_library/Cargo.toml \
#    crates/playground/Cargo.toml crates/publish/Cargo.toml crates/tools/*/Cargo.toml crates/xmpl/*/Cargo.toml ./

## Bake the dependencies into (a separate layer of) the CI image (since they don't change often).
#RUN cargo fetch

## Optionally pre-compile the dependencies but not the code of tools and examples (which change more often)
## a) Copy the (empty) src/lib.rs for default members of the workspace (no other code)
# COPY --parents crates/about/src/ crates/categories/*/src/ crates/language/src crates/other/src crates/standard_library/src ./
## b) Compile the dependencies (and src/lib.rs)
# RUN cargo build --all-targets --locked --profile ci

## Copy the whole code in the container (excluding what is in .dockerignore)
COPY . .

## Make sure the start scripts have permission to execute
RUN chmod +x ./.devcontainer/ci.sh; chmod +x ./.devcontainer/spellcheck.sh

## Compilation of the whole code happens within that script at runtime
ENTRYPOINT [ "./.devcontainer/ci.sh" ]
