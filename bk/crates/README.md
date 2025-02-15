# README

This folder contains the Rust example code that is embedded into the book.

## Layout

crates
    ├── about
    ├── categories
        ├── <crate named after the first letters of categories>
        │   ├── tests
        │       ├── <category>
        │           ├── <examples>
        ├── <...>
    ├── language
    ├── other
    ├── standard_library

- `about` contains the code examples in the book's introduction.
- `categories` contain multiple crates that contain the code examples for all (crates.io) categories.
- `language` contains the code examples for the Rust language per se.
- `other` contains the code examples for the Rust ecosystem that can't be mapped to a [`crates.io`][crates.io-website]{{hi:crates.io}}⮳ category.
- `standard_library` contains the code examples for the Rust standard library.

## In each crate

- `Cargo.toml` list all dependencies / libraries used in the book.
- Run `cargo add <crate>` (or edit `Cargo.toml`) in the appropriate crate to add new dependencies that new examples may require.

`tests` contains the current book examples, written as tests.
`temp` contains the outputs of the tests.
`src` contains a stub `lib.rs` file.

## Common commands

Type `just` to browse common commands, or manually enter the following:

```bash
cd <crate_folder>

cargo +nightly fmt --all
# then
cargo check --tests
# or
cargo build --tests
# or
cargo clippy --tests
# or
cargo nextest run -- --show-output
```
