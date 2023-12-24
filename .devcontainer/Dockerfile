FROM mcr.microsoft.com/devcontainers/rust:bookworm as final

SHELL ["bash", "-c"]

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install wget fzf lld clang \
    && apt-get autoremove -y && apt-get clean -y

## Install fmt and clippy
RUN rustup update; rustup component add clippy; rustup component add rustfmt

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

# Install mdbook from the binary release
RUN cargo binstall --no-confirm mdbook
## Alternatively
# RUN <<EOF
# set -e
# wget -c https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz -O - | sudo tar -xvz -C /usr/local/bin
# EOF

# Install mdbook-hide and keeper (note the workaround)
RUN --mount=type=cache,target=/usr/local/cargo/registry/ \
     cargo install mdbook-hide; \
     cargo install mdbook-keeper --git https://github.com/tfpk/mdbook-keeper.git

ARG WORK_DIR
WORKDIR ${WORK_DIR}

VOLUME /usr/local/cargo/registry/
# See .cargo/config.toml
VOLUME /cargo-target-rust_howto/

EXPOSE 3000

# Build stub project required by mdbook-keeper
CMD cargo build; sleep infinity
