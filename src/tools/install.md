
# [Install Rust]( https://www.rust-lang.org/tools/install )

## Key Steps

- Install [Rustup]( https://rustup.rs/ )

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
code .      # open VS Code and edit 
```

- Build / run the code.

```bash
cargo check
cargo build
cargo run
```
