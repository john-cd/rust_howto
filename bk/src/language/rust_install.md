# Rust Installation

{{#include rust_install.incl.md}}

## Install Rust and Create a First Project {#rust-install}

First, install [Rustup][rustup-website]{{hi:rustup}}⮳.

On Linux, macOS, or other Unix-like systems (including Windows Subsystem for Linux - WSL), open your terminal and run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This command downloads a script and starts the rustup installation. Follow the on-screen prompts.

After installation, rustup might ask you to configure your current shell. You can either restart your terminal or run the command provided by the installer (usually something like `source $HOME/.cargo/env` on Unix-like systems).

On Windows, it's recommended to visit the official Rust website's [installation page](https://www.rust-lang.org/tools/install) and download the `rustup-init.exe` installer. Running this executable will guide you through the installation. You may also need the C++ build tools for Visual Studio, which the installer can help you set up.

Second, check whether you have Rust installed correctly. Open a new terminal window and check if Rust is installed correctly by running:

```bash
rustc --version
cargo --version
```

These commands should print the installed versions of the Rust compiler (`rustc`) and the Rust package manager and build tool (`cargo`).

Third, open the documentation, if needed. You can open the locally installed Rust documentation in your web browser using:

```bash
rustup doc
```

Fourth, you can start creating projects using `cargo new` and build/run them with `cargo build` and `cargo run`.

- Create a new project.

```bash
cargo new hello_world
cd hello_world
code . # open VS Code (or your favorite editor) and edit the code as you wish.
```

- Build / run the code.

```bash
cargo check # check if the code can compile.
cargo build # compile.
cargo run # run the executable.
```

`cargo run` builds the code if `cargo build` has not been invoked before or the code has changed.

## References {#skip}

- [Install Rust][rust-install-rust]{{hi:Rust installation}}⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[rust_install NOW](https://github.com/john-cd/rust_howto/issues/556)
</div>
