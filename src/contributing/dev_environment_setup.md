## Development Environment Setup

### Using VS Code

Clone the [repo][rust-howto-github]⮳ and open the folder in [VS Code][rust-in-vs-code]⮳. Edit `.devcontainer/.env` if needed. {{i:VS Code}} should prompt you to open the code in a `docker` container, which installs {{i:`mdbook`}} and {{i:rust tooling}} automatically. Make sure you have previously installed

- [Dev Container extension][dev-container-extension]⮳
- [Docker Desktop][docker-desktop]⮳ (or at least the Docker engine).

Note that opening the code folder in VS Code may take a little while the first time around.

### Other

If you are not using VS Code, install the [Dev Container CLI][dev-container-cli-github]⮳ or simply install the required tools on your local machine:

```bash
sudo apt-get update # or equivalent for other distros
sudo apt-get install fzf
sudo apt-get mold clang # if using
rustup update
rustup component add clippy
cargo install cargo-nextest
cargo install mdbook
cargo install just
cargo install mdbook-linkcheck
cargo install mdbook-utils
# for cargo +nightly fmt
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
```

You may need `sudo apt-get install libsqlite3-dev` on WSL.

Review `.devcontainer/Dockerfile` for other dependencies.

{{#include ../refs/link-refs.md}}
