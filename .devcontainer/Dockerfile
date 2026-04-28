# syntax=docker/dockerfile:1-labs

# This Dockerfile defines multiple stages to build a Rust development environment optimized for building the Rust How-To book and its examples.
# The stages are:
# 1. `tool-builder`: Builds and installs Rust tools and binaries (e.g., mdbook and its plugins) in a Rust image.
# 2. `base`: Sets up the base image with necessary system dependencies and copies the tools from the builder stage.
# 3. `tools-app-builder`: Builds local Rust tools (e.g., mdbook-scrub) from the source code.
# 4. `development`: The final image for development, which includes additional tools and configurations for an optimal development experience.
# 5. `ci`: A stage optimized for Continuous Integration, which includes only the necessary tools and dependencies to build the book in a CI environment.
# Each stage is designed to optimize caching and reduce the final image size by only including necessary components
#
# Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/
# See also https://docs.docker.com/language/rust/

# ------------------------------------------------------------------------------------------------
# The base Rust image stage, used as the starting point for both the tool builders and the final images.

# The default value is overriden by the build arg passed by compose.yaml,
# itself set from the RUST_IMAGE_LABEL env. variable in `.devcontainer/.env` file
ARG RUST_IMAGE_LABEL=invalid

FROM rust:${RUST_IMAGE_LABEL} AS rust-core
## Repo for the Rust images: https://github.com/rust-lang/docker-rust/tree/master
## The full Rust image depends on https://github.com/docker-library/buildpack-deps

## Install stable rustfmt (used by `rust-analyzer` e.g. when using VS code) and clippy
RUN rustup component add clippy rustfmt

## Install nightly with rustfmt - last, since it changes often
RUN rustup toolchain install nightly --profile minimal --component rustfmt
## Optional: make nightly the default
#     && rustup default nightly
## You must update all .github wokflows as well

# ------------------------------------------------------------------------------------------------
# Stage to build and install (external) Rust tools (e.g., mdbook and its plugins) in the image, used for both development and CI stages.

FROM rust-core AS tool-builder

# The default value is overriden by the build arg passed by compose.yaml,
# itself set from the RUST_IMAGE_LABEL env. variable in `.devcontainer/.env` file
ARG MDBOOK_VERSION=invalid

## Install cargo binstall (binary install)
## Install nextest
## Install mdbook
## Install mdbook preprocessors
## Note: you can cache the registry (not the bin folder)- only useful in development
## Disable to save space on the GitHub Action runner
## --mount=type=cache,target=/usr/local/cargo/registry \
## See https://doc.rust-lang.org/cargo/guide/cargo-home.html
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    <<EOF
    set -e
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
        lychee
    cargo install --force mdbook-indexing
EOF

## Install the mdbook-utils tool that is a companion to the book.
## Installed last, because the tool is still in development
RUN cargo install --locked mdbook-utils

## ------------------------------------------------------------------------------------------------

## Stage to build local tools (e.g., mdbook-scrub)
FROM rust-core AS local-tool-builder

WORKDIR /code

RUN mkdir -p /code/bin/

## Set target directory to a predictable path
ENV CARGO_TARGET_DIR=/code/target/tools/

## Copy only the tools source to optimize caching
COPY tools/ /code/tools/

RUN cargo fetch --locked --manifest-path /code/tools/Cargo.toml

## Build the tools in release mode
RUN --mount=type=cache,target=/usr/local/cargo/registry \
  cargo +nightly build --bins --locked --all-features --release --artifact-dir /code/bin --manifest-path /code/tools/Cargo.toml

# ------------------------------------------------------------------------------------------------

# The base image with all system dependencies and tools installed, used for both development and CI stages.
FROM rust-core AS base

SHELL ["bash", "-c"]

WORKDIR /code

## Install
## `aspell*` required for spell checking
## `systemd` required for one example using `journalctl`
## `gcc pkg-config libc6-dev libssl-dev` required for https://docs.rs/crate/lychee/latest
## `wget` required for `cmake` install
## `cmake`
## `libopencv-dev` `clang` `libclang-dev` for `opencv`
## `llvm` for `llvm-config` required for `clang-sys`
## `libwebkit2gtk-4.1-dev` `build-essential` `curl` `wget` `file` `libxdo-dev` `libssl-dev` `libayatana-appindicator3-dev` `librsvg2-dev` for `tauri`
## `python3` for `pyo3`
## `libgtk-4-dev` `build-essential` for `relm4` and `gtk4`
## `clang` `libwayland-dev` `libxkbcommon-x11-dev` `libvulkan-dev` for `xilem`
## `cmake build-essential libudev-dev xorg-dev libglu1-mesa-dev libasound2-dev libxkbcommon-dev` for `openrr`
## `diffutils gcc m4 make` for `gmp-mpfr-sys` / `rug`
## `pandoc` for `mdbook-pandoc`
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends \
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
    locales \
    mold \
    openssl \
    llvm \
    m4 \
    make \
    pkg-config \
    protobuf-compiler \
    python3 \
    wget \
    xorg-dev \
    && apt-get clean -y

## Alternatively, use `lld` instead of the `mold` linker
## See configuration in .cargo/config.toml
## If using the `slim` Rust image, you may need to install: curl pkg-config libssl-dev

## Fix perl warning: Setting locale failed
RUN sed -i '/en_US.UTF-8/s/^# //g' /etc/locale.gen && \
    locale-gen
ENV LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en \
    LC_ALL=en_US.UTF-8

## Install pandoc from the official GitHub binary release.
## Using a direct binary download instead of `apt install pandoc` avoids pulling in large Haskell dependencies.
## Pandoc is required by the `mdbook-pandoc` backend to generate the PDF version of the book.
ARG PANDOC_VERSION=invalid
RUN wget -q "https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-1-amd64.deb" \
    -O /tmp/pandoc.deb \
    && dpkg -i /tmp/pandoc.deb \
    && rm /tmp/pandoc.deb

## Copy only binaries from builder to keep the base image lean
COPY --from=tool-builder /usr/local/cargo/bin/ /usr/local/cargo/bin/

## ------------------------------------------------------------------------------------------------

## Stage for development / Dev Container, with additional tools and configurations for an optimal development experience.
FROM base AS development

## Additional packages for development only.
## You can cache with
## --mount=type=cache,target=/var/cache/apt,sharing=locked \
## --mount=type=cache,target=/var/lib/apt,sharing=locked \
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y jq fzf \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
# `fzf` is a fast fuzzy search used by `just`
## Other optional tools:
# firefox-esr \
# locales xdg-utils desktop-file-utils \
# nano \
# pipx \

## Install the GitHub CLI
RUN curl -sS https://webi.sh/gh | sh

## Install additional dev tools
RUN cargo binstall --no-confirm --secure kani-verifier bacon ripgrep
## Install optional cargo plugins:
#  cargo install --locked cargo-outdated || true
#  cargo-udeps
#  cargo-audit
#  cargo-pants
#  cargo-sweep
#  cargo-cache
## Optionally, install tool to convert TOML to JSON
#   && cargo binstall --no-confirm toml2json \

## Copy only binaries from local tool builder to keep the base image lean
COPY --from=local-tool-builder /code/bin/ /code/bin/

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
COPY --link --chmod=755 .devcontainer/dev.sh ./.devcontainer/
ENTRYPOINT [ "./.devcontainer/dev.sh" ]

## Sleep to keep the container running.
## The command is passed to `dev.sh`.
CMD [ "sleep", "infinity" ]

## ------------------------------------------------------------------------------------------------

## Stage for Continuous Integration (book building)
FROM base AS ci

## Set environment variables to optimize for CI compilation
ENV CARGO_INCREMENTAL=0 \
    RUST_BACKTRACE=1

## Copy local tools from the builder stage
COPY --from=local-tool-builder /code/bin/ /code/bin/

## Copy the whole code in the container (excluding what is in .dockerignore)
## Ensure .dockerignore excludes large directories like .git and target/
COPY --link . .

## Make sure the start scripts have permission to execute
RUN find . -name "target" -prune -o \( -type f -name "*.sh" -exec chmod +x {} \; \)

## Cleanup: remove cargo registry index to save space if not using cache mounts here
## This reduces image size by hundreds of megabytes.
RUN rm -rf /usr/local/cargo/registry/index/*

## Compilation of the whole code happens within that script at runtime
ENTRYPOINT [ "./.devcontainer/ci.sh" ]
