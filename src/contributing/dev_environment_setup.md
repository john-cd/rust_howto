# Development Environment Setup {#dev-env-setup}

{{#include dev_environment_setup.incl.md}}

## Using VS Code {#using-vs-code}

Clone the [repo][rust-howto-github]⮳ and open the folder in [VS Code][rust-in-vs-code]⮳. Edit `.devcontainer/.env` if needed. VS Code{{hi:VS code}} should prompt you to open the code in a [`docker`][docker-website]{{hi:docker}}⮳ container, which installs [`mdbook`][c-mdbook-documentation]{{hi:mdbook}}⮳ and rust tooling{{hi:Rust tooling}} automatically. Make sure you have previously installed

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
TODO: review

TODO windows install

`winget install openssl`
need Python
</div>
