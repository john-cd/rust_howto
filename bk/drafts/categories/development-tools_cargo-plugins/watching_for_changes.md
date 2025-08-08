# Watching for Changes

{{#include watching_for_changes.incl.md}}

| Topic | Rust Crates |
|---|---|
| File Watching and Rebuilding | `cargo watch` watches your project for changes and automatically rebuilds and reruns your code. This is the most common and generally recommended tool. |
| Other [File Watching][p~file-watching] Tools | (Less common for Rust projects specifically, but might be used in more complex setups) [`watchexec`][c~watchexec~docs]↗{{hi:watchexec}} is a general-purpose file watcher that can execute commands on file changes. You could use it to trigger Cargo commands, but [cargo][p~cargo] watch is usually simpler. |

It's worth noting that some [IDEs][p~ides] also have built-in [file watching][p~file-watching] and automatic build features. If you're using an IDE, check its settings as you might not need a separate tool like `cargo watch`.

## `cargo watch` {#cargo-watch}

[![cargo-watch][c~cargo-watch~docs~badge]][c~cargo-watch~docs]{{hi:cargo-watch}}
[![cargo-watch~crates.io][c~cargo-watch~crates.io~badge]][c~cargo-watch~crates.io]
[![cargo-watch~github][c~cargo-watch~github~badge]][c~cargo-watch~github]
[![cargo-watch~lib.rs][c~cargo-watch~lib.rs~badge]][c~cargo-watch~lib.rs]
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}} [![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}

`cargo watch`

```sh
cargo install cargo-watch

# Runs `cargo check` after every code change
cargo watch -x check

# Run `cargo check` after code changes.
# If it succeeds, it launches `cargo test`.
# If tests pass, it launches the application with `cargo run`.
cargo watch -x check -x test -x run
```

## `cargo limit` {#cargo-limit}

[![cargo-limit][c~cargo-limit~docs~badge]][c~cargo-limit~docs]{{hi:cargo-limit}}
[![cargo-limit~crates.io][c~cargo-limit~crates.io~badge]][c~cargo-limit~crates.io]
[![cargo-limit~github][c~cargo-limit~github~badge]][c~cargo-limit~github]
[![cargo-limit~lib.rs][c~cargo-limit~lib.rs~badge]][c~cargo-limit~lib.rs]

[cargo-limit][c~cargo-limit~crates.io]↗ is [Cargo][p~cargo] with less noise: warnings are skipped until errors are fixed, Neovim integration, etc.

- errors have highest priority.
- they never appear in the middle of warnings.
- warnings are skipped by default until errors are fixed.
- external path dependencies' warnings are skipped by default.
- all messages come in reversed order by default to avoid extra scrolling.
- messages are grouped by filenames.
- number of messages can be limited.
- after encountering first error the rest of build time is limited by default.
- files can be automatically opened in your text editor on affected lines.

This tool is especially useful in combination with [`cargo-watch`][c~cargo-watch~docs]↗{{hi:cargo-watch}}.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[watching_for_changes: expand](https://github.com/john-cd/rust_howto/issues/315)
</div>
