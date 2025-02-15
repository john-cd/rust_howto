# Watching for changes

{{#include watching_for_changes.incl.md}}

## `cargo watch` {#cargo-watch}

[![cargo-watch][c-cargo_watch-badge]][c-cargo_watch]{{hi:cargo-watch}}
[![cargo-watch-crates.io][c-cargo_watch-crates.io-badge]][c-cargo_watch-crates.io]
[![cargo-watch-github][c-cargo_watch-github-badge]][c-cargo_watch-github]
[![cargo-watch-lib.rs][c-cargo_watch-lib.rs-badge]][c-cargo_watch-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}

`cargo watch`

```sh
cargo install cargo-watch

# Runs `cargo check` after every code change
cargo watch -x check

# Run cargo check after code changes.
# If it succeeds, it launches cargo test.
# If tests pass, it launches the application with cargo run.
cargo watch -x check -x test -x run
```

## `cargo limit` {#cargo-limit}

[![cargo-limit][c-cargo_limit-badge]][c-cargo_limit]{{hi:cargo-limit}}
[![cargo-limit-crates.io][c-cargo_limit-crates.io-badge]][c-cargo_limit-crates.io]
[![cargo-limit-github][c-cargo_limit-github-badge]][c-cargo_limit-github]
[![cargo-limit-lib.rs][c-cargo_limit-lib.rs-badge]][c-cargo_limit-lib.rs]

[cargo-limit][c-cargo_limit-crates.io]⮳ is Cargo with less noise: warnings are skipped until errors are fixed, Neovim integration, etc.

- errors have highest priority
- they never appear in the middle of warnings
- warnings are skipped by default until errors are fixed
- external path dependencies' warnings are skipped by default
- all messages come in reversed order by default to avoid extra scrolling
- messages are grouped by filenames
- number of messages can be limited
- after encountering first error the rest of build time is limited by default
- files can be automatically opened in your text editor on affected lines

This tool is especially useful in combination with `cargo-watch`.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[watching_for_changes: expand (P1)](https://github.com/john-cd/rust_howto/issues/315)
</div>
