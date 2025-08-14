# Build and run

{{#include building.incl.md}}

## `cargo make` {#cargo-make}

[![cargo-make][c~cargo-make~docs~badge]][c~cargo-make~docs]{{hi:cargo-make}}
[![cargo-make~crates.io][c~cargo-make~crates.io~badge]][c~cargo-make~crates.io]
[![cargo-make~github][c~cargo-make~github~badge]][c~cargo-make~github]
[![cargo-make~lib.rs][c~cargo-make~lib.rs~badge]][c~cargo-make~lib.rs]
[![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::[testing][p~testing]]{{hi:Testing}}
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}
[![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}
[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}
[![cat~development-tools::build-utils][cat~development-tools::build-utils~badge]][cat~development-tools::build-utils]{{hi:Build Utils}}

[`cargo make`][c~cargo-make~github]↗{{hi:cargo make}} is a Rust task runner and build tool. The [`cargo-make`][c~cargo-make~docs]↗{{hi:cargo-make}} task runner enables to define and configure sets of tasks and run them as a flow. A task is a command, script, rust code, or other sub tasks to execute. Tasks can have dependencies which are also tasks that will be executed before the task itself.
With a simple [toml][p~toml] based [configuration][p~configuration] file, you can define a multi platform build script that can run build, test, generate [documentation][p~documentation], run bench tests, run security validations and more, executed by running a single command.

Install with

```bash
cargo install --force cargo-make
cargo make --version
```

[automating-your-rust-workflows-with-cargo-make][c~cargo-make~blog]↗.

## `cargo xtask` {cargo-xtask}

[![cargo-xtask][c~cargo-xtask~docs~badge]][c~cargo-xtask~docs]{{hi:cargo-xtask}}
[![cargo-xtask~crates.io][c~cargo-xtask~crates.io~badge]][c~cargo-xtask~crates.io]
[![cargo-xtask~github][c~cargo-xtask~github~badge]][c~cargo-xtask~github]
[![cargo-xtask~lib.rs][c~cargo-xtask~lib.rs~badge]][c~cargo-xtask~lib.rs]

[cargo-xtask][c~cargo-xtask~github]↗{{hi:cargo-xtask}} adds free-form automation to a Rust project, a-la [`make`][make~website]↗{{hi:make}}, [`npm run`][npm~website]↗{{hi:npm run}} or bespoke bash scripts.

The two distinguishing features of [`xtask`][c~cargo-xtask~docs]↗{{hi:xtask}} are the following:

- It doesn't require any other binaries besides [`cargo`][c~cargo~docs]↗{{hi:cargo}} and [`rustc`][book~rustc]↗{{hi:rustc}}, it fully bootstraps from them.
- Unlike `bash`, it can more easily be cross platform, as it doesn't use the shell.

### Use `devx` {#devx}

[![devx-cmd][c~devx-cmd~docs~badge]][c~devx-cmd~docs] [![devx-cmd~crates.io][c~devx-cmd~crates.io~badge]][c~devx-cmd~crates.io] [![devx-cmd~github][c~devx-cmd~github~badge]][c~devx-cmd~github] [![devx-cmd~lib.rs][c~devx-cmd~lib.rs~badge]][c~devx-cmd~lib.rs]{{hi:devx-cmd}}{{hi:Cmd}}{{hi:Bash}}{{hi:Shell}}{{hi:Process}} [![devx-pre-commit][c~devx-pre-commit~docs~badge]][c~devx-pre-commit~docs] [![devx-pre-commit~crates.io][c~devx-pre-commit~crates.io~badge]][c~devx-pre-commit~crates.io] [![devx-pre-commit~github][c~devx-pre-commit~github~badge]][c~devx-pre-commit~github] [![devx-pre-commit~lib.rs][c~devx-pre-commit~lib.rs~badge]][c~devx-pre-commit~lib.rs]{{hi:devx-pre-commit}}{{hi:Formatter}}{{hi:Hook}}{{hi:Pre-commit}}{{hi:Rustfmt}}{{hi:Git}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[`devx`][devx~github]↗{{hi:devx}} is a collection of utilities for writing your own dev scripts in Rust. The project is inspired by and intended for seamless usage with [`cargo-xtask`][c~cargo-xtask~github]↗ idioms.

[`devx-cmd`][c~devx-cmd~docs]↗{{hi:devx-cmd}} provides primitives for spawning child processes that are easier than `std::process targeted` when used in development scripts. [`devx-pre-commit`][c~devx-pre-commit~docs]↗{{hi:devx-pre-commit}} creates git pre-commit hooks that enforce good practices.

## Write cross-platform `bash`-like scripts in Rust with `xshell` {#xshell}

[![xshell][c~xshell~docs~badge]][c~xshell~docs] [![xshell~crates.io][c~xshell~crates.io~badge]][c~xshell~crates.io] [![xshell~github][c~xshell~github~badge]][c~xshell~github] [![xshell~lib.rs][c~xshell~lib.rs~badge]][c~xshell~lib.rs]{{hi:xshell}} [![cat~development-tools::build-utils][cat~development-tools::build-utils~badge]][cat~development-tools::build-utils]{{hi:Build Utils}} [![cat~filesystem][cat~filesystem~badge]][cat~filesystem]{{hi:Filesystem}}

"[`xshell`][c~xshell~github]↗{{hi:xshell}} is a swiss-army knife for writing cross-platform 'bash' scripts in Rust.

It doesn't use the shell directly, but rather re-implements parts of its scripting environment in Rust. The intended use-case is various bits of glue code, which could be written in [`bash`][bash~website]↗{{hi:bash}} or [`python`][python~website]↗{{hi:python}}. The original motivation is `xtask`{{hi:xtask}} development" [(docs.rs)][c~xshell~docs]↗.

The following example executes shell commands and interacts with the file system. It showcases:

- Basic command execution,
- Handling command arguments,
- Working directory manipulation,
- Using pipes,
- Checking command status,
- Conditional execution,
- Capturing `stderr`{{hi:stderr}},
- Environment variable manipulation,
- Path manipulations,
- File removal.

```rust,editable
{{#include ../../../crates/cats/development_tools_cargo-plugins/examples/building/xshell.rs}}
```

## Related Topics

| Topic | Rust Crates |
|---|---|
| General Build Tools | [`cargo build`][book~cargo~cargo-build]↗{{hi:cargo build}} (built-in) compiles your project. `cargo check` checks your code for errors without compiling. |
| Cross-Compilation | [`cross`][c~cross~docs]↗{{hi:cross}} simplifies cross-compilation. |
| Packaging, Distribution | [`cargo-deb`][c~cargo-deb~docs]↗{{hi:cargo-deb}} creates Debian packages. [`cargo-rpm`][c~cargo-rpm~docs]↗{{hi:cargo-rpm}} creates RPM packages. [`create-dmg`][create-dmg~github]↗{{hi:create-dmg}} creates macOS disk images. |
| Build Script Helpers | [`cc`][c~cc~docs]↗{{hi:cc}} helps with compiling C/C++ code in build scripts. [`pkg-config`][c~pkg-config~docs]↗{{hi:pkg-config}} finds system libraries. |
| Code Generation | Use build scripts. |
| Link-Time Optimization (LTO) Configuration | Configured in Cargo.toml |
| Incremental Compilation Management | Handled by [cargo][p~cargo] directly. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[building: expand; cross link](https://github.com/john-cd/rust_howto/issues/309)
</div>
