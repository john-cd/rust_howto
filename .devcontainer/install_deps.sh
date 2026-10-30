#!/usr/bin/env bash
# Purpose: Install the operating-system and Rust tools required by the development container.
# Usage: Run with bash .devcontainer/install_deps.sh from the repository root; no arguments are required.
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

# Directly install on Ubuntu all required dependencies to compile the book's code.
#
# Essentially a copy of the Dockerfile.

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# Keep Linux developer installs aligned with the versions used by Docker and CI.
set -a
# shellcheck source=.env
source "$repo_root/.devcontainer/.env"
set +a
RUSTFMT_TOOLCHAIN="${RUSTFMT_TOOLCHAIN:-$(sed -n 's/^channel *= *"\([^"]*\)".*/\1/p' "$repo_root/rustfmt-toolchain.toml")}"

# GTK4 and pkg-config are required for the Rust GUI examples that depend on gtk4/relm4.
sudo apt-get update \
&& export DEBIAN_FRONTEND=noninteractive \
&& sudo apt-get install -y --no-install-recommends \
    aspell \
    aspell-en \
    autoconf \
    bison \
    build-essential \
    capnproto \
    clang \
    cmake \
    curl \
    default-jdk \
    diffutils \
    file \
    gcc \
    git \
    libasound2-dev \
    libayatana-appindicator3-dev \
    libc6-dev \
    libaio1t64 \
    libclang-dev \
    libffi-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    libgdbm-compat-dev \
    libgdbm-dev \
    libgmp-dev \
    libglu1-mesa-dev \
    libgraphite2-dev \
    libharfbuzz-dev \
    libgtk-4-dev \
    libicu-dev \
    libncurses-dev \
    libopencv-dev \
    libpng-dev \
    libpython3-dev \
    libreadline-dev \
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
    libyaml-dev \
    mold \
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
    jq \
    fzf \
    zlib1g-dev \
&& nasm --version \
&& sudo apt-get clean -y

# Install the pinned Oracle Instant Client Basic Light and SDK for OCI examples.
ORACLE_CLIENT_ARCH="$(uname -m)" "$repo_root/.devcontainer/install_oracle_client.sh" "$HOME/.local/opt/oracle"
oracle_client_major="${ORACLE_CLIENT_VERSION%%.*}"
oracle_client_version_tail="${ORACLE_CLIENT_VERSION#*.}"
oracle_client_minor="${oracle_client_version_tail%%.*}"
oracle_client_dir="$HOME/.local/opt/oracle/instantclient_${oracle_client_major}_${oracle_client_minor}"
oracle_client_env="$HOME/.local/opt/oracle/rust_howto-env.sh"
{
    printf 'export ORACLE_HOME="%s"\n' "$oracle_client_dir"
    printf 'export OCI_LIB_DIR="%s"\n' "$oracle_client_dir"
    printf 'export LD_LIBRARY_PATH="%s:${LD_LIBRARY_PATH:-}"\n' "$oracle_client_dir"
    printf 'export LIBRARY_PATH="%s:${LIBRARY_PATH:-}"\n' "$oracle_client_dir"
} > "$oracle_client_env"
oracle_profile_line='source "$HOME/.local/opt/oracle/rust_howto-env.sh"'
touch "$HOME/.profile"
if ! grep -Fqx "$oracle_profile_line" "$HOME/.profile"; then
    printf '%s\n' "$oracle_profile_line" >> "$HOME/.profile"
fi
source "$oracle_client_env"

# Install the pinned Ruby 4.0 runtime, headers, and shared library for Rutie.
"$repo_root/.devcontainer/install_ruby_4_0.sh"
export RUBY=/usr/local/bin/ruby
ruby_profile_line='export RUBY=/usr/local/bin/ruby'
touch "$HOME/.profile"
if ! grep -Fqx "$ruby_profile_line" "$HOME/.profile"; then
    printf '%s\n' "$ruby_profile_line" >> "$HOME/.profile"
fi

# Install Pandoc binary
wget -q "https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-1-amd64.deb" -O /tmp/pandoc.deb \
    && sudo dpkg -i /tmp/pandoc.deb \
    && rm /tmp/pandoc.deb

# Rust install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup update && rustup component add clippy rustfmt
rustup toolchain install "$RUSTFMT_TOOLCHAIN" \
    && rustup component add rustfmt clippy --toolchain "$RUSTFMT_TOOLCHAIN"

## binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

cargo binstall --no-confirm --secure --version "$CARGO_NEXTEST_VERSION" cargo-nextest
cargo binstall --no-confirm --secure --version "$MATURIN_VERSION" maturin
cargo binstall --no-confirm --secure --version "$MDBOOK_VERSION" mdbook
cargo binstall --no-confirm --secure --version "$MDBOOK_LINKCHECK_VERSION" mdbook-linkcheck
cargo binstall --no-confirm --secure --version "$MDBOOK_PANDOC_VERSION" mdbook-pandoc
cargo binstall --no-confirm --secure --version "$JUST_VERSION" just
cargo binstall --no-confirm --secure --force --version "$CARGO_FMT_TOML_VERSION" cargo-fmt-toml
cargo binstall --no-confirm --secure --force --version "$DPRINT_VERSION" dprint
cargo binstall --no-confirm --secure --force --version "$RUMDL_VERSION" rumdl
cargo binstall --no-confirm --secure --force --version "$PREK_VERSION" prek
cargo binstall --no-confirm --secure --version "$FLUTTER_RUST_BRIDGE_CODEGEN_VERSION" flutter_rust_bridge_codegen
cargo binstall --no-confirm --secure sccache cargo-deny lychee bacon ripgrep

flutter_install_path="$HOME/.local/share/flutter/$FLUTTER_VERSION"
bash "$repo_root/.devcontainer/install_flutter_sdk.sh" \
    "$FLUTTER_VERSION" "$FLUTTER_GIT_REVISION" "$flutter_install_path"
export FLUTTER_ROOT="$flutter_install_path"
export PATH="$FLUTTER_ROOT/bin:$PATH"

profile_file="$HOME/.bashrc"
profile_temp="$(mktemp "${profile_file}.XXXXXXXX")"
profile_start="# >>> rust_howto Flutter SDK >>>"
profile_end="# <<< rust_howto Flutter SDK <<<"
if [[ -f "$profile_file" ]] && ! awk -v start="$profile_start" -v end="$profile_end" '
    $0 == start { if (inside) exit 2; inside = 1; next }
    $0 == end { if (!inside) exit 2; inside = 0; next }
    !inside { print }
    END { if (inside) exit 2 }
' "$profile_file" > "$profile_temp"; then
    rm -f -- "$profile_temp"
    echo "The rust_howto Flutter SDK block is malformed in $profile_file" >&2
    exit 1
fi
{
    printf '\n%s\n' "$profile_start"
    printf 'export FLUTTER_ROOT="%s"\n' "$flutter_install_path"
    printf 'export PATH="${FLUTTER_ROOT}/bin:${PATH}"\n'
    printf '%s\n' "$profile_end"
} >> "$profile_temp"
if [[ -f "$profile_file" ]]; then
    chmod --reference="$profile_file" "$profile_temp"
else
    chmod 644 "$profile_temp"
fi
mv -f -- "$profile_temp" "$profile_file"

if [[ "$(uname -m)" == "x86_64" ]]; then
    tectonic_archive="$(mktemp)"
    curl --fail --location --proto '=https' --tlsv1.2 \
        "https://github.com/tectonic-typesetting/tectonic/releases/download/tectonic%40${TECTONIC_VERSION}/tectonic-${TECTONIC_VERSION}-x86_64-unknown-linux-gnu.tar.gz" \
        --output "$tectonic_archive"
    echo "${TECTONIC_LINUX_SHA256}  ${tectonic_archive}" | sha256sum --check
    tar -xzf "$tectonic_archive" -C "$HOME/.cargo/bin" tectonic
    chmod +x "$HOME/.cargo/bin/tectonic"
    rm -f "$tectonic_archive"
else
    cargo binstall --no-confirm --secure --version "$TECTONIC_VERSION" tectonic
fi

cargo install --locked --force --version "$MDBOOK_INDEXING_VERSION" mdbook-indexing
cargo install --git https://github.com/john-cd/mdbook-utils --rev "$MDBOOK_UTILS_REV" --locked --force mdbook-utils

# GitHub
curl -sS https://webi.sh/gh | sh

cargo binstall --no-confirm --secure kani-verifier
tectonic --version
nasm --version
flutter --version
flutter_rust_bridge_codegen --version
cargo fmt-toml --version
dprint --version
rumdl --version
