# Publish to `crates.io`

{{#include publication.incl.md}}

## Publication Steps {#publish}

The [`publish`](https://github.com/john-cd/rust_howto/tree/main/publish)↗ folder contains a placeholder crate, so that the book could be located when searching on [`crates.io`][crates.io~website]{{hi:crates.io}}↗.

To publish the crate to `crates.io`, use the following steps:

- `cd publish`
- Review [`Cargo.toml`][book~cargo~cargo-toml]↗{{hi:Cargo.toml}}, update the metadata as needed. Keep the version in sync with that of the main workspace in [`bk/crates`][rust-howto~code-examples~github]↗.
- [`cargo update`]( ){{hi: }} if necessary.
- Go to [`crates.io`][crates.io~website]{{hi:crates.io}}↗, sign in, and create an API token in `Account Settings` > `API Tokens`.
- Use [`cargo login`]( ){{hi: }} to save the token in `$CARGO_HOME/credentials.toml`.
- `cargo build --locked --release`.
- `cargo clippy`.
- `cargo run --release`.
- `cargo doc`.
- Review `cargo package --list`.
- `cargo package`.
- Review the packaging output in `target/publish/package`.
- When ready, `cargo publish --dry-run; cargo publish`.

Type [`just`]( ){{hi: }} at the command line for a list of shortcuts for some of the above commands.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
