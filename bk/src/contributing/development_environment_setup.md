# Development Environment Setup

{{#include development_environment_setup.incl.md}}

## Using VS Code {#using-vs-code}

Clone the [repo][rust-howto~repo]↗ and open the folder in [VS Code][rust-in-vs-code~website]↗. Edit `.devcontainer/.env` if needed. VS Code{{hi:VS Code}} should prompt you to open the code in a [`docker`][docker~website]↗{{hi:docker}} container, which installs [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} and rust tooling{{hi:Rust tooling}} automatically. Make sure you have previously installed

- [Dev Container extension][dev-container-extension~website]↗{{hi:Dev Container}}.
- [Docker Desktop][docker~desktop~website]↗ (or at least the Docker engine).

Note that opening the code folder in VS Code may take a little while the first time around.

## Other {#other}

If you are not using VS Code, install the [Dev Container CLI][dev-container-cli~repo]↗ or simply install the required tools on your local machine:

```bash
sudo apt-get update # Or equivalent for other distros
# sudo apt-get install fzf # Optional
# sudo apt-get mold clang # If using
rustup update
rustup component add clippy
cargo install cargo-nextest
cargo install mdbook
cargo install just
cargo install mdbook-linkcheck
cargo install mdbook-utils
# for `cargo +nightly fmt`
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
```

You may need `sudo apt-get install libsqlite3-dev` on WSL.

Review [`.devcontainer/Dockerfile`][rust-howto~Dockerfile~repo]↗ for other dependencies.

## Alternative `just` Install {#alternative-just-install}

[`just`][c~just~docs]↗{{hi:just}}

```sh
sudo apt update
apt-get -y install just
```

## Alternative `mdbook` Install {#alternative-mdbook-install}

[`mdbook`][c~mdbook~docs]↗{{hi:mdbook}}

```dockerfile
# RUN <<EOF
# set -e
# wget -c <https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz> -O - | sudo tar -xvz -C /usr/local/bin
# EOF
```

## Related Topics {#related-topics .skip}

- [[rust-installation | Rust Installation]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[dev_environment_setup: review; write windows install; `winget install openssl`; need Python](https://github.com/john-cd/rust_howto/issues/527)

- [[containers | Containers]].
- [[development_tools | Development Tools]].
- [[text-editors | Text Editors]].
- [[vscode | Vscode]].

</div>
