
[Install Rust]( https://www.rust-lang.org/tools/install )


1. Install [Rustup]( https://rustup.rs/ ) 

On WSL / Unix: 

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

1. Check whether you have Rust installed correctly

```bash
rustc --version
```

1. Open the documentation

```bash
rustup doc
```

1. Create a new project

```bash
cargo new hello_world
cd hello_world
code .      # open VS Code and edit 
```

1. Build or run the code. 

```bash
cargo check
cargo build
cargo run
```





