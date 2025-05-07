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

[automating-your-rust-workflows-with-cargo-make][c-cargo_make-blog]⮳.

## `cargo xtask` {cargo-xtask}

[![cargo-xtask][c-cargo_xtask-badge]][c-cargo_xtask]{{hi:cargo-xtask}}
[![cargo-xtask-crates.io][c-cargo_xtask-crates.io-badge]][c-cargo_xtask-crates.io]
[![cargo-xtask-github][c-cargo_xtask-github-badge]][c-cargo_xtask-github]
[![cargo-xtask-lib.rs][c-cargo_xtask-lib.rs-badge]][c-cargo_xtask-lib.rs]

[cargo-xtask][c-cargo_xtask-github]{{hi:cargo-xtask}}⮳ adds free-form automation to a Rust project, a-la `make`, `npm run` or bespoke bash scripts.

The two distinguishing features of [`xtask`][c-cargo_xtask]⮳{{hi:xtask}} are the following:

- It doesn't require any other binaries besides [`cargo`][c-cargo]⮳{{hi:cargo}} and [`rustc`][rustc]⮳{{hi:rustc}}, it fully bootstraps from them.
- Unlike `bash`, it can more easily be cross platform, as it doesn't use the shell.

### Use `devx` {#devx}

[![devx-cmd][c-devx_cmd-badge]][c-devx_cmd] [![devx-cmd-crates.io][c-devx_cmd-crates.io-badge]][c-devx_cmd-crates.io] [![devx-cmd-github][c-devx_cmd-github-badge]][c-devx_cmd-github] [![devx-cmd-lib.rs][c-devx_cmd-lib.rs-badge]][c-devx_cmd-lib.rs]{{hi:devx-cmd}}{{hi:Cmd}}{{hi:Bash}}{{hi:Shell}}{{hi:Process}} [![devx-pre-commit][c-devx_pre_commit-badge]][c-devx_pre_commit] [![devx-pre-commit-crates.io][c-devx_pre_commit-crates.io-badge]][c-devx_pre_commit-crates.io] [![devx-pre-commit-github][c-devx_pre_commit-github-badge]][c-devx_pre_commit-github] [![devx-pre-commit-lib.rs][c-devx_pre_commit-lib.rs-badge]][c-devx_pre_commit-lib.rs]{{hi:devx-pre-commit}}{{hi:Formatter}}{{hi:Hook}}{{hi:Pre-commit}}{{hi:Rustfmt}}{{hi:Git}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

[`devx`][devx-github]{{hi:devx}}⮳ is a collection of utilities for writing your own dev scripts in Rust. The project is inspired by and intended for seamless usage with [`cargo-xtask`][c-cargo_xtask-github]⮳ idioms.

[`devx-cmd`][c-devx_cmd]⮳{{hi:devx-cmd}} provides primitives for spawning child processes that are easier than `std::process targeted` when used in development scripts. [`devx-pre-commit`][c-devx_pre_commit]⮳{{hi:devx-pre-commit}} creates git pre-commit hooks that enforce good practices.

## Write cross-platform `bash`-like scripts in Rust with `xshell` {#xshell}

[![xshell][c-xshell-badge]][c-xshell] [![xshell-crates.io][c-xshell-crates.io-badge]][c-xshell-crates.io] [![xshell-github][c-xshell-github-badge]][c-xshell-github] [![xshell-lib.rs][c-xshell-lib.rs-badge]][c-xshell-lib.rs]{{hi:xshell}} [![cat-development-tools::build-utils][cat-development-tools::build-utils-badge]][cat-development-tools::build-utils]{{hi:Build Utils}} [![cat-filesystem][cat-filesystem-badge]][cat-filesystem]{{hi:Filesystem}}

"[`xshell`][c-xshell-github]⮳{{hi:xshell}} is a swiss-army knife for writing cross-platform 'bash' scripts in Rust.

It doesn't use the shell directly, but rather re-implements parts of its scripting environment in Rust. The intended use-case is various bits of glue code, which could be written in `bash` or `python`. The original motivation is `xtask` development" [(docs.rs)][c-xshell].

The following example executes shell commands and interacts with the file system. It showcases:

- Basic command execution,
- Handling command arguments,
- Working directory manipulation,
- Using pipes,
- Checking command status,
- Conditional execution,
- Capturing `stderr`,
- Environment variable manipulation,
- Path manipulations,
- File removal.

```rust,editable
{{#include ../../../crates/cats/development_tools_cargo_plugins/tests/building/xshell.rs}}
```

## Related Topics

| Topic | Rust Crates |
|---|---|
| General Build Tools | `cargo build` (built-in) compiles your project. `cargo check` checks your code for errors without compiling. |
| Cross-Compilation | [`cross`][c-cross]⮳{{hi:cross}} simplifies cross-compilation. |
| Packaging, Distribution | [`cargo-deb`][c-cargo_deb]⮳{{hi:cargo-deb}} creates Debian packages. [`cargo-rpm`][c-cargo_rpm]⮳{{hi:cargo-rpm}} creates RPM packages. `create-dmg` creates macOS disk images. |
| Build Script Helpers | [`cc`][c-cc]⮳{{hi:cc}} helps with compiling C/C++ code in build scripts. [`pkg-config`][c-pkg_config]⮳{{hi:pkg-config}} finds system libraries. |
| Code Generation | Use build scripts. |
| Link-Time Optimization (LTO) Configuration | Configured in Cargo.toml |
| Incremental Compilation Management | Handled by [cargo][p-cargo] directly. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[building: expand; cross link](https://github.com/john-cd/rust_howto/issues/309)
</div>
