# Build and run

{{#include building.incl.md}}

## `cargo make` {#cargo-make}

[![cargo-make][c-cargo_make-badge]][c-cargo_make]{{hi:cargo-make}}
[![cargo-make-crates.io][c-cargo_make-crates.io-badge]][c-cargo_make-crates.io]
[![cargo-make-github][c-cargo_make-github-badge]][c-cargo_make-github]
[![cargo-make-lib.rs][c-cargo_make-lib.rs-badge]][c-cargo_make-lib.rs]
[![cat-development-tools::testing][cat-development-tools::testing-badge]][cat-development-tools::[testing][p-testing]]{{hi:Testing}}
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}
[![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}}
[![cat-development-tools::cargo-plugins][cat-development-tools::cargo-plugins-badge]][cat-development-tools::cargo-plugins]{{hi:Cargo plugins}}
[![cat-development-tools::build-utils][cat-development-tools::build-utils-badge]][cat-development-tools::build-utils]{{hi:Build Utils}}

`cargo make` is a Rust task runner and build tool. The [`cargo-make`][c-cargo_make]⮳{{hi:cargo-make}} task runner enables to define and configure sets of tasks and run them as a flow. A task is a command, script, rust code, or other sub tasks to execute. Tasks can have dependencies which are also tasks that will be executed before the task itself.
With a simple [toml][p-toml] based [configuration][p-configuration] file, you can define a multi platform build script that can run build, test, generate [documentation][p-documentation], run bench tests, run security validations and more, executed by running a single command.

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

The two distinguishing features of [`xtask`][c-cargo_xtask]⮳{{hi:xtask}} are the following:

- It doesn't require any other binaries besides [`cargo`][c-cargo]⮳{{hi:cargo}} and [`rustc`][rustc]⮳{{hi:rustc}}, it fully bootstraps from them
- Unlike bash, it can more easily be cross platform, as it doesn't use the shell.

### Use `devx` {#devx}

[![devx-cmd][c-devx_cmd-badge]][c-devx_cmd] [![devx-cmd-crates.io][c-devx_cmd-crates.io-badge]][c-devx_cmd-crates.io] [![devx-cmd-github][c-devx_cmd-github-badge]][c-devx_cmd-github] [![devx-cmd-lib.rs][c-devx_cmd-lib.rs-badge]][c-devx_cmd-lib.rs]{{hi:devx-cmd}}{{hi:Cmd}}{{hi:Bash}}{{hi:Shell}}{{hi:Process}} [![devx-pre-commit][c-devx_pre_commit-badge]][c-devx_pre_commit] [![devx-pre-commit-crates.io][c-devx_pre_commit-crates.io-badge]][c-devx_pre_commit-crates.io] [![devx-pre-commit-github][c-devx_pre_commit-github-badge]][c-devx_pre_commit-github] [![devx-pre-commit-lib.rs][c-devx_pre_commit-lib.rs-badge]][c-devx_pre_commit-lib.rs]{{hi:devx-pre-commit}}{{hi:Formatter}}{{hi:Hook}}{{hi:Pre-commit}}{{hi:Rustfmt}}{{hi:Git}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

[`devx`][devx-github]{{hi:devx}}⮳ is a collection of utilities for writing your own dev scripts in Rust. The project is inspired by and intended for seamless usage with [`cargo-xtask`][c-cargo_xtask-github]⮳ idioms.

[`devx-cmd`][c-devx_cmd]⮳{{hi:devx-cmd}} provides primitives for spawning child processes that are easier than `std::process targeted` when used in development scripts. [`devx-pre-commit`][c-devx_pre_commit]⮳{{hi:devx-pre-commit}} creates git pre-commit hooks that enforce good practices.

## Make Rust a better `bash` with `xshell` {#xshell}

[![xshell][c-xshell-badge]][c-xshell]{{hi:xshell}}
[![xshell-crates.io][c-xshell-crates.io-badge]][c-xshell-crates.io]
[![xshell-github][c-xshell-github-badge]][c-xshell-github]
[![xshell-lib.rs][c-xshell-lib.rs-badge]][c-xshell-lib.rs]

[`xshell`][c-xshell-github]⮳{{hi:xshell}} provides a set of cross-platform utilities for writing cross-platform and ergonomic "bash" scripts.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[building: expand (P1)](https://github.com/john-cd/rust_howto/issues/309)

Link to:

General Build Tools:

cargo build: (Built-in, but fundamental) Compiles your project.
cargo check: Checks your code for errors without compiling.
Cross-Compilation:

cross: Simplifies cross-compilation.
Packaging/Distribution:

cargo-deb: Creates Debian packages.
cargo-rpm: Creates RPM packages.
create-dmg: Creates macOS disk images.
Build Automation/Task Running:

xtask: Manages complex build tasks, often used for CI/CD.
Build Script Helpers:

cc: Helps with compiling C/C++ code in build scripts.
pkg-config: Finds system libraries.
Code Generation: (Often done in build scripts, but no single "build util" crate)

Link-Time Optimization (LTO) Configuration: (Configured in Cargo.toml, not a separate plugin)

Incremental Compilation Management: (Handled by [cargo][p-cargo] directly)
</div>
