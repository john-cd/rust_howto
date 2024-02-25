# Cargo Plugins

Subcommands that extend the capabilities of Cargo.

{{#include index.incl.md}}

[![cargo-crates-github][cargo-crates-github-badge]][cargo-crates-github]

[![cargo-edit-github][cargo-edit-github-badge]][cargo-edit-github]

[![cargo-expand-github][cargo-expand-github-badge]][cargo-expand-github]

[![cargo-hack-github][cargo-hack-github-badge]][cargo-hack-github]

[![cargo-machete-github][cargo-machete-github-badge]][cargo-machete-github]

[![cargo-make-github][cargo-make-github-badge]][cargo-make-github]

[![cargo-xtask-github][cargo-xtask-github-badge]][cargo-xtask-github]

## Watch for changes

```sh
cargo install cargo-watch

# Runs `cargo check` after every code change
cargo watch -x check

# Run cargo check after code changes.
# If it succeeds, it launches cargo test.
# If tests pass, it launches the application with cargo run.
cargo watch -x check -x test -x run
```

## Formatting

```sh
# Install `rustfmt` if needed
rustup component add rustfmt

cargo fmt

# Fails if code is not formatted; use in CD / CI
cargo fmt -- --check
```

## Linting

```sh
rustup component add clippy # install if needed
cargo clippy
```

Mute a warning using the `#[allow(clippy::lint_name)]` attributes

## Fix Compiler Warnings

Can automatically fix compiler warnings that have a clear way to correct the problem that’s likely what you want.

```sh
cargo fix
```

## Code coverage

[![tarpaulin-github][tarpaulin-github-badge]][tarpaulin-github]

## Security audit

```sh
cargo install cargo-audit
cargo audit
```

## Unused dependencies

[udeps][cargo-udeps-crate]⮳

or (simpler) [Machete][cargo-machete-blog]⮳

```sh
cargo install cargo-machete
cargo machete
```

## Templates

[Cargo Generate][cargo-generate-crate]⮳ is a developer tool to help you get up and running quickly with a new Rust project by leveraging a pre-existing git repository as a template.

{{#include ../../refs/link-refs.md}}
