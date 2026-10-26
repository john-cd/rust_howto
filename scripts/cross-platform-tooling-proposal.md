# Proposal: make every command work on Linux and Windows

Status: implementation in progress ([issue #2000](https://github.com/john-cd/rust_howto/issues/2000)). Keep `just` as the task runner, use Bash as the preferred scripting language with Git Bash on Windows, and migrate suitable shared script logic to Rust tools incrementally. Do not adopt cargo-make or Python for tooling. Scope: `rust_howto` (the `justfile`s, `bk/scripts/**`, `scripts/`, `.devcontainer/`), with notes on `book_tooling` and `mdbook-utils`.

## What exists today

- About 1,970 lines of `justfile` / `*.just` across 7 workspaces and 23 `bk/scripts/*` modules.
- 39 `.sh`, 4 `.ps1` and 6 `.py` files outside the example crates.
- Most justfiles use Bash on Unix and PowerShell on Windows. The affected `bk/scripts` modules now use Git Bash for the corresponding Windows recipes.
- About 70 `[unix]` and 24 `[windows]` attributes. Recipes are duplicated per OS only in `book`, `docs`, `precommit`, `recipes`, `utils` and a few others. These modules had **only `[unix]` recipes and no Windows counterpart**: `anchors`, `chapters`, `crates`, `examples`, `gh`, `includes`, `spelling`, `titles`, `toc`, `urls`, `wikilinks`. The `quick` book-preview recipe also lacked Windows support; Windows counterparts or shared Bash recipes have now been added for these gaps.
- The unix scripts lean on `grep -P`, `comm`, `sed -r`, `mktemp`, `realpath`, `/tmp`, `rm --recursive` and `aspell`/`lychee`.
- The unguarded `$env:RUSTC_WRAPPER = ""` lines in `bk/scripts/code/mod.just` and `later/justfile` were split into Unix Bash and Windows PowerShell recipe variants.
- `scripts/build.just` already hard-codes `D:/target`, `C:/code_build` and `D:/sccache` on Windows, and `setup-build.sh` / `setup-build.ps1` are two ports of the same ~90 lines.
- CI already runs on `ubuntu`, `windows-2025` and `macos-15`, and the repo already ships Rust helper binaries (`tools/`, `bin/clean.exe`, `mdbook-utils`) and one Python script (`refdefs.py`).

The simple recipes (`cargo ...`, `just code ...`) already work on both OSes. The pain is concentrated in the roughly 40 shell scripts and the duplicated `[unix]`/`[windows]` pairs.

## Options compared

| | (a) Keep just, use Bash | (b) Rewrite in cargo-make | (c) Rust tools | (d) Python (`uv`) |
|---|---|---|---|---|
| Idea | Keep recipes in just; make Bash the preferred scripting language and use Git Bash on Windows, with OS-specific recipes where needed | Move all tasks to `Makefile.toml` | Port shared script logic to Rust CLIs called from thin recipes | Port scripts to Python |
| Effort | 2-4 days | 2-3 weeks | 1-2 weeks, incremental, per script | 1-2 weeks, incremental |
| Cross-platform quality | Good for one-liners, poor for the 40 script bodies | Good, but cargo-make shell tasks still shell out | Best: one implementation, testable, no quoting issues | Good; needs Python or pwsh installed |
| Fit with this repo | Excellent: modules, `JUST_VERSION` pin, AGENTS.md, CI, devcontainer all assume just; Git Bash is the shell bridge for Windows | Poor: loses `mod` namespaces, `just --list` groups, and everything above must be rewritten | Excellent: `tools/` workspace, `tool_lib`, `bin/` release flow and `mdbook-utils` already exist | Not selected: adds another maintained tooling language |
| Main risk | Behavioural drift between bash and pwsh; Git-Bash vs WSL `bash.exe` ambiguity on Windows | Big-bang migration, new DSL and Duckscript to learn, docs and CI rewrites | Compile time before first use; needs the `bin/` release step | Extra runtime to install; two languages to maintain |

Notes on each:

- **(a)** Keep just and prefer Bash for script bodies. On Windows, run Bash recipes through Git Bash (`C:/Program Files/Git/bin/bash.exe`) and add `[windows]` counterparts when the recipe or its dependencies need an explicit Windows entry point. Keep native Windows commands behind `[windows]` guards. This avoids turning every recipe into PowerShell and lets existing `.sh` scripts remain the source of truth. Custom installations can override the shell with just's `--shell` and `--shell-arg` options.
- **(b)** cargo-make adds a second task runner, different variable and condition syntax, and Duckscript. It does not remove the need for portable script bodies, so it costs the most and solves the least here.
- **(c)** Same approach as `bin/clean.exe`: add a `tools` subcommand (or a small `xtask`-style binary) per script, for example `refdefs check`, `urls check`, `links inline`, `toc missing`, `setup-build`. Recipes become one-line calls like `cargo run -p tool -- refdefs check {{root}}`. Plain `cargo xtask` is a weaker fit because this is a monorepo with seven independent Cargo workspaces, so a single workspace-level alias would not be reachable from all of them; living in `tools/` and releasing to `bin/` already matches the existing pattern. Much of the logic (reference definitions, links, includes, sitemap) already lives in `mdbook-utils` and `book_tooling/parse_lib`, which would be reused rather than rewritten.
- **(d)** Python is not selected for repository tooling. Existing one-off Python examples can remain where they are, but new tooling should use Bash or the existing Rust tool workspace.

## Recommendation

Keep `just`, use Bash through Git Bash for shell-based tooling, and migrate suitable shared logic to Rust tools over time. Do not adopt cargo-make or Python for repository tooling.

1. **Quick wins.** Fix recipes that contain PowerShell syntax without a `[windows]` guard. Add Windows counterparts for frequently used Unix-only recipes, invoking Bash through Git Bash where their script dependencies support it.
2. **Consolidate duplicated pairs.** Replace `[unix]`/`[windows]` pairs that differ only in shell syntax (`copystatic`, `sitemap`, `mdbook build`, `clean`) with a shared Bash recipe when both platforms have the required tools.
3. **Port suitable scripts to Rust tools incrementally.** Start with frequently used shared logic such as reference definitions, URLs, TOC, includes, links, and build setup. Reuse `mdbook-utils` and `book_tooling/parse_lib` where possible. Keep one-off integrations and book examples in their existing language when a Rust port has no clear benefit.
4. **Review** remaining scripts for incremental Rust ports and record which external commands still need per-OS installation guidance.

Git Bash is the supported Bash provider for Windows recipes. The affected modules use the standard `C:/Program Files/Git/bin/bash.exe` install path; custom installations can use just's `--shell` and `--shell-arg` options. Git Bash supplies the shell and common Unix utilities, but it does not install project CLIs such as `aspell`, `lychee`, `typos`, `gh`, or `mdbook-utils`; those tools still need Windows installations and their own compatibility checks. Book-visible scripts under `bk/crates/**/examples/` are content, not repository tooling, and remain out of scope for this migration.
