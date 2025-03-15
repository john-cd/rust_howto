# Publish to `crates.io` {#publish}

{{#include publication.incl.md}}

The `crates/publish` folder contains a placeholder crate, so that the book could be located when searching on [`crates.io`][crates.io-website]{{hi:crates.io}}⮳.

- `cargo update` if necessary.
- Go to [`crates.io`][crates.io-website]{{hi:crates.io}}⮳, sign in, and create an API token in `Account Settings` > `API Tokens`.
- Use `cargo login` to save the token in `$CARGO_HOME/credentials.toml`.
- `cd crates/publish`.
- `cargo build --locked --release`.
- `cargo clippy`.
- `cargo run --release`.
- `cargo doc`.
- Review `cargo package --list`.
- `cargo package`.
- Review the packaging output in `/cargo-target-rust_howto/target/package`.
- When ready, `cargo publish --dry-run; cargo publish`.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[publication: edit](https://github.com/john-cd/rust_howto/issues/531)
</div>
