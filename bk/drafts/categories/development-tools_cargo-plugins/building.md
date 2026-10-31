<!-- TODO Reviewed ??-??-2026 -->

# Build and run

{{#include building.incl.md}}

## `cargo make` {#cargo-make}

[![cargo-make][c~cargo-make~docs~badge]][c~cargo-make~docs]{{hi:cargo-make}}
[![cargo-make~crates.io][c~cargo-make~crates.io~badge]][c~cargo-make~crates.io]
[![cargo-make~repo][c~cargo-make~repo~badge]][c~cargo-make~repo]
[![cargo-make~lib.rs][c~cargo-make~lib.rs~badge]][c~cargo-make~lib.rs]
[![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::[testing][p~testing]]{{hi:Testing}}
[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}
[![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}
[![cat~development-tools::cargo-plugins][cat~development-tools::cargo-plugins~badge]][cat~development-tools::cargo-plugins]{{hi:Cargo plugins}}
[![cat~development-tools::build-utils][cat~development-tools::build-utils~badge]][cat~development-tools::build-utils]{{hi:Build Utils}}

[`cargo make`][c~cargo-make~repo]↗{{hi:cargo make}} is a Rust task runner and build tool. The [`cargo-make`][c~cargo-make~docs]↗{{hi:cargo-make}} task runner enables to define and configure sets of tasks and run them as a flow. A task is a command, script, rust code, or other sub tasks to execute. Tasks can have dependencies which are also tasks that will be executed before the task itself.
With a simple [toml][p~toml] based [configuration][p~configuration] file, we can define a multi platform build script that can run build, test, generate [documentation][p~documentation], run bench tests, run security validations and more, executed by running a single command.

Install with

```bash
cargo install --force cargo-make
cargo make --version
```

[automating-your-rust-workflows-with-cargo-make][c~cargo-make~blog]↗.

## `cargo xtask` {cargo-xtask}

[![cargo-xtask][c~cargo-xtask~docs~badge]][c~cargo-xtask~docs]{{hi:cargo-xtask}}
[![cargo-xtask~crates.io][c~cargo-xtask~crates.io~badge]][c~cargo-xtask~crates.io]
[![cargo-xtask~repo][c~cargo-xtask~repo~badge]][c~cargo-xtask~repo]

[cargo-xtask][c~cargo-xtask~repo]↗{{hi:cargo-xtask}} adds free-form automation to a Rust project, a-la [`make`][make~website]↗{{hi:make}}, [`npm run`][npm~website]↗{{hi:npm run}} or bespoke bash scripts.

The two distinguishing features of [`xtask`][c~cargo-xtask~docs]↗{{hi:xtask}} are the following:

- It doesn't require any other binaries besides [`cargo`][c~cargo~docs]↗{{hi:cargo}} and [`rustc`][book~rustc]↗{{hi:rustc}}, it fully bootstraps from them.
- Unlike `bash`, it can more easily be cross platform, as it doesn't use the shell.

Choose the smallest tool that fits the task: use Cargo profiles and build scripts for package compilation, a Cargo subcommand for a reusable Cargo operation, and a task runner or `xtask` for repository workflows such as release checks or code generation. Keep `build.rs` focused on work required to build that package; it runs as part of Cargo's build graph and is not a general task runner. An `xtask` is ordinary Rust code in a separate workspace member, so it can share project types and report errors through Rust's normal tooling.

### Use `devx` {#devx}

{{#badges devx-cmd}} {{hi:Cmd}} {{hi:Bash}} {{hi:Shell}} {{hi:Process}} {{#badges devx-pre-commit}} {{hi:Formatter}} {{hi:Hook}} {{hi:Pre-commit}} {{hi:Rustfmt}} {{hi:Git}} {{#categories development-tools="Development tools"}}

[`devx`][devx~repo]↗{{hi:devx}} is a collection of utilities for writing your own dev scripts in Rust. The project is inspired by and intended for seamless usage with [`cargo-xtask`][c~cargo-xtask~repo]↗ idioms.

[`devx-cmd`][c~devx-cmd~docs]↗{{hi:devx-cmd}} provides primitives for spawning child processes that are easier than `std::process targeted` when used in development scripts. [`devx-pre-commit`][c~devx-pre-commit~docs]↗{{hi:devx-pre-commit}} creates git pre-commit hooks that enforce good practices.

## Write cross-platform `bash`-like scripts in Rust with `xshell` {#xshell}

{{#badges xshell development-tools::build-utils="Build Utils" filesystem=Filesystem}}

"[`xshell`][c~xshell~repo]↗{{hi:xshell}} is a swiss-army knife for writing cross-platform 'bash' scripts in Rust.

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
{{#include ../../../crates/cats/development_tools_cargo_plugins/examples/building/xshell.rs}}
```

## Related Topics {#related-topics .skip}

| Topic                   | Rust Crates                                                                                                                                                                                                                   |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| General Build Guide     | See the [Cargo build overview][p~code-building] for ordinary package builds and profile configuration.                                                                                                                        |
| Repository Automation   | See [[development-tools_automation                                                                                                                                                                                            | Repository Automation]] for an `xtask` project-check example and Git history automation. |
| Cargo Commands          | See [[development-tools_cargo                                                                                                                                                                                                 | Cargo]] for built-in commands, package layout, workspaces, and build scripts. |
| Testing                 | See [[development-tools_testing                                                                                                                                                                                               | Testing]] for test runners and CI checks that can be composed by a task runner. |
| Cross-Compilation       | [`cross`][c~cross~docs]↗{{hi:cross}} simplifies cross-compilation.                                                                                                                                                            |
| Packaging, Distribution | [`cargo-deb`][c~cargo-deb~docs]↗{{hi:cargo-deb}} creates Debian packages. [`cargo-rpm`][c~cargo-rpm~docs]↗{{hi:cargo-rpm}} creates RPM packages. [`create-dmg`][create-dmg~repo]↗{{hi:create-dmg}} creates macOS disk images. |
| Build Script Helpers    | [`cc`][c~cc~docs]↗{{hi:cc}} helps with compiling C/C++ code in build scripts. [`pkg-config`][c~pkg-config~docs]↗{{hi:pkg-config}} finds system libraries.                                                                     |
| Code Generation         | Use build scripts.                                                                                                                                                                                                            |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
