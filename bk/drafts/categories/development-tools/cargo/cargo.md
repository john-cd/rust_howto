# Cargo, the Rust Package Manager

{{#include cargo.incl.md}}

## Basic `cargo` Usage {#basic-cargo-usage}

[The Cargo book][c~cargo~book]⮳ [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}{{hi:cargo}}

[`cargo`][c~cargo~docs]⮳{{hi:cargo}}

`cargo help` or `cargo <command> --help`

```sh
cargo --version

# Create a new project. You can add `--bin` or `--lib`
cargo new hello_cargo

# Creates an executable file in `target/debug/hello_cargo`
cargo build
cargo build --release

# Build and run a project in one step
cargo run

# Pass arguments to the program and collect output
cargo run -- arg1 somefile.txt > output.txt

# Quickly checks your code to make sure it compiles, but doesn't produce an executable
cargo check

# Removes build artifacts
cargo clean

# Looks for tests to run in two places: in each of your src files and any tests in tests/.
cargo test

# Updates all dependencies - respects the SemVer constraints in Cargo.toml
cargo update
# Updates just "regex"
cargo update -p regex
```

## `Cargo.toml` and Lock Files {#cargo-toml}

```toml
# Configure the package
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at
# <https://doc.rust-lang.org/cargo/reference/manifest.html>

[dependencies]

# Reference a Crate to be Pulled from `crates.io`
time = "0.1.12"
# This is equivalent to the `^0.1.12` SemVer version range.
# `cargo update time` should update to version `0.1.13`, if it is the latest `0.1.z` release,
# but would not update to `0.2.0`

# Reference a Git repo
regex = { git = "https://github.com/rust-lang/regex.git" }

# Reference a sub-crate
# Points to folder `hello_utils`, inside of which a `Cargo.toml` and `src` folder
hello_utils = { path = "hello_utils", version = "0.1.0" }
```

Examples of version requirements{{hi:Version requirements}} and the versions that would be allowed with them:

```toml
1.2.3  :=  >=1.2.3, <2.0.0
1.2    :=  >=1.2.0, <2.0.0
1      :=  >=1.0.0, <2.0.0
0.2.3  :=  >=0.2.3, <0.3.0
0.2    :=  >=0.2.0, <0.3.0
0.0.3  :=  >=0.0.3, <0.0.4
0.0    :=  >=0.0.0, <0.1.0
0      :=  >=0.0.0, <1.0.0
```

Details in [Specifying Dependencies][book~cargo~specifying-dependencies]⮳.

If you're [building][p~building] a non-end product, such as a rust library that other rust packages will depend on, put `Cargo.lock` in your `.gitignore`.

If you're [building][p~building] an end product, which are executable like command-line tool or an application, or a system library with crate-type of [`staticlib`][book~rust-reference~linkage]{{hi:staticlib}}⮳ or [`cdylib`][book~rust-reference~linkage]{{hi:cdylib}}⮳, check `Cargo.lock`{{hi:Cargo.lock}} into git.

```sh
# Add dependencies to Cargo.toml from the command line
cargo add actix-web@4.0.0
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/915)
</div>
