<!-- TODO Reviewed ??-??-2026 -->

# Development Environment Setup

{{#include development_environment_setup.incl.md}}

## Validation Environments {#validation-environments}

Run each check from its owning workspace. A successful build on one operating system does not validate code behind another platform's `cfg` condition.

| Validation target                                                                  | Environment                                                                                                    |
| ---------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| Formatting, linting, tests, and book checks                                        | Windows, Linux, or macOS, subject to the workspace's native dependencies. Use the workspace's `just` recipes.  |
| Linux-specific examples such as the `ggez` and `Relm4` paths                       | Linux with the required system development libraries. A Windows fallback does not compile the Linux-only code. |
| Example bodies excluded by Windows `cfg` conditions, including Tauri and WebRender | A platform where those bodies are enabled, with their native dependencies installed.                           |
| Windows API examples and MSVC build or link behavior                               | Native Windows with the required Windows SDK and MSVC tools.                                                   |
| Tests that connect to PostgreSQL, Cassandra, Oracle, or another service            | A reachable service with its required credentials and native client libraries.                                 |
| Interactive [GUI][cat~gui]↗{{hi:gui}} or speech tests                              | A desktop session with a window system or the required speech backend.                                         |

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
# sudo apt-get install mold clang # If using
rustup update
rustup component add clippy
cargo install cargo-nextest --version 0.9.148 --locked
cargo install mdbook --version 0.5.4 --locked
cargo install just
cargo install mdbook-linkcheck --version 0.7.7 --locked
cargo install --git https://github.com/john-cd/mdbook-utils --rev 171a98f93a6d3817561c8dd9f8075a9dcf6629f3 --locked --force mdbook-utils
# for the formatter toolchain pinned in `rustfmt-toolchain.toml`
RUSTFMT_TOOLCHAIN="$(sed -n 's/^channel *= *"\([^"]*\)".*/\1/p' rustfmt-toolchain.toml)"
rustup toolchain install "$RUSTFMT_TOOLCHAIN"
rustup component add rustfmt --toolchain "$RUSTFMT_TOOLCHAIN"
```

Install the pinned spell checker with `cargo install typos-cli --version 1.51.1 --locked` on Windows, Linux, or macOS.

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
winget install NASM.NASM  # Required to build `aws-lc-sys` (used by `rustls` / `aws-lc-rs`) on Windows
winget install Ninja-build.Ninja  # Build backend for CMake-based `-sys` crates
```

`winget` places Ninja on your `PATH` (via `%LOCALAPPDATA%\Microsoft\WinGet\Links`). The NASM installer does not always add itself to `PATH`: if `nasm --version` fails in a new terminal, add the install folder (for example `%LOCALAPPDATA%\bin\NASM` or `C:\Program Files\NASM`) to your user `PATH`, then restart the terminal. Without NASM, `aws-lc-sys` fails to build with an error that `nasm` was not found. On Linux and macOS the build uses the system C compiler, so NASM is not needed; install Ninja with `sudo apt-get install ninja-build` or `brew install ninja` if a CMake-based crate asks for it.

After installing Rust via `rustup`, open a new terminal and run:

```powershell
rustup component add clippy
cargo install cargo-nextest --version 0.9.148 --locked
cargo install mdbook --version 0.5.4 --locked
cargo install mdbook-linkcheck --version 0.7.7 --locked
cargo install --git https://github.com/john-cd/mdbook-utils --rev 171a98f93a6d3817561c8dd9f8075a9dcf6629f3 --locked --force mdbook-utils
$rustfmtToolchain = (Select-String -Path rustfmt-toolchain.toml -Pattern '^channel\s*=\s*"([^"]+)"').Matches[0].Groups[1].Value
rustup toolchain install $rustfmtToolchain
rustup component add rustfmt --toolchain $rustfmtToolchain
```

> Note: On Windows, you may also need the [Microsoft C++ Build Tools][msvc~website]↗{{hi:MSVC}} or the Visual Studio C++ workload for compiling some crates.

### Native Windows and book recipe support {#windows-book-recipes}

The book workspaces and many `just` recipes run natively on Windows. The root `bk/justfile` uses PowerShell 7. Some imported modules use PowerShell, while others launch Git Bash explicitly; each module declares its own Windows shell. Recipes marked `[unix]` are unavailable on Windows, and a `[windows]` recipe provides the Windows implementation. The `bk/scripts/README.md` support table records the current module boundaries.

The pinned `just` version is defined by `JUST_VERSION` in `.devcontainer/.env`; use the repository's setup script or install that version explicitly. From `bk/`, `just bb`, `just s`, `just spell`, and `just lnk <pattern>` are available on Windows. The faster `just q` preview also has a Windows recipe and requires Git Bash. `just indices` exposes the Windows crate-index commands; the Bash-based index generators remain Unix-only. Native Windows API examples and tests still need a Windows host, and Linux-only code paths need Linux with their system dependencies.

## Format Book Markdown {#format-book-markdown}

Install [`dprint`][dprint~website] and use the `mdfmt` recipes from the `bk` directory to format or check book Markdown. The repository pins the Markdown plugin in `dprint.json` and retains authored line wrapping, so the formatter preserves long tables, mdBook directives, heading anchors, and fenced code examples.

```sh
cd bk
just mdfmt drafts/categories/development-tools_debugging/debugging.md
just mdfmt-check drafts/categories/development-tools_debugging/debugging.md
```

Pass a path under `src`, `drafts`, or `../later/src` to format or check that file. The root configuration also excludes generated indices and build output.

## Lint Markdown {#lint-book-markdown}

Install [`rumdl`][rumdl~repo] version 0.3.0 with `cargo install rumdl --version 0.3.0 --locked`. From `bk/`, run `just lint-md` to lint `src`, `drafts`, and `../later/src` on Windows, Linux, or macOS. This check keeps markdownlint installed and applies its rule choices from `.markdownlint.yaml` to all three paths; the config is passed explicitly because `later/src` is outside the `bk/` directory tree. The generated `src/refs/dependency-refs.md` fragment is excluded because its intentional blank line between each reference triggers MD012 thousands of times; `just book refdefs-check` verifies that generated file instead. Include-only `refs.incl.md` fragments locally disable MD057 because their relative links are resolved from the pages that include them.

The Markdownlint MD040 `language_only: false` setting is omitted because it is markdownlint's default and rumdl's compatibility loader does not support that redundant option.

## Check spelling {#check-spelling}

Install [`typos-cli`][typos~repo] version 1.51.1 with `cargo install typos-cli --version 1.51.1 --locked`. From `bk/`, run `just spell` to check the published book, drafts, and future chapters on Windows, Linux, or macOS. The root `_typos.toml` records reviewed project names and excludes generated reference lists.

When Typos flags a correct project term, verify it in context and add it to `[default.extend-words]` in `_typos.toml` as `word = "word"`. Do not add a term before reviewing the reported location.

The optional Unix-only `just spelling spell-aspell` recipe remains available for interactive Aspell checks; the cross-platform `just spell` recipe uses Typos.

## Install `just` (Alternative) {#alternative-just-install}

[`just`][c~just~docs]↗{{hi:just}}

```sh
sudo apt update
sudo apt-get -y install just
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

<div class="hidden">
  accuracy review: contributing guide
</div>
