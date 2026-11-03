<!-- TODO Reviewed ??-??-2026 -->

# rust_howto Instructions

## Human review markers

- `TODO Reviewed ??-??-<year>` and `Reviewed <date>` comments are for human review only. Preserve existing markers verbatim during edits; do not remove them or treat them as software TODOs.
- When modifying a file that contains a `Reviewed <date>` marker without `TODO`, preserve that marker and add a separate `TODO Reviewed ??-??-<year>` comment to request fresh human review.

## Working agreement and task tracking

- Keep answers concise, use American spelling, use Unix-style line endings.
- Read this file, the root `README.md`, `TODO.md` when present, the relevant `justfile` and Cargo manifests, and `git status` before editing. Follow the owning component's documentation and existing architecture.
- Keep updates concise and actionable, with links to relevant files. Whenever you change the active branch or worktree, immediately state the current branch name and worktree path in your next update. Include the current issue when ownership changes.
- Preserve unrelated staged, unstaged, and untracked work. Keep edits and commits scoped to the requested task.
- Track implementation work in GitHub Issues. Link acceptance criteria to source and validation evidence; record blockers and concrete remaining work on the issue. Use priorities `P0` through `P4` where the tracker supports them.
- Reconcile actionable `TODO` and `LATER` entries with issues instead of maintaining duplicate backlogs. Do not delete unresolved comments or rename a `TODO` to a `NOTE` to hide unfinished work.
- Close an issue only after its acceptance criteria, required checks, and focused commits are complete. A merged PR alone is insufficient evidence.

## Git workflow

- For issue work, create at least one commit per issue. Every commit must address only one issue; never combine multiple issues in a commit. An issue may have multiple focused commits.
- Keep the primary checkout on `develop`; use short-lived topic branches in clean, inactive worktree slots for implementation. Prefer `codex/issue-<number>-<description>` for issue work.
- Always use elevated access for Git operations that create or change worktrees, fetch from remotes, or otherwise write repository metadata or remote refs. This includes creating or deleting branches, commits, rebases, cherry-picks, merges, and pushes. Read-only Git inspection does not need elevation unless the sandbox denies access to submodule metadata; retry the exact inspection with elevated access in that case.
- On Windows, Just recipes launch Git Bash. If Git Bash exits immediately with `0xC0000142` in the managed sandbox, retry the recipe with elevated process access. The repository's Git Bash installation and submodule ACLs do not need changes when the same commands work elevated; investigate them only if the failure persists outside the sandbox.
- Keep history linear. Rebase topic branches onto current `develop` before integration; use fast-forward integration or squash-merge PRs according to the requested workflow. Do not create merge commits to synchronize `develop`.
- Inspect local and remote history before integrating. Fast-forward when only behind; rebase unpublished local commits when histories diverge. Never rewrite shared history; use `--force-with-lease` only for an authorized update to an unshared rebased topic branch.
- Preserve the repository's existing independent `main` history. Do not push directly to `main`.
- Create at least one commit for every GitHub issue. Each commit must address only one issue; never combine changes for multiple issues in a single commit. An issue may have multiple focused commits when needed.
- Use focused commits with `type(scope): imperative summary` (`feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `build`, `ci`, `chore`, or `style`). Include the related GitHub issue or PR link and short bullets describing the changes.
- Compare content and ancestry before retiring topic branches. Keep reusable worktree directories and their caches; do not discard unique work.

## Build, test, and lint

This repository is a monorepo with several independent Rust workspaces. Run commands from the matching workspace instead of assuming the repo root is a Cargo workspace.

| Area                          | Working directory | Build / lint / test                                                                           | Run one test                                            |
| ----------------------------- | ----------------- | --------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| Main book examples            | `bk\`             | `just ba`, `just ca`, `just nta`                                                              | `just ntt <keyword>`                                    |
| Book rendering                | `bk\`             | `just bb` to build the book, `just s` to serve it, `just q` for a faster preview path on Unix | n/a                                                     |
| Future/staging crates         | `later\`          | `just ba`, `just ca`, `just nta`                                                              | `cargo test -p <crate> <test_name>` from `later\crates` |
| Helper binaries               | `tools\`          | `just ba`, `just ca`, `just nta`                                                              | `cargo test -p <crate> <test_name>`                     |
| Additional long-form examples | `xmpl\`           | `just ba`, `just ca`, `just nta`                                                              | `cargo test -p <crate> <test_name>`                     |
| Scratch workspace             | `playground\`     | `just ba`, `just ca`, `just nta`                                                              | `cargo test -p <crate> <test_name>`                     |
| Placeholder published crate   | `publish\`        | `just ba`, `just ca`, `just nta`                                                              | `cargo test <test_name>`                                |

Additional repo-specific commands:

- `just all <cmd>` at the repo root fans out to the main subprojects.
- `just release` in `tools\` builds the helper binaries and copies them into the repo-level `bin\` directory.
- `rustfmt-toolchain.toml` is the nightly formatter pin used by the workspace `just` recipes. Use `just fmtall` to format a workspace.

## High-level architecture

This repo is primarily an mdBook publishing repo with Rust workspaces attached to it.

- `bk\` is the canonical published book. `bk\src\SUMMARY.md` defines the book structure, and `bk\book.toml` wires the mdBook preprocessors and outputs.
- `bk\crates\` is a separate Cargo workspace containing the Rust examples embedded in the book. Crates are grouped either by book section (`language`, `standard_library`, `code_organization`, etc.) or by crates.io category under `bk\crates\cats\...`.
- `tools\` is a separate Rust workspace for repo-specific helper binaries. `mdbook-scrub` is built there and consumed by `bk\book.toml` through the repo-level `bin\` directory. `tool_lib` is the shared library used by the tool crates.
- `mdbook-utils\` is a companion CLI/library project used to manage links, reference definitions, code block extraction/includes, and sitemap generation for large mdBook source trees.
- `later\src\` and `later\crates\` mirror the book/example structure for work-in-progress or future content that is not yet part of the main published book.
- `xmpl\` holds standalone examples that are too large to embed directly in the book, `playground\` is for exploratory code, and `publish\` is the small placeholder crate published to crates.io so the book is discoverable there.

## Worktrees and compilation caches

- `develop` is the default integration branch. `main` starts with an empty root commit and has independent history. The primary checkout keeps its stable `main` cache identity so the branch rename does not invalidate compiled artifacts.
- Keep the primary checkout at `D:\code\rust_howto` on `develop`, using cache slot `main`. The permitted topic worktrees are `D:\code\rust_howto\.worktrees\a`, `D:\code\rust_howto\.worktrees\b`, and `D:\code\rust_howto\.worktrees\c`. Inspect `git worktree list` and each slot's status before choosing one; preserve dirty or active slots. Do not create additional worktrees. `main` is the primary checkout's cache identity, not a folder under `.worktrees`.
- Reuse a clean, inactive `a`, `b`, or `c` slot so its Cargo target and intermediate build directories stay warm. The build runner supports all three slots; if they are unavailable, ask before creating or using another slot.
- On Windows, build artifacts belong in CODE_TARGET_ROOT\rust_howto\<slot> and intermediate artifacts in CODE_BUILD_ROOT\rust_howto\<slot>, where slot is main, a, b, or c. On Linux/macOS, use the XDG cache folder (or ~/.cache) under rust_howto/target/<slot> and rust_howto/build/<slot>. Keep these directories across branch changes to reuse compiled dependencies.
- Every workspace justfile imports scripts/build.just, which detects the slot automatically and sets external per-checkout target/build paths. No RUST_HOWTO_WORKTREE_NAME variable is needed.
- Use `just cargo <subcommand> <args...>` from the owning workspace's justfile for ad hoc Cargo commands; the shared recipe forwards arguments while retaining its configured build environment. The repository root, `bk`, and `later` are not Cargo workspace roots themselves, so pass `--manifest-path` for the intended workspace when invoking from those justfile directories.
- Run `just build-setup` once per checkout before direct Cargo commands. On Windows it creates ignored `target` and `build` junctions to that checkout's external directories and on Linux it creates symlinks to the Linux cache directories. An ignored `.sccache` link points at the shared compiler cache. Setup refuses to replace existing files or directories. Use separate native Windows and Linux checkouts rather than sharing their platform-specific links. The root `.cargo/config.toml` supplies these paths to all independent workspaces; their local configs retain workspace-specific options. The existing `a` and `b` slots were set up by `just worktree`.
- On Windows, set `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` to the absolute external paths above for direct Cargo commands. A direct Cargo build through the checkout junctions failed with Windows device error 650 in this environment; using the external paths succeeded.
- Build speed rules, measured on Windows (20 cores, tools workspace, `sccache` 0.18):
  - Keep a small number of worktree slots and reuse them. A warm slot rebuilds after an edit in 5-8 s; a cold or reset slot rebuilds everything (about 90-200 s for `tools`, far longer for `bk/crates`). Do not open a new slot when a clean, merged one exists; retire clean, merged slots promptly so build directories do not pile up on `C:` and `D:`.
  - Direct Cargo and workspace `just cargo` both use sccache with incremental compilation disabled. Both modes obey the same cache/incremental pairing; use the workspace justfiles for known central output paths.
  - Previous local measurements found cold builds slower with sccache, while warm incremental builds benefit from Cargo incremental compilation. Keep the configured sccache/incremental pairing consistent across direct Cargo and the runner unless new measurements justify changing both.
  - The shared sccache limit is 35G. Just sets `SCCACHE_BASEDIRS` to the main checkout and registered `.worktrees/a`, `.worktrees/b`, and `.worktrees/c` slots only. `just cache-setup` supplies the same list to the daemon and discards inherited paths.
  - Dev profiles use `debug = "line-tables-only"` (and `debug = 0` for dependencies): this halved a cold build (196 s to 90 s) and the target directory size (1955 MB to 980 MB). `bk/crates` compiles workspace code at `opt-level = 0` and dependencies at `opt-level = "s"`. Linking with `rust-lld` on `x86_64-pc-windows-msvc` showed no gain; keep the default linker.
  - Day to day, check, lint, and test the package you are editing instead of the whole workspace: `just bk ckp <pkg>`, `just bk cap <pkg>`, `just bk ntp <pkg>` (or `ckc`/`ntc` from inside the crate). Reserve `cka`, `ca`, and `nta` (all packages, all features) for delivery.
  - `SCCACHE_DIR` defaults to `CODE_SCCACHE_ROOT` on Windows and `${XDG_CACHE_HOME:-$HOME/.cache}/sccache` on Linux, while respecting an existing environment override. Preserve the shared cache directory when cleaning. See [sccache's Rust guidance](https://github.com/mozilla/sccache#rust).
- Before cleaning `CODE_TARGET_ROOT`, inspect registered worktrees and running Cargo/rustc processes. Preserve active outputs, reusable slot caches, and the shared sccache directory. Remove obsolete task-named outputs only after establishing that no worktree uses them. Never run a recursive cleanup through the checkout's junctions.

## Key conventions

- Book content is include-heavy. Many chapter files are thin wrappers that include a sibling `*.incl.md` file plus shared reference-definition files. Preserve that structure instead of inlining generated tables or link refs.
- When adding or moving book content under `bk\src\`, also update `bk\src\SUMMARY.md`; the book structure is driven from there.
- Markdown links are often centralized through `refs.incl.md` or `refs\link-refs.md` instead of inline URLs.
- Embedded Rust examples usually live under `bk\crates\<section-or-category>\examples\<chapter>\`. A chapter-level `main.rs` pulls recipe files in via `mod ...;`.
- Example files normally expose only the book-visible snippet between `// ANCHOR: example` and `// ANCHOR_END: example`, then add hidden `pub fn run() { main(); }` and `#[test] fn test() { main(); }` so the snippet is continuously exercised by workspace tests and is accessible from the root module.
- The `bk\justfile` is the main contributor entry point. It delegates to `bk\scripts\...\mod.just` modules for code, book, links, refs, examples, indices, and related maintenance tasks; prefer those recipes over inventing new ad hoc commands.
- On Windows, `rust_howto`-owned `just` recipes use Git Bash. PowerShell helpers are invoked explicitly with `pwsh.exe`; prefer `just` recipes over ad hoc shell commands.

## Rust design and code organization

- Prefer direct, readable control and data flow, one owner for each rule or state, and acyclic crate dependencies. Lower-level libraries must not depend on CLI composition roots.
- Keep `main.rs` thin and reusable behavior in libraries or focused modules. Avoid pass-through wrappers, speculative traits, stateless namespace structs, and abstractions without a demonstrated boundary.
- Name modules after their owning type, behavior, or domain; avoid generic names such as `types.rs`, `models.rs`, or `implementation.rs`. Keep one primary type or closely related group of functions per file.
- Use folder modules with `mod.rs` when modules have children. Keep `lib.rs` and `mod.rs` primarily declarative; put substantial implementations in focused child modules. Keep nested modules private and re-export only required items owned by the crate.
- Do not use `#[path = "..."]` to arrange modules. Update callers when moving or renaming code; remove superseded implementations and orphan source files in the same change.
- Order module documentation and attributes before declarations, then imports, constants/statics, types and implementations/functions, with inline tests last. Sort sibling module declarations alphabetically and let Rustfmt order imports.
- Prefer `use` imports over long qualified paths unless qualification prevents ambiguity. Import external items from their defining crate instead of re-exporting them.
- Use domain types for identities and values with invariants. Keep behavioral state private; expose constructors and narrow accessors or mutation methods. Passive configuration, event, result, and serialization records may have public fields.
- Load application configuration and environment variables at the entry point and pass typed configuration into reusable code.
- In actual Cargo workspaces, declare shared dependencies in `[workspace.dependencies]` and inherit them with `workspace = true`. Keep standalone package dependencies in their own manifest. Preserve declared Rust versions and feature choices unless the task requires a change.

## Errors, IO, and observability

- Propagate fallible operations with `Result` and `?`, adding actionable context. Do not swallow errors, ignore failed writes, or substitute default values for failed computations.
- Do not use `unwrap()` in maintained Rust code, including tests and benchmarks, or `panic!` in production code. Use `expect()` only for a documented invariant, or with a descriptive message in tests and benchmarks.
- Prefer specific error types with `thiserror` in libraries and `anyhow` at CLI/application boundaries. Preserve the owning crate's established public error contracts when making a focused change.
- Use `tracing` for diagnostics and instrument meaningful orchestration boundaries; avoid unconditional spans or allocation in tight loops.
- Write replacements through temporary sibling files and atomic rename where supported. Preserve source data and report partial failures accurately.
- Prefer existing maintained crates and measured improvements. Preallocate known sizes and reuse hot-loop buffers when useful; do not add speculative performance machinery.

## Documentation, tests, and formatting

- Document every module and public API, including fields and variants. Explain ownership, invariants, side effects, and concrete error conditions; document non-obvious private helpers and algorithms.
- Add a specific doc comment to every function and method, including private helpers and tests, except standard trait implementations whose contracts are already defined by the trait.
- Keep Rustdoc specific to the item. Put doc comments before attributes; avoid generic placeholder text and document fallible APIs with meaningful `# Errors` sections.
- Add focused behavior tests for new code and bug fixes, integration tests for CLI/file effects, and benchmarks when performance-sensitive behavior warrants them.
- Keep unit tests near their code in an inline `#[cfg(test)] mod tests` at the bottom of the file. Do not wrap integration-test files in `#[cfg(test)]`.
- Give tests descriptive names and a concise doc comment stating the behavior they verify. Construct concrete fixtures and meaningful identifiers; review snapshots before accepting changes.
- Keep README files, user guides, command examples, and configuration documentation synchronized with changed behavior.
- Use American spelling, LF line endings, and CommonMark. Leave blank lines after headings and specify a language on fenced code blocks.
- Prefer ASCII in tooling code and agent instructions; preserve intentional Unicode in book content, examples, and fixtures.
- Format Rust with the repository's nightly Rustfmt recipe. Use `dprint` for Markdown/JSON when configured. Format Cargo manifests with `just fmt-toml`; check them with `just fmt-toml-check` (both use `cargo fmt-toml`).
- Fix lint causes instead of hiding them. Use `#[expect(lint, reason = "...")]` for intentional, documented exceptions rather than unexplained suppression.
- Prefer Rust for new permanent processing tools. Retain established shell entrypoints and platform/build setup scripts; do not introduce permanent Python or Bash programs for application logic.
- Prefer `cargo binstall` for supported development tools and pin CI tool versions.

## Validation and resource use

- Select checks by impact: documentation-only changes need applicable Markdown/link checks; Rust behavior changes need focused crate and behavior tests; dependency, feature, or shared-interface changes need affected consumers and configurations.
- Use the owning repository's `just` recipes. Do not assume another repository's `cargo make`, UI checks, or release gates exist here.
- Use focused checks in the inner loop; run the relevant complete gates before integration or delivery. Include doctests, examples, and required feature variants when they cover affected behavior.
- Do not run `cargo check` immediately before equivalent Clippy checks. Reuse passing evidence while the relevant inputs and configuration remain unchanged.
- Record commands, configurations, results, and remaining failures. Use read-only validation; apply fixes deliberately, inspect their diff, and preserve unrelated work.
- Run builds for a checkout sequentially. Independent checkouts may build concurrently when CPU, memory, and disk capacity permit; avoid changing inputs during a build.
- Inspect running Cargo/rustc processes before starting expensive builds or cleaning artifacts. Do not stop another task's processes or restart the shared sccache server while builds are active.
- Use `CODE_TEMP_ROOT` for Windows temporary files where an override is needed. Keep temporary files separate from persistent target, intermediate-build, Cargo, and sccache caches.
- Finish with `git diff --check`. Report unavailable checks and pre-existing failures without attributing them to the current change.

These Rust design and error-handling rules govern maintained tools and reusable libraries. Book-visible snippets may intentionally demonstrate `unwrap`, panics, Unicode, or a different module shape when that is the lesson; keep the example's behavior explicit and preserve the documented anchor and runner structure.

## Central build outputs and worktrees

- Keep auxiliary Git worktrees directly under `.worktrees/` in slots `a`, `b`, or `c` only; keep the primary checkout separate.
- Keep the main worktree on `develop`. Keep topic branches in auxiliary worktrees.
- Windows: target outputs go to CODE_TARGET_ROOT/rust_howto/<slot> and intermediate build outputs go to CODE_BUILD_ROOT/rust_howto/slot. Keep CODE_BUILD_ROOT on an NTFS volume; do not use a ReFS volume for builds.
- Linux/macOS: target outputs go to the XDG cache folder (or ~/.cache) under cargo-target/rust_howto/<slot>; intermediate outputs go under cargo-build/rust_howto/<slot>.
- Run ad hoc commands as just cargo <subcommand> [args...]; existing cargo-make tasks also receive the configured paths.
- Direct Cargo and workspace justfiles use sccache with incremental compilation disabled by `.cargo/config.toml`. Both paths keep Cargo outputs in the external directories above. Prefer the justfiles for ad hoc Cargo commands so they select the worktree's external paths.
- The Just runner sets `SCCACHE_BASEDIRS` to the main checkout and registered `.worktrees/a`, `.worktrees/b`, and `.worktrees/c` slots only. The cache setup command supplies the same list to the daemon and discards inherited paths. Use the runner's Cargo command to select external per-worktree paths.

## Machine storage roots

- On Windows, use `CODE_TEMP_ROOT` for temporary and staging files, setting `TEMP`, `TMP`, and `TMPDIR` as needed. Use `C:\temp` only when `CODE_TEMP_ROOT` is unavailable.
- Keep package-manager data, library caches, and package installations under `CODE_PACKAGES_ROOT`, including Cargo, npm, NuGet, pip/uv, vcpkg, and similar data. Do not store standalone tool distributions there.
- Keep portable developer tools and SDKs under `CODE_TOOLS_ROOT`, in a named subdirectory per tool (for example NASM, Oracle Instant Client, Cap'n Proto, Flutter, Pandoc, Tectonic, and repository tool bundles). Update PATH and installer/config references when relocating a tool. Leave package-manager-managed tools in their managed roots unless replacing them with a portable installation.
- In WSL, keep temporary files on the distribution's configured temp paths. Keep package data in `CODE_PACKAGES_ROOT` (default `~/.local/share`) and portable tools in `CODE_TOOLS_ROOT` (default `~/.local/tools`); avoid Windows drive mounts for these roots unless a tool requires them.

## Portable storage-root configuration

The current Windows defaults below can be overridden per machine or shell with environment variables. The per-repository Rust Justfile or Cargo Make helper, when present, reads these variables. Non-Rust repositories do not need a Cargo build runner; use the environment roots only for tools or scripts that consume them.

- CODE_REPOS_ROOT: D:/code on Windows; ~/code by default on Unix.
- CODE_BUILD_ROOT: C:/code_build on Windows; XDG cache or ~/.cache/cargo-build by default on Unix. Keep Windows build output on NTFS.
- CODE_TARGET_ROOT: D:/target on Windows; XDG cache or ~/.cache/cargo-target by default on Unix.
- CODE_SCCACHE_ROOT: D:/sccache on Windows; XDG cache or ~/.cache/sccache by default on Unix.
- CODE_TEMP_ROOT: D:/temp on Windows, with C:/temp as a fallback where a script needs one; Unix uses the existing TMPDIR or /tmp.
- CODE_PACKAGES_ROOT: D:/packages on Windows; ~/.local/share by default on Unix.
- CODE_TOOLS_ROOT: C:/tools on Windows; ~/.local/tools by default on Unix.

Windows drive paths elsewhere in this file are defaults. Set a root variable to relocate the corresponding directory. WSL must keep its distribution temp behavior and must not redirect temp variables to a Windows drive mount.

## Git Hooks

- Use prek with this repository's prek.toml; install Git hook shims with hooks-install (or prek install directly).
- Use `prek` and the repository `prek.toml` for Git hooks.

## Python projects and uv

Python project directories in this repository:

- `later/crates/cats/development_tools_ffi/python_extension`

- Keep each standalone Python project on a committed `uv.lock` and `.python-version`. Workspace members share the workspace root lock and Python pin.
- Run commands from the repository root through `just --justfile <path>/justfile`: append `env`, `uv sync --locked`, or `uv run <command>`. Formatting and lint tasks use a separate `<slot>-lint` environment with only the locked Ruff group and do not build the extension. Install the hook tool with `just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile uv tool install prek`; install and run hooks with `just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile hooks-install` and `just --justfile later/crates/cats/development_tools_ffi/python_extension/justfile hooks`.
- The Justfile assigns project/worktree-specific environments under `CODE_PYTHON_ENV_ROOT`, Python caches under `CODE_PYTHON_BUILD_ROOT`, generated outputs under `CODE_PYTHON_OUTPUT_ROOT`, package-manager data under `CODE_PACKAGES_ROOT`, and tool executables under `CODE_TOOLS_ROOT`. Override these roots through environment variables.
- On Windows the defaults are `D:/packages/uv/environments`, `D:/temp/python-build`, `D:/temp/python-output`, `D:/packages`, `C:/tools`, and `D:/temp` (with `C:/temp` fallback). Unix-like defaults use native home directories and `TMPDIR` or `/tmp`.
- `CODE_BUILD_ROOT`, `CODE_TARGET_ROOT`, and `CODE_SCCACHE_ROOT` are Rust/Cargo-specific; the Python Justfile does not read or set them. Use the repo's Cargo runner for Rust builds.

## Required repository storage and Git policy

- Keep the primary checkout on `develop`. Keep `main` as an independent empty root commit; do not merge development history into it.
- Auxiliary worktrees must be direct children of `.worktrees/` and use slots `a`, `b`, or `c` only. Never use task-named or external worktree paths.
- Preserve unique commits on named branches before moving a detached checkout. Keep history linear, squash unpublished related changes, and never rewrite published history without explicit authorization.
- Use `CODE_TEMP_ROOT` for Windows temp files: `D:/temp` when available, otherwise `C:/temp`. Unix uses its native `TMPDIR` or `/tmp`.
- Use `CODE_PACKAGES_ROOT` for package data (Windows `D:/packages`) and `CODE_TOOLS_ROOT` for portable tools (Windows `C:/tools`). Existing tool-specific environment overrides take precedence; changing a root does not move installed packages.
- Run Cargo through `just cargo`. The runner must export central per-repo/per-slot target and build paths, sccache with incremental disabled, and only registered allowed sccache base directories.
- Windows intermediate builds must use NTFS. The runner validates `CODE_BUILD_ROOT` before Cargo starts; a ReFS override is an error. Defaults are `C:/code_build/<repo>/<slot>` and `D:/target/<repo>/<slot>`.
- Run `just configure-cargo-host` once per host and after changing roots to configure direct Cargo's central fallback. Run the build-setup recipe per checkout. Direct Cargo and rust-analyzer then use that checkout's target/build links; the host config remains a fallback for repositories without local output settings.

## Configuration consistency

Follow [CONFIGURATION.md](CONFIGURATION.md) when changing repository tooling. Use American
spelling and LF line endings; keep README, contributor instructions, and configuration comments
synchronized. Preserve documented platform, domain, and teaching-example exceptions.

## Required worktree workflow

- Keep the primary checkout (`main` worktree) on `develop`. Do task work in the primary checkout or in direct
  children of `.worktrees/` named `a`, `b`, or `c`. Never use task-named directories, other slots, or
  external locations. Reuse a clean, inactive permitted slot whenever possible.
- Before reusing a slot, inspect `git worktree list`, branch, status, ownership, and active build
  processes. Never reset, overwrite, move, or remove a dirty or active worktree. If no permitted slot is
  available, stop and ask; do not create another slot or custom path.

- For each issue or task, implement in its worktree, lint the changes, format the changed files, and
  make a focused commit. For issue work, make at least one commit per issue and keep every commit
  limited to that single issue. Do not combine multiple issues in one commit. Keep the worktree
  clean; make a WIP commit if needed. Wait until the task is
  complete to integrate into `develop` linearly. Then lint and format the whole repository and commit
  integration fixes. Do not leave dirty worktrees.
- Detach a clean, unused worktree and retain its slot and build directory. Delete topic branches only
  after confirming they are fully merged; never delete `develop`.
- Always keep the primary checkout's build directory. Preserve build directories for slots `a`, `b`, and
  `c` when disk space allows. When disk space is low, inspect active worktrees and build processes; remove
  the oldest obsolete build directories first. Then clean task-owned temporary directories and obsolete
  `build`/`target` directories. Do not remove directories used by active checkouts or processes.
- `cargo-deny` findings are not a blocker gate. Fix them, add a justified exception, or create a follow-up
  issue or task and record the outcome. Other OS or Docker validation is not a blocker gate either;
  create a follow-up issue or task when that validation is unavailable.
