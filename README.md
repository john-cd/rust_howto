<!-- TODO Reviewed ??-??-2026 -->

# The Rust How-To Book

[![Build and deploy (Linux)](https://github.com/john-cd/rust_howto/actions/workflows/build_deploy_book.yml/badge.svg)](https://github.com/john-cd/rust_howto/actions/workflows/build_deploy_book.yml)[![(Windows)](https://github.com/john-cd/rust_howto/actions/workflows/build_book_code_and_book_on_windows.yml/badge.svg)](https://github.com/john-cd/rust_howto/actions/workflows/build_book_code_and_book_on_windows.yml)

**A documentation summary and a compendium of recipes for the Rust language and ecosystem.**
**Everything you need for day-to-day Rust coding, all in one place.**

Point your browser to [`https://john-cd.com/rust_howto`][rust-howto~website]↗ to read the [Rust How-to][rust-howto~website]↗ book.

## Work In Progress

This book is going through _heavy edits_. Pardon the dust.

## Contributing

Contributions are most welcome! Please review the [contributing][rust-howto~contributing~website]↗ section of the book and peruse the book's [GitHub repo][rust-howto~repo]↗.

## Rust tooling

Use the pinned Rust 1.99.1 toolchain from [rust-toolchain.toml](rust-toolchain.toml) with
`just` for build, lint, and test tasks. Install Rust, sccache, and Just. Windows recipes require Git Bash.
Build paths and diagnostics are implemented directly in the justfile.

- `just build-env` displays the effective environment, checkout, and branch.
- `just cargo <subcommand> [args...]` runs Cargo with this checkout's output paths.

The repository root is not a Cargo workspace. Use the owning workspace
justfile, or select a manifest from the root, for example:

```sh
just cargo check --manifest-path tools/Cargo.toml
```

Run `just build-setup` once per checkout to create the ignored links used by
direct Cargo commands. On Windows, prefer `just cargo` so Cargo receives
absolute external paths instead of relying on junctions.

### Compiler cache

Direct Cargo defaults and `just` use sccache with incremental compilation
disabled and a 35G cache limit. The runner exports `RUSTC_WRAPPER=sccache`,
`CARGO_BUILD_RUSTC_WRAPPER=sccache`, `CARGO_INCREMENTAL=0`, and
`SCCACHE_CACHE_SIZE=35G`, replacing inherited values. Explicit shell or Cargo
command-line overrides still take precedence for direct Cargo invocations.

`CODE_SCCACHE_ROOT` overrides `SCCACHE_DIR`; otherwise the default below applies.
The task runner sets `SCCACHE_BASEDIRS` to the main checkout and registered `.worktrees/a`, `.worktrees/b`, and `.worktrees/c`,
discarding inherited unrelated paths. sccache reads daemon settings at startup;
wait for all builds to finish before restarting a daemon with stale settings.

Use `just cache-setup` to refresh the daemon configuration after all Rust
builds have stopped.

### Output paths and worktrees

Keep the primary checkout on `develop`. Use `.worktrees/a`, `.worktrees/b`, and
`.worktrees/c` for auxiliary checkouts. Preserve active or dirty checkouts when
reusing slots, and keep each slot's build cache. The primary checkout's output
slot is `main`, regardless of its branch name.

| Variable            | Windows default | Linux/macOS default                                       |
| ------------------- | --------------- | --------------------------------------------------------- |
| `CODE_TARGET_ROOT`  | `D:/target`     | `$XDG_CACHE_HOME/cargo-target` or `~/.cache/cargo-target` |
| `CODE_BUILD_ROOT`   | `C:/code_build` | `$XDG_CACHE_HOME/cargo-build` or `~/.cache/cargo-build`   |
| `CODE_SCCACHE_ROOT` | `D:/sccache`    | `$XDG_CACHE_HOME/sccache` or `~/.cache/sccache`           |
| `CODE_TEMP_ROOT`    | `D:/temp`       | Existing native `TMPDIR` or `/tmp`                        |

Set roots to absolute paths and create the directories before building. Keep
Windows intermediate builds on NTFS. Targets use
`CODE_TARGET_ROOT/rust_howto/<slot>` and intermediate files use
`CODE_BUILD_ROOT/rust_howto/<slot>`. The runner exports these as
`CARGO_TARGET_DIR`, `CARGO_BUILD_TARGET_DIR`, and `CARGO_BUILD_BUILD_DIR`.

On Windows, the runner sets `TEMP`, `TMP`, and `TMPDIR` from `CODE_TEMP_ROOT`.
Unix and WSL retain their native temporary directories.

## Companion Tools

Unpublished chapters have separate authoring previews. From `bk/`, run `just book drafts`
or `just book future` after building the helper binaries with `just release` from `tools/`.
These commands use `bk/drafts/book.toml` and `later/book.toml`, respectively, and write
HTML under `bk/book/`. They expand includes and custom directives without adding draft
chapters to the published book. Review build diagnostics as well as the exit status:
mdBook can report a missing include while continuing to render a chapter.

The [`mdbook-utils`][c~mdbook-utils~crates.io]↗ utility ([(github)][c~mdbook-utils~repo]↗ [(docs.rs)][c~mdbook-utils~docs]↗ [(user guide)][c~mdbook-utils~user-guide]↗) helps manage links and code blocks in [`mdbook`][c~mdbook~docs]↗ source directories. It is the companion tool for this book.

There are also a number of utilities and pre-processors in the [`tools`][rust-howto~tools~repo]↗ folder of this repo.

## Legal

Please refer to the LICENSE file.

[c~mdbook~docs]: https://rust-lang.github.io/mdBook
[c~mdbook-utils~repo]: https://github.com/john-cd/mdbook-utils
[c~mdbook-utils~docs]: https://docs.rs/mdbook-utils/latest/mdbook_utils
[c~mdbook-utils~crates.io]: https://crates.io/crates/mdbook-utils
[c~mdbook-utils~user-guide]: https://john-cd.github.io/mdbook-utils
[rust-howto~website]: https://john-cd.com/rust_howto
[rust-howto~contributing~website]: https://john-cd.com/rust_howto/appendices/contributing/index.html
[rust-howto~tools~repo]: https://github.com/john-cd/rust_howto/tree/develop/tools
[rust-howto~repo]: https://github.com/john-cd/rust_howto

## Python development

The Python extension example lives in `later/crates/cats/development_tools_ffi/python_extension` and uses the committed `uv.lock` and `.python-version`. From the repository root, use `just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile env`, then `just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile sync` and `just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile uv run <command>`. Use `just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile hooks` for hooks. See [`later/crates/cats/development_tools_ffi/python_extension/README.md`](later/crates/cats/development_tools_ffi/python_extension/README.md) for project details.

Python environments, caches, and generated outputs use the configurable `CODE_PYTHON_ENV_ROOT`, `CODE_PYTHON_BUILD_ROOT`, and `CODE_PYTHON_OUTPUT_ROOT`; package data uses `CODE_PACKAGES_ROOT`. These are separate from the Rust-specific `CODE_BUILD_ROOT`, `CODE_TARGET_ROOT`, and `CODE_SCCACHE_ROOT`.

## Python tooling

Install Just and uv; Windows recipes use Git Bash. Each Python project has a
Justfile. Environment variables are defined in `scripts/env.just`, imported
by those Justfiles; no Python environment wrapper is required.

- `just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile env` — `later/crates/cats/development_tools_ffi/python_extension`.

From a project's directory, run `just sync`, `just fmt`, `just lint`, or
`just check`. `just --list` shows its available tasks. Named validation tasks
use `uv run --locked`. Ruff formatting and linting use a separate `<slot>-lint`
environment synchronized from the locked `lint` dependency group only; they do
not install or build the Python extension. `just uv <args...>` forwards arbitrary
uv commands and `just run <command...>` runs a command in the development environment.

`just hooks-install` installs the repository's prek hooks; `just hooks` runs
them against all files. Install prek first if needed with `just uv tool install prek`.

Virtual environments use `CODE_PYTHON_ENV_ROOT/<repo>/<project>/<slot>`.
Caches and generated files use matching paths under `CODE_PYTHON_BUILD_ROOT`
and `CODE_PYTHON_OUTPUT_ROOT`. The main checkout uses slot `main`; linked
worktrees use their letter. Configure `CODE_PACKAGES_ROOT` for uv/pip/prek
caches and `CODE_TOOLS_ROOT` for uv-managed interpreters and tools.
Windows defaults are `D:/packages`, `C:/tools`, and `D:/temp` for temporary
files; Unix uses `~/.local/share`, `~/.local/tools`, and native temporary paths.
`CODE_PYTHON_ENV_ROOT` defaults to `<packages>/uv/environments`; build and output
roots default to `<temp>/python-build` and `<temp>/python-output`.
`just env` prints the resolved values. `just uv build` routes distributions to the external
output directory unless an explicit `--out-dir` or `-o` is supplied.

## Checkout and storage policy

The primary checkout uses `develop`; `main` is an independent empty branch.
Auxiliary worktrees belong under `.worktrees/`, with names a and b only.
Storage roots are configurable through `CODE_REPOS_ROOT`, `CODE_TEMP_ROOT`,
`CODE_PACKAGES_ROOT`, and `CODE_TOOLS_ROOT`. Windows defaults are `D:/code`,
`D:/temp` (`C:/temp` fallback), `D:/packages`, and `C:/tools` respectively.
Unix uses native home/cache/temp paths. A root override changes destinations;
it does not move existing installations. Tool-specific overrides such as
`CARGO_HOME`, `npm_config_cache`, and `NUGET_PACKAGES` remain independent.
Rust builds also read `CODE_BUILD_ROOT`, `CODE_TARGET_ROOT`, and
`CODE_SCCACHE_ROOT`. Their Windows defaults are `C:/code_build`, `D:/target`,
and `D:/sccache`. The runner rejects non-NTFS intermediate build roots.
Run `just configure-cargo-host` once per host (PowerShell 7 is required for
this setup command), and rerun it after changing roots. This preserves other Cargo
settings and sets the host fallback in `CARGO_HOME/config.toml` for repositories
without local output settings:
`CODE_TARGET_ROOT/direct-cargo` for targets and
`CODE_BUILD_ROOT/direct-cargo/{workspace-path-hash}` for intermediates.
Use `just cargo <arguments>` for isolated per-repo/worktree directories,
the configured temp environment, and registered sccache base directories.
Run the build-setup recipe per checkout so direct Cargo and rust-analyzer use
that checkout's target/build links. Local config takes precedence over the host
fallback. The runner uses sccache with incremental compilation disabled.

## Repository configuration

See [CONFIGURATION.md](CONFIGURATION.md) for shared formatting, lint, dependency-update,
build-profile, cache, linker, and editor conventions. Keep it synchronized with configuration changes.
