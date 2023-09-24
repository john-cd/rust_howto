# Cargo (package manager) and tools

[The Cargo book]( https://doc.rust-lang.org/cargo/index.html )


`cargo help` or `cargo <command> --help`

```sh
cargo --version
cargo new hello_cargo   # Create a new project. Can add --bin  or --lib
cargo build             # Creates an executable file in target/debug/hello_cargo
cargo build --release
cargo run               # Build and run a project in one step 
cargo run -- arg1 somefile.txt > output.txt # pass arguments to the program and collect output
cargo check             # Quickly checks your code to make sure it compiles but doesn’t produce an executable
cargo clean             # Removes build artifacts
cargo test              # Looks for tests to run in two places: in each of your src files and any tests in tests/. 
cargo update            # Updates all dependencies - respect the SemVer constraints in cargo.toml
cargo update -p regex   # Updates just “regex”
```


## `Cargo.toml` and lock files

```toml
# configuring a package
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
time = "0.1.12"   # equivalent to ^1.2.3 SemVer version range - e.g. `cargo update -p time` should update to version 0.1.13 if it is the latest 0.1.z release, but would not update to 0.2.0
regex = { git = "https://github.com/rust-lang/regex.git" }
hello_utils = { path = "hello_utils", version = "0.1.0" }                      # Reference a sub-crate - points to folder hello_utils inside of which a `Cargo.toml` and `src` folder
```

Examples of version requirements and the versions that would be allowed with them:

```
1.2.3  :=  >=1.2.3, <2.0.0
1.2    :=  >=1.2.0, <2.0.0
1      :=  >=1.0.0, <2.0.0
0.2.3  :=  >=0.2.3, <0.3.0
0.2    :=  >=0.2.0, <0.3.0
0.0.3  :=  >=0.0.3, <0.0.4
0.0    :=  >=0.0.0, <0.1.0
0      :=  >=0.0.0, <1.0.0
```

Details in <https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html>

If you’re building a non-end product, such as a rust library that other rust packages will depend on, put `Cargo.lock` in your `.gitignore`. If you’re building an end product, which are executable like command-line tool or an application, or a system library with crate-type of staticlib or cdylib, check `Cargo.lock` into git. 


```sh
cargo install cargo-edit   # install if needed
cargo add actix-web@4.0.0  # add dependencies to Cargo.toml from the command line
```

## cargo-watch

```sh
cargo install cargo-watch
cargo watch -x check                # Run cargo check after every code change
cargo watch -x check -x test -x run # Run cargo check after code changes. If it succeeds, it launches cargo test. If tests pass, it launches the application with cargo run.
```


## Formatting

```sh
rustup component add rustfmt # install if needed
cargo fmt
cargo fmt -- --check # fails if code is not formatted, use in CD / CI
```


## Linting

```sh
rustup component add clippy # install if needed
cargo clippy   
```

Mute a warning using the #[allow(clippy::lint_name)] attributes


## Fix

Can automatically fix compiler warnings that have a clear way to correct the problem that’s likely what you want.

```sh
cargo fix 
```


## Code coverage

[Tarpaulin]( https://github.com/xd009642/tarpaulin )


## Security audit

```sh
cargo install cargo-audit
cargo audit
```


## Unused dependencies

[udeps]( https://github.com/est31/cargo-udeps )

or (simpler) [Machete]( https://blog.benj.me/2022/04/27/cargo-machete/ )

```sh
cargo install cargo-machete
cargo machete
```


## Templates

[Cargo Generate](https://cargo-generate.github.io/cargo-generate/index.html): developer tool to help you get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.