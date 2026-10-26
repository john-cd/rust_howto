# Proposal: make every command work on Linux and Windows

Status: proposal only, nothing here is implemented. Scope: `rust_howto` (the `justfile`s, `bk/scripts/**`, `scripts/`, `.devcontainer/`), with notes on `book_tooling` and `mdbook-utils`.

## What exists today

- About 1,970 lines of `justfile` / `*.just` across 7 workspaces and 23 `bk/scripts/*` modules.
- 39 `.sh`, 4 `.ps1` and 6 `.py` files outside the example crates.
- Every justfile sets `shell := bash` and `windows-shell := pwsh.exe`, so recipe bodies run in two different shells.
- About 70 `[unix]` and 24 `[windows]` attributes. Recipes are duplicated per OS only in `book`, `docs`, `precommit`, `recipes`, `utils` and a few others. These modules have **only `[unix]` recipes and no Windows counterpart**: `anchors`, `chapters`, `crates`, `examples`, `gh`, `includes`, `spelling`, `titles`, `toc`, `urls`, `wikilinks` (plus `just q`, which explicitly refuses to run on Windows).
- The unix scripts lean on `grep -P`, `comm`, `sed -r`, `mktemp`, `realpath`, `/tmp`, `rm --recursive` and `aspell`/`lychee`.
- Recipes written in PowerShell syntax with no `[windows]` guard (for example `$env:RUSTC_WRAPPER = ""` in `clippyall` and `nextestall` in `bk/scripts/code/mod.just`) will most likely fail under bash on Linux. I did not run them.
- `scripts/build.just` already hard-codes `D:/target`, `C:/code_build` and `D:/sccache` on Windows, and `setup-build.sh` / `setup-build.ps1` are two ports of the same ~90 lines.
- CI already runs on `ubuntu`, `windows-2025` and `macos-15`, and the repo already ships Rust helper binaries (`tools/`, `bin/clean.exe`, `mdbook-utils`) and one Python script (`refdefs.py`).

The simple recipes (`cargo ...`, `just code ...`) already work on both OSes. The pain is concentrated in the roughly 40 shell scripts and the duplicated `[unix]`/`[windows]` pairs.

## Options compared

| | (a) Keep just, fix recipes | (b) Rewrite in cargo-make | (c) Rust tool / xtask | (d) Other: Python (`uv`) or one shell everywhere |
|---|---|---|---|---|
| Idea | One shell for all OSes plus OS-specific recipes where unavoidable | Move all tasks to `Makefile.toml` | Port script logic to a Rust CLI called from thin recipes | Port scripts to Python or standardise on pwsh/bash |
| Effort | 2-4 days | 2-3 weeks | 1-2 weeks, incremental, per script | 1-2 weeks, incremental |
| Cross-platform quality | Good for one-liners, poor for the 40 script bodies | Good, but cargo-make shell tasks still shell out | Best: one implementation, testable, no quoting issues | Good; needs Python or pwsh installed |
| Fit with this repo | Excellent: modules, `JUST_VERSION` pin, AGENTS.md, CI, devcontainer all assume just | Poor: loses `mod` namespaces, `just --list` groups, and everything above must be rewritten | Excellent: `tools/` workspace, `tool_lib`, `bin/` release flow and `mdbook-utils` already exist | Fair: Python is already used once; no new build step |
| Main risk | Behavioural drift between bash and pwsh; Git-Bash vs WSL `bash.exe` ambiguity on Windows | Big-bang migration, new DSL and Duckscript to learn, docs and CI rewrites | Compile time before first use; needs the `bin/` release step | Extra runtime to install; two languages to maintain |

Notes on each:

- **(a)** Options inside just: `set shell := ["pwsh", "-NoLogo", "-NoProfile", "-Command"]` on all OSes (PowerShell 7 runs on Linux/macOS), or `set windows-shell := ["bash", "-uc"]` when Git for Windows is present. Using one shell removes the per-OS recipe pairs, but pwsh and bash differ on quoting, `&&`, env vars and globbing, and bash-only tools (`grep -P`, `comm`) still do not exist on Windows. Also consider just's `[script(...)]` attribute (check it is stable in the pinned just 1.59) for multi-line bodies, plus `os_family()` ternaries for paths such as `build.just`. This fixes the glue but does not fix the scripts.
- **(b)** cargo-make adds a second task runner, different variable and condition syntax, and Duckscript. It does not remove the need for portable script bodies, so it costs the most and solves the least here.
- **(c)** Same approach as `bin/clean.exe`: add a `tools` subcommand (or a small `xtask`-style binary) per script, for example `refdefs check`, `urls check`, `links inline`, `toc missing`, `setup-build`. Recipes become one-line calls like `cargo run -p tool -- refdefs check {{root}}`. Plain `cargo xtask` is a weaker fit because this is a monorepo with seven independent Cargo workspaces, so a single workspace-level alias would not be reachable from all of them; living in `tools/` and releasing to `bin/` already matches the existing pattern. Much of the logic (reference definitions, links, includes, sitemap) already lives in `mdbook-utils` and `book_tooling/parse_lib`, which would be reused rather than rewritten.
- **(d)** Python via `uv run` is a reasonable choice for text-munching scripts if Rust feels heavy, and `refdefs.py` already sets the precedent. External CLIs (`lychee`, `aspell`, `dprint`, `rumdl`) are a separate matter: they need install docs for each OS regardless of option.

## Recommendation

Do (a) now, then migrate scripts to (c) gradually. Do not adopt cargo-make.

1. **Quick wins (about 1 day).** Pick one `shell` per justfile (recommend pwsh 7 everywhere because the repo already requires `pwsh.exe` on Windows, or bash everywhere if Git Bash is acceptable; decide once). Fix the unguarded `$env:` recipes. Remove hard-coded drive letters from `build.just` in favour of overridable env vars. Add a CI job that runs `just --list` and `just --dry-run` on both OSes to catch broken recipes early.
2. **Consolidate duplicated pairs (about 2 days).** Replace `[unix]`/`[windows]` pairs that differ only in syntax (`copystatic`, `sitemap`, `mdbook build`, `clean`) with a single recipe in the chosen shell.
3. **Port scripts to Rust or Python (1-2 weeks, spread out).** Start with the ones that are windows-less and used often: `refdefs/check_refdefs.sh`, `urls/check_urls.sh`, `toc`, `includes`, `links/*`, `setup-build.*` (merge the two ports into one program). Leave one-off scripts (docker, `gh`, benchmarking examples under `bk/crates/**`) as shell, documented as Linux/macOS/WSL-only.
4. **Revisit** after step 3. If most recipes are then one-line calls, the remaining question is only which shell runs them.

Risks and open questions for John: which shell to standardise on, whether WSL counts as "Windows support" for the long tail of scripts, and whether the `bk/crates/**/examples/*.sh` files (they are book content, not tooling) are in scope. I treated them as out of scope.
