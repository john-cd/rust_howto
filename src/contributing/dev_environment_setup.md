# Development Environment Setup {#dev-env-setup}

{{#include dev_environment_setup.incl.md}}

## Using VS Code {#using-vs-code}

Clone the [repo][rust-howto-github]⮳ and open the folder in [VS Code][rust-in-vs-code]⮳. Edit `.devcontainer/.env` if needed. VS Code{{hi:VS code}} should prompt you to open the code in a [`docker`][docker-website]{{hi:docker}}⮳ container, which installs [`mdbook`][c-mdbook]{{hi:mdbook}}⮳ and rust tooling{{hi:Rust tooling}} automatically. Make sure you have previously installed

- [Dev Container extension][dev-container-extension]{{hi:Dev Containers}}⮳
- [Docker Desktop][docker-desktop-website]]⮳ (or at least the Docker engine).

Note that opening the code folder in VS Code may take a little while the first time around.

## Other {#other}

If you are not using VS Code, install the [Dev Container CLI][dev-container-cli-github]⮳ or simply install the required tools on your local machine:

```bash
sudo apt-get update # or equivalent for other distros
# sudo apt-get install fzf # optional
# sudo apt-get mold clang # if using
rustup update
rustup component add clippy
cargo install cargo-nextest
cargo install mdbook
cargo install just
cargo install mdbook_linkcheck
cargo install mdbook-utils
# for cargo +nightly fmt
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
```

You may need `sudo apt-get install libsqlite3-dev` on WSL.

Review `.devcontainer/Dockerfile` for other dependencies.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO P1: review

[dev_environment_setup: windows install (P1)](https://github.com/john-cd/rust_howto/issues/528)

`winget install openssl`
need Python

## Alternative `just` install {#skip1}

```dockerfile
# RUN <<EOF
# set -e
# wget -qO - '<https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub>' | gpg --dearmor | sudo tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null
# echo "deb [arch=all,$(dpkg --print-architecture) signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr $(lsb_release -cs)" | sudo tee /etc/apt/sources.list.d/prebuilt-mpr.list
# sudo apt update
# apt-get -y install just
# EOF
```

## Alternative `mdbook` install {#skip2}

```dockerfile
# RUN <<EOF
# set -e
# wget -c <https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz> -O - | sudo tar -xvz -C /usr/local/bin
# EOF
```

</div>
