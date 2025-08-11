# README

The `bk/crates/` folder contains the Rust example code that is embedded into the book.

## Layout

`Cargo.toml` defines a Cargo workspace that includes the following crates:

```txt
bk
├── crates
    ├── about
    ├── cats
    |    ├── <category>
    |    │   ├── tests
    |    │   |    ├── <chapter>
    |    │   |        ├── <example>.rs
    |    |   |        ├── <...>
    |    |   |        ├── main.rs
    |    |   ├── Cargo.toml (crate)
    |    ├── <...>
    ├── code_organization
    ├── language
    ├── other
    ├── proc_macros
    ├── standard_library
    ├── Cargo.toml (workspace config file)
```

- `about` contains the code examples in the book's introduction.
- `cats` contains many crates, each containing the code examples for one `crates.io` category.
- `code_organization` contains the code examples for the code organization section.
- `language` contains the code examples for the Rust language _per se_.
- `other` contains the code examples that can't be mapped to a [`crates.io`][crates.io~website]{{hi:crates.io}}↗ category.
- `proc_macros` contains procedural macro examples used in the "language" section of the book.
- `standard_library` contains the code examples for the Rust standard library.

## In Each Crate Folder

- The `examples` folder (and sometimes the `tests` folder, or `build.rs`) contains the code examples.
- `temp` contains the (temporary) outputs of the tests.
- `src` may contain a stub `lib.rs` file. However, most crates are configured as `autolib = false`.
- `Cargo.toml` list all dependencies / libraries used by code examples in the crate.
  - Run `cargo add <dependency_crate> -p <crate_name>` (or edit the crate's `Cargo.toml`) to add new dependencies that new examples may require.
  - `Cargo.toml` may contain `[features]` to reduce build times, when dealing with very large dependencies.

In the `tests` folder, you will typically find multi-file integration tests, meaning folders, each named after a chapter (a `*.md` file below `bk/src`), containing a `main.rs` file along several modules (`.rs` files), each a code example.

See [Cargo package layout][cargo-package-layout]↗ for more details.

## Common Commands

At the terminal, type `just` to browse available commands.

For example,

- `just bc` runs `cargo build --all-targets --locked --all-features` for the crate in the current working directory.
- `just ntc` tests the crate.

You may also manually execute one of the following commands:

```bash
cd <crate_folder>

cargo +nightly fmt --all

cargo check --tests --examples

cargo build --tests --examples

cargo clippy --tests --examples

cargo nextest run --status-level all --success-output final
```

[crates.io~website]: https://crates.io
[cargo-package-layout]: https://doc.rust-lang.org/cargo/guide/project-layout.html
