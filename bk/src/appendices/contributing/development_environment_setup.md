<!-- TODO Reviewed ??-??-2026 -->
# Development Environment Setup

{{#include development_environment_setup.incl.md}}

## Develop with VS Code {#using-vs-code}

Clone the [repo][rust-howto~repo]↗ and open the folder in [VS Code][rust-in-vs-code~website]↗. Edit `.devcontainer/.env` if needed. VS Code{{hi:VS Code}} should prompt you to open the code in a [`docker`][docker~website]↗{{hi:docker}} container, which installs [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} and rust tooling{{hi:Rust tooling}} automatically. Make sure you have previously installed

- [Dev Container extension][dev-container-extension~website]↗{{hi:Dev Container}}.
- [Docker Desktop][docker~desktop~website]↗ (or at least the Docker engine).

Note that opening the code folder in VS Code may take a little while the first time around.

### Debug Rust Programs on Windows

Some debugger configurations can make allocation-heavy Rust programs run unusually slowly on Windows. The article [Fixing Rust memory allocation slowdown in VS Code on Windows][blog~vs-code-debug-heap]↗ explains the Windows debug heap and shows a CodeLLDB launch setting that disables it. The workaround depends on the debugger engine, so check that it applies to your VS Code configuration.

## Develop with Other IDEs {#other}

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
cargo install --git https://github.com/john-cd/mdbook-utils --rev dedddf3699e461cd418c1a044d82aabd3fec62e0 --locked --force mdbook-utils
# for `cargo +nightly-2026-10-22 fmt`
rustup toolchain install nightly-2026-10-22
rustup component add rustfmt --toolchain nightly-2026-10-22
```

You may need `sudo apt-get install libsqlite3-dev` on WSL.

Review [`.devcontainer/Dockerfile`][rust-howto~Dockerfile~repo]↗ for other dependencies.

## Install on Windows {#windows-install}

On Windows, use [`winget`][winget~website]↗{{hi:winget}} (Windows Package Manager) to install the required tools:

```powershell
winget install Rustlang.Rustup
winget install Microsoft.VisualStudioCode
winget install Docker.DockerDesktop
winget install Casey.Just
winget install Python.Python.3
winget install openssl  # Required by crates that depend on native TLS (e.g. `openssl-sys`)
```

After installing Rust via `rustup`, open a new terminal and run:

```powershell
rustup component add clippy
cargo install cargo-nextest
cargo install mdbook
cargo install mdbook-linkcheck
cargo install --git https://github.com/john-cd/mdbook-utils --rev dedddf3699e461cd418c1a044d82aabd3fec62e0 --locked --force mdbook-utils
rustup toolchain install nightly-2026-10-22
rustup component add rustfmt --toolchain nightly-2026-10-22
```

> Note: On Windows, you may also need the [Microsoft C++ Build Tools][msvc~website]↗{{hi:MSVC}} or the Visual Studio C++ workload for compiling some crates.

## Install `just` (Alternative) {#alternative-just-install}

[`just`][c~just~docs]↗{{hi:just}}

```sh
sudo apt update
apt-get -y install just
```

## Install `mdbook` (Alternative) {#alternative-mdbook-install}

[`mdbook`][c~mdbook~docs]↗{{hi:mdbook}}

```dockerfile
# RUN <<EOF
# set -e
# wget -c <https://github.com/rust-lang/mdBook/releases/download/v0.5.4/mdbook-v0.5.4-x86_64-unknown-linux-gnu.tar.gz> -O - | sudo tar -xvz -C /usr/local/bin
# EOF
```

## Related Topics {#related-topics .skip}

- [[containers | Containers]].
- [[development_tools | Development Tools]].
- [[rust-installation | Rust Installation]].
- [[text-editors | Text Editors]].
- [[vscode | Vscode]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
