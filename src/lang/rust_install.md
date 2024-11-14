# Rust Installation

{{#include rust_install.incl.md}}

[Install Rust][rust-install-rust]{{hi:Rust installation}}⮳
<div class="hidden">
TODO: header
</div>

## Key Steps {#rust-install}

- Install [Rustup][rustup-website]{{hi:rustup}}⮳

On WSL / Unix:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Check whether you have Rust installed correctly

```bash
rustc --version
cargo --version
```

- Open the documentation, if needed

```bash
rustup doc
```

- Create a new project

```bash
cargo new hello_world
cd hello_world
code . # open VS Code (or your favorite editor) and edit the code as you wish
```

- Build / run the code.

```bash
cargo check # check if the code can compile
cargo build # compile
cargo run # run the executable
```

`cargo run` builds the code if `cargo build` has not been invoked before or the code has changed.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO: expand
</div>
