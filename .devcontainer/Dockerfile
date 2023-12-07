FROM mcr.microsoft.com/devcontainers/rust:bullseye as final

SHELL ["bash", "-c"]

# Prereqs to install Just: https://just.systems/man/en/chapter_4.html
RUN <<EOF
  wget -qO - 'https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub' | gpg --dearmor | sudo tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null
  echo "deb [arch=all,$(dpkg --print-architecture) signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr $(lsb_release -cs)" | sudo tee /etc/apt/sources.list.d/prebuilt-mpr.list
  sudo apt update
EOF

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install just wget \
    && apt-get autoremove -y && apt-get clean -y

RUN --mount=type=cache,target=/usr/local/cargo/registry/ \
#    cargo install mdbook && \
    cargo install mdbook-hide

# Install mdbook from the binary release
RUN <<EOF
set -e
wget -c https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz -O - | sudo tar -xvz -C /usr/local/bin
EOF

EXPOSE 3000

CMD sleep infinity
