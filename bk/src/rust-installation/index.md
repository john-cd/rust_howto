<!-- TODO Reviewed ??-??-2026 -->

# Rust Installation

{{#include index.incl.md}}

## Install Rust and Create a First Project

First, install [Rustup][rustup~website]↗{{hi:rustup}}. On Linux, macOS, or other Unix-like systems (including Windows Subsystem for Linux - WSL), open your terminal and run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This command downloads a script and starts the `rustup` installation. Follow the on-screen prompts.

All tools are installed to the `~/.cargo/bin` directory by default. After installation, `rustup` might ask you to configure your current shell (to update your `PATH`). You can either restart your terminal or run the command provided by the installer (usually something like `source $HOME/.cargo/env` on Unix-like systems).

On Windows, it's recommended to visit the official Rust website's [installation page][rust-lang~install-rust]↗ and download the `rustup-init.exe` installer. Running this executable will guide you through the installation. You may also need the C++ build tools for Visual Studio, which the installer can help you set up.

If you've installed `rustup` in the past, you can update your installation by running `rustup update`.

Second, check whether you have Rust installed correctly. Open a new terminal window and check if Rust is installed correctly by running:

```bash
rustc --version
cargo --version
```

These commands should print the installed versions of the Rust compiler (`rustc`) and the Rust package manager and build tool (`cargo`).

Third, open the Rust documentation, if needed. You can open the locally installed Rust documentation in your web browser using:

```bash
rustup doc
```

Fourth, start creating projects using `cargo new` and build/run them with `cargo build` and `cargo run`.

- Create a new project:

```bash
cargo new hello_world # Create a minimal new project.
cd hello_world
code . # Open VS Code (or your favorite editor).
```

Edit the code as you wish. `cargo new` creates a minimal `main.rs` file in the `src` subdirectory. You should also review the `Cargo.toml` package configuration file and add dependencies with the `cargo add` command:

```sh
cargo add anyhow # Add `anyhow` to Cargo.toml as a dependency.
```

- Build / run the code:

```bash
cargo check # Check if the code can compile.
cargo build # Compile.
cargo run   # Run the executable.
```

`cargo run` builds the code if `cargo build` has not been invoked before or if the code has changed.

For more details, peruse the [Getting Started][book~rust~ch01-00-getting-started]↗ chapter of the Rust book.

Fifth, consider adding your project to source control. When `cargo new` creates a project outside an existing version-control repository, it initializes Git and adds a `.gitignore` that excludes Cargo's `target/` directory by default. It does not create a nested repository when the project is already inside one. You can choose this behavior explicitly with `--vcs git` or disable it with `--vcs none`. If you disabled Git initialization, initialize the repository yourself and ensure `target/` is ignored:

```txt
target/
```

Once you have edited your code, stage and commit the changes with:

```sh
git add .
git commit -m "commit message"
```

Review the [git][git~website]↗ manual for more commands.

## References

- [Install Rust][rust-lang~install-rust]↗{{hi:Rust installation}}.

## Related Topics

- [[cargo |`cargo`]].
- [[entry_points | Entrypoints]].
- [[language | Language]].
- [[package_layout | Package layout]].
- [[rustup | `rustup`]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
  accuracy review: introduction and language basics
</div>
