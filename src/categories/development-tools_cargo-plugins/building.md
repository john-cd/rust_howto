# Build and run

{{#include building.incl.md}}

## `cargo make` {#cargo-make}

[![cargo-make][c-cargo_make-badge]][c-cargo_make]{{hi:cargo-make}}
[![cargo-make-crates.io][c-cargo_make-crates.io-badge]][c-cargo_make-crates.io]
[![cargo-make-github][c-cargo_make-github-badge]][c-cargo_make-github]
[![cargo-make-lib.rs][c-cargo_make-lib.rs-badge]][c-cargo_make-lib.rs]
[![cat-development-tools::testing][cat-development-tools::testing-badge]][cat-development-tools::testing]{{hi:Testing}}
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}
[![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}}
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}
[![cat-development-tools::build-utils][cat-development-tools::build-utils-badge]][cat-development-tools::build-utils]{{hi:Build Utils}}

Rust task runner and build tool. The cargo-make task runner enables to define and configure sets of tasks and run them as a flow. A task is a command, script, rust code, or other sub tasks to execute.
Tasks can have dependencies which are also tasks that will be executed before the task itself.
With a simple toml based configuration file, you can define a multi platform build script that can run build, test, generate documentation, run bench tests, run security validations and more, executed by running a single command.

Install with

```bash
cargo install --force cargo-make
cargo make --version
```

[automating-your-rust-workflows-with-cargo-make][c-cargo_make-blog]⮳

## `cargo xtask` {cargo-xtask}

[![cargo-xtask][c-cargo_xtask-badge]][c-cargo_xtask]{{hi:cargo-xtask}}
[![cargo-xtask-crates.io][c-cargo_xtask-crates.io-badge]][c-cargo_xtask-crates.io]
[![cargo-xtask-github][c-cargo_xtask-github-badge]][c-cargo_xtask-github]
[![cargo-xtask-lib.rs][c-cargo_xtask-lib.rs-badge]][c-cargo_xtask-lib.rs]

[cargo-xtask][c-cargo_xtask-github]{{hi:cargo-xtask}}⮳ adds free-form automation to a Rust project, a-la `make`, `npm run` or bespoke bash scripts.

The two distinguishing features of `xtask` are the following:

- It doesn't require any other binaries besides `cargo` and `rustc`, it fully bootstraps from them
- Unlike bash, it can more easily be cross platform, as it doesn't use the shell.

### Devx {#devx}

[devx][devx-github]{{hi:devx}}⮳ is a collection of utilities for writing your own dev scripts in Rust. The project is inspired by and intended for seamless usage with [`cargo-xtask`][c-cargo_xtask-github]⮳ idioms.

## xshell: Making Rust a Better Bash {#xshell}

[![xshell][c-xshell-badge]][c-xshell]{{hi:xshell}}
[![xshell-crates.io][c-xshell-crates.io-badge]][c-xshell-crates.io]
[![xshell-github][c-xshell-github-badge]][c-xshell-github]
[![xshell-lib.rs][c-xshell-lib.rs-badge]][c-xshell-lib.rs]

[`xshell`][c-xshell-github]⮳ {{hi:xshell}} provides a set of cross-platform utilities for writing cross-platform and ergonomic "bash" scripts.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: expand
</div>
