# syntax=docker/dockerfile:1-labs
# TODO Reviewed ??-??-2026.

# This multi-stage build supplies the Rust toolchains and native libraries used by the book workspace.
# `rust-core` installs the pinned formatter toolchain. `tool-builder` installs external Rust tools,
# and `local-tool-builder` builds this repository's tools. `base` combines those tools with native
# libraries; `development` adds interactive utilities, while `ci` copies in the source tree and runs
# the CI entry point.
#
# Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/
# See also https://docs.docker.com/language/rust/

# ------------------------------------------------------------------------------------------------
# Start from the official Rust image for all build stages.

# The build arg is passed by compose.yaml and set from the RUST_IMAGE_LABEL
# variable in `.devcontainer/.env`.
ARG RUST_IMAGE_LABEL=1.99.1-slim-trixie

FROM rust:${RUST_IMAGE_LABEL} AS rust-core
## Repo for the Rust images: https://github.com/rust-lang/docker-rust/tree/master
## The full Rust image depends on https://github.com/docker-library/buildpack-deps

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

## Install stable rustfmt for editor integration and Clippy for linting.
RUN rustup component add clippy rustfmt

## Install the repository-pinned formatter toolchain after stable, because it changes more often.
ARG RUSTFMT_TOOLCHAIN
COPY rustfmt-toolchain.toml /usr/local/share/rustfmt-toolchain.toml
ENV RUSTFMT_TOOLCHAIN_FILE=/usr/local/share/rustfmt-toolchain.toml
RUN RUSTFMT_TOOLCHAIN="${RUSTFMT_TOOLCHAIN:-$(sed -n 's/^channel *= *"\([^"]*\)".*/\1/p' "$RUSTFMT_TOOLCHAIN_FILE")}" \
    && rustup toolchain install "$RUSTFMT_TOOLCHAIN" --profile minimal --component rustfmt
ENV RUSTFMT_TOOLCHAIN=${RUSTFMT_TOOLCHAIN}

## Optional: make the pinned formatter toolchain the default for interactive Cargo commands.
# RUN rustup default "$RUSTFMT_TOOLCHAIN"
## Update CI workflow toolchain pins when changing the repository's formatter toolchain.

# ------------------------------------------------------------------------------------------------
# Stage to build and install (external) Rust tools (e.g., mdbook and its plugins) in the image, used for both development and CI stages.

FROM rust-core AS tool-builder

# Tool versions are passed as build args by compose.yaml and set in
# `.devcontainer/.env`.
ARG MDBOOK_VERSION
ARG MDBOOK_LINKCHECK_VERSION
ARG MDBOOK_INDEXING_VERSION
ARG MDBOOK_PANDOC_VERSION
ARG MDBOOK_UTILS_REV
ARG CARGO_NEXTEST_VERSION
ARG JUST_VERSION
ARG CARGO_FMT_TOML_VERSION
ARG DPRINT_VERSION
ARG RUMDL_VERSION
ARG MATURIN_VERSION
ARG FLUTTER_RUST_BRIDGE_CODEGEN_VERSION

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
    cargo binstall --no-confirm --secure --version ${CARGO_NEXTEST_VERSION} cargo-nextest
    cargo binstall --no-confirm --secure --version ${MATURIN_VERSION} maturin
    cargo binstall --no-confirm --secure --version ${MDBOOK_VERSION} mdbook
    cargo binstall --no-confirm --secure --version ${MDBOOK_LINKCHECK_VERSION} mdbook-linkcheck
    cargo binstall --no-confirm --secure --version ${MDBOOK_PANDOC_VERSION} mdbook-pandoc
    cargo binstall --no-confirm --secure --version ${JUST_VERSION} just
    cargo binstall --no-confirm --secure --version ${CARGO_FMT_TOML_VERSION} cargo-fmt-toml
    cargo binstall --no-confirm --secure --version ${DPRINT_VERSION} dprint
    cargo binstall --no-confirm --secure --version ${RUMDL_VERSION} rumdl
    cargo binstall --no-confirm --secure --version ${FLUTTER_RUST_BRIDGE_CODEGEN_VERSION} flutter_rust_bridge_codegen
    cargo binstall --no-confirm --secure sccache cargo-deny lychee
    cargo install --locked --force --version ${MDBOOK_INDEXING_VERSION} mdbook-indexing
EOF

## Build mdbook-utils from its pinned Git revision while it is still in development.
## Cargo's --rev keeps this binary aligned with the revision pinned in .devcontainer/.env.
RUN cargo install --git https://github.com/john-cd/mdbook-utils --rev ${MDBOOK_UTILS_REV} --locked mdbook-utils

## ------------------------------------------------------------------------------------------------

## Stage to build local tools (e.g., mdbook-scrub)
FROM rust-core AS local-tool-builder

WORKDIR /code

RUN mkdir -p /code/bin/

## Set target directory to a predictable path
ENV CARGO_TARGET_DIR=/code/target/tools/ \
    CARGO_BUILD_BUILD_DIR=/code/build/tools/

## Copy only the tools source to optimize caching
COPY tools/ /code/tools/

RUN --mount=type=secret,id=github_token,required=false \
    if [ -s /run/secrets/github_token ]; then \
        token="$(cat /run/secrets/github_token)"; \
        auth_header="$(printf 'x-access-token:%s' "$token" | base64 | tr -d '\r\n')"; \
        CARGO_NET_GIT_FETCH_WITH_CLI=true \
            GIT_CONFIG_COUNT=1 \
            GIT_CONFIG_KEY_0=http.https://github.com/.extraheader \
            GIT_CONFIG_VALUE_0="AUTHORIZATION: basic ${auth_header}" \
            cargo fetch --locked --manifest-path /code/tools/Cargo.toml; \
    else \
        cargo fetch --locked --manifest-path /code/tools/Cargo.toml; \
    fi

## Build repository tools in release mode with the pinned formatter toolchain.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    RUSTFMT_TOOLCHAIN="${RUSTFMT_TOOLCHAIN:-$(sed -n 's/^channel *= *"\([^"]*\)".*/\1/p' "$RUSTFMT_TOOLCHAIN_FILE")}" \
    && cargo +"$RUSTFMT_TOOLCHAIN" build --bins --locked --all-features --release -Z unstable-options --artifact-dir /code/bin --manifest-path /code/tools/Cargo.toml

# ------------------------------------------------------------------------------------------------

# Add the native libraries and external tools shared by the development and CI images.
FROM rust-core AS base

SHELL ["bash", "-c"]

ARG MDBOOK_UTILS_REV
ARG TECTONIC_VERSION
ARG TECTONIC_LINUX_SHA256
ARG TARGETARCH
ARG ORACLE_CLIENT_VERSION
ARG ORACLE_CLIENT_DOWNLOAD_ID
ARG ORACLE_CLIENT_LINUX_AMD64_BASICLITE_SHA256
ARG ORACLE_CLIENT_LINUX_AMD64_SDK_SHA256
ARG ORACLE_CLIENT_LINUX_ARM64_BASICLITE_SHA256
ARG ORACLE_CLIENT_LINUX_ARM64_SDK_SHA256
ENV MDBOOK_UTILS_REV=${MDBOOK_UTILS_REV}
ENV CARGO_TARGET_DIR=/code/target/ \
    CARGO_BUILD_BUILD_DIR=/code/build/

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
## `default-jdk` for Java/JNI examples; `ruby-dev` for Ruby FFI bindings
## `python3-venv` and `python-is-python3` for building the separate Maturin extension example
## `libgtk-4-dev` `build-essential` for `relm4` and `gtk4`
## `clang` `libwayland-dev` `libxkbcommon-x11-dev` `libvulkan-dev` for `xilem`
## `cmake build-essential libudev-dev xorg-dev libglu1-mesa-dev libasound2-dev libxkbcommon-dev` for `openrr`
## `diffutils gcc m4 make` for `gmp-mpfr-sys` / `rug`
## `ninja-build` for CMake-based `-sys` crates
## `pandoc` for `mdbook-pandoc`
## `git unzip xz-utils zip libglu1-mesa` for Flutter's Linux SDK prerequisites
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
    git \
    libasound2-dev \
    libayatana-appindicator3-dev \
    libc6-dev \
    libaio1t64 \
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
    default-jdk \
    locales \
    mold \
    openssl \
    llvm \
    m4 \
    make \
    nasm \
    ninja-build \
    pkg-config \
    protobuf-compiler \
    unzip \
    python3 \
    python-is-python3 \
    python3-venv \
    ruby-dev \
    unzip \
    wget \
    xz-utils \
    zip \
    xorg-dev \
    && nasm --version \
    && apt-get clean -y

## Install the hash-pinned Oracle Instant Client runtime and SDK for OCI examples.
COPY --link --chmod=755 .devcontainer/install_oracle_client.sh /usr/local/bin/install_oracle_client.sh
RUN ORACLE_CLIENT_ARCH="$TARGETARCH" \
    ORACLE_CLIENT_VERSION="$ORACLE_CLIENT_VERSION" \
    ORACLE_CLIENT_DOWNLOAD_ID="$ORACLE_CLIENT_DOWNLOAD_ID" \
    ORACLE_CLIENT_LINUX_AMD64_BASICLITE_SHA256="$ORACLE_CLIENT_LINUX_AMD64_BASICLITE_SHA256" \
    ORACLE_CLIENT_LINUX_AMD64_SDK_SHA256="$ORACLE_CLIENT_LINUX_AMD64_SDK_SHA256" \
    ORACLE_CLIENT_LINUX_ARM64_BASICLITE_SHA256="$ORACLE_CLIENT_LINUX_ARM64_BASICLITE_SHA256" \
    ORACLE_CLIENT_LINUX_ARM64_SDK_SHA256="$ORACLE_CLIENT_LINUX_ARM64_SDK_SHA256" \
    /usr/local/bin/install_oracle_client.sh /opt/oracle \
    && echo /opt/oracle/instantclient_23_26 > /etc/ld.so.conf.d/oracle-instantclient.conf \
    && ldconfig
ENV ORACLE_HOME=/opt/oracle/instantclient_23_26 \
    OCI_LIB_DIR=/opt/oracle/instantclient_23_26 \
    LD_LIBRARY_PATH=/opt/oracle/instantclient_23_26 \
    LIBRARY_PATH=/opt/oracle/instantclient_23_26

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
ARG PANDOC_VERSION
RUN wget -q "https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-1-amd64.deb" \
    -O /tmp/pandoc.deb \
    && dpkg -i /tmp/pandoc.deb \
    && rm /tmp/pandoc.deb

## Copy only binaries from builder to keep the base image lean
COPY --from=tool-builder /usr/local/cargo/bin/ /usr/local/cargo/bin/

## Prefer the pinned Tectonic release binary on Linux x86_64; use Cargo for other architectures.
RUN if [ "$TARGETARCH" = "amd64" ]; then \
        archive="/tmp/tectonic-${TECTONIC_VERSION}-x86_64-unknown-linux-gnu.tar.gz"; \
        curl --fail --location --proto '=https' --tlsv1.2 \
            "https://github.com/tectonic-typesetting/tectonic/releases/download/tectonic%40${TECTONIC_VERSION}/tectonic-${TECTONIC_VERSION}-x86_64-unknown-linux-gnu.tar.gz" \
            --output "$archive"; \
        echo "${TECTONIC_LINUX_SHA256}  ${archive}" | sha256sum --check; \
        tar -xzf "$archive" -C /usr/local/cargo/bin tectonic; \
        rm "$archive"; \
    else \
        cargo binstall --no-confirm --secure --version "${TECTONIC_VERSION}" tectonic; \
    fi

## ------------------------------------------------------------------------------------------------

## Stage for development / Dev Container, with additional tools and configurations for an optimal development experience.
FROM base AS development

ARG FLUTTER_VERSION
ARG FLUTTER_GIT_REVISION

COPY --link --chmod=755 .devcontainer/install_flutter_sdk.sh /usr/local/bin/install_flutter_sdk.sh
RUN install_flutter_sdk.sh "${FLUTTER_VERSION}" "${FLUTTER_GIT_REVISION}" /opt/flutter \
    && FLUTTER_ROOT=/opt/flutter CI=true /opt/flutter/bin/flutter --version --machine
ENV FLUTTER_ROOT=/opt/flutter
ENV PATH="/opt/flutter/bin:${PATH}"

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
# cargo install --locked cargo-outdated || true
# cargo-udeps
# cargo-audit
# cargo-pants
# cargo-sweep
# cargo-cache
## Optionally, install tool to convert TOML to JSON
# && cargo binstall --no-confirm toml2json \

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
# Compose build arguments override these fallback Git identity values.
ARG EMAIL=nobody@users.noreply.github.com
ARG GIT_AUTHOR_NAME=nemo
RUN git config --global user.email ${EMAIL} && git config --global user.name ${GIT_AUTHOR_NAME}

## For `mdbook serve --open`
EXPOSE 3000

## https://code.visualstudio.com/remote/advancedcontainers/start-processes
COPY --link --chmod=755 .devcontainer/dev.sh ./.devcontainer/
ENTRYPOINT ["./.devcontainer/dev.sh"]

## Sleep to keep the container running.
## The command is passed to `dev.sh`.
CMD ["sleep", "infinity"]

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
ENTRYPOINT ["./.devcontainer/ci.sh"]
