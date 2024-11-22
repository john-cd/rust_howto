# Deps

This package contains the Rust example code that is embedded into the book.

- `Cargo.toml` list all dependencies / libraries used in the book.
- Run `cargo add <crate>` (or edit `Cargo.toml`) in this folder to add new dependencies that new examples may require.

## Folders

`tests` contains the current book examples, written as tests.
`drafts` contains WIP book examples.
`temp` contains the outputs of the tests.
`src` contains a stub `lib.rs` file.

## Common commands

Type `just` to browse common commands, or manually enter the following:

```bash
cargo +nightly fmt --all
# then
cargo check --tests
# or
cargo build --tests
# or
cargo clippy --tests
# or
cargo test -- --show-output
```

<p xmlns:cc="http://creativecommons.org/ns#" >Code examples in this folder are marked with <a href="https://creativecommons.org/publicdomain/zero/1.0/?ref=chooser-v1" target="_blank" rel="license noopener noreferrer" style="display:inline-block;">CC0 1.0<img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1" alt=""><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/zero.svg?ref=chooser-v1" alt=""></a></p>
