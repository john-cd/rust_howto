## Installation

### Using VS Code

Clone the repo and open the folder in [VS Code][rust-in-vs-code]. Edit `.devcontainer/.env` if needed. VS Code should prompt you to open the code in a `docker` container, which installs `mdbook` and rust tooling automatically. Make sure you have previously installed

- [Dev Container extension][dev-container-extension]
- [Docker Desktop][docker-desktop] (or at least the Docker engine).

Note that opening the code folder in VS Code may take a little while the first time around.

### Other

If you are not using VS Code, install the [Dev Container CLI][dev-container-cli-github] or simply install the required tools on your local machine:

```bash
sudo apt-get update # or equivalent for other distros
sudo apt-get install fzf mold clang
rustup update
rustup component add clippy
rustup component add rustfmt
cargo install cargo-nextest
cargo install mdbook
cargo install just
cargo install mdbook-linkcheck
# for cargo +nightly fmt
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
```

You may need `sudo apt-get install libsqlite3-dev` on WSL.

Review `.devcontainer/Dockerfile` for other dependencies.

{{#include ../refs/link-refs.md}}
