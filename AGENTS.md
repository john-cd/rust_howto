<!-- TODO Reviewed ??-??-2026 -->

# rust_howto Instructions

## Working agreement and task tracking

- Read this file, the root `README.md`, `TODO.md` when present, the relevant `justfile` and Cargo manifests, and `git status` before editing. Follow the owning component's documentation and existing architecture.
- Keep updates concise and actionable, with links to relevant files. State the current issue, branch, and checkout when ownership changes.
- Preserve unrelated staged, unstaged, and untracked work. Keep edits and commits scoped to the requested task.
- Track implementation work in GitHub Issues. Link acceptance criteria to source and validation evidence; record blockers and concrete remaining work on the issue. Use priorities `P0` through `P4` where the tracker supports them.
- Reconcile actionable `TODO` and `LATER` entries with issues instead of maintaining duplicate backlogs. Do not delete unresolved comments or rename a `TODO` to a `NOTE` to hide unfinished work.
- Preserve human review comments and `TODO Reviewed` markers; those markers are reserved for human review.
- Close an issue only after its acceptance criteria, required checks, and focused commits are complete. A merged PR alone is insufficient evidence.

## Git workflow

- Keep the primary checkout on `develop`; use short-lived topic branches in clean, inactive worktree slots for implementation. Prefer `codex/issue-<number>-<description>` for issue work.
- Keep history linear. Rebase topic branches onto current `develop` before integration; use fast-forward integration or squash-merge PRs according to the requested workflow. Do not create merge commits to synchronize `develop`.
- Inspect local and remote history before integrating. Fast-forward when only behind; rebase unpublished local commits when histories diverge. Never rewrite shared history; use `--force-with-lease` only for an authorized update to an unshared rebased topic branch.
- Preserve the repository's existing independent `main` history. Do not push directly to `main`.
- Use focused commits with `type(scope): imperative summary` (`feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `build`, `ci`, `chore`, or `style`). Include related GitHub issue or PR links and short bullets describing the changes.
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

- Keep the primary checkout at `D:\code\rust_howto` on `develop`. Use known reusable slots `D:\code\rust_howto\.worktrees\a`, `b`, `c`, and so on for topic branches. Inspect `git worktree list` and each slot's status before choosing one; preserve dirty or active slots.
- Create a new slot with `just worktree a codex/topic`. Existing slots are intentionally preserved: inspect their status and branch, then switch a clean inactive slot to the next topic instead of removing and recreating it. Do not use task, issue, branch, or random names for worktree directories. Worktree slots are exactly the single-letter folders `a`, `b`, `c` ... `z` directly inside `.worktrees` (for example `.worktrees\a`, never `.worktrees\a\work`, `a-work` or `checkout`); nothing else may live in `.worktrees`.
- On Windows, build artifacts belong in `D:\target\rust_howto\<slot>` and intermediate artifacts in `C:\code_build\rust_howto\<slot>`, where `<slot>` is `main`, `a`, `b`, etc. On Linux, the defaults are `${XDG_CACHE_HOME:-$HOME/.cache}/rust_howto/target/<slot>` and `${XDG_CACHE_HOME:-$HOME/.cache}/rust_howto/build/<slot>`. Keep these directories across branch changes to reuse compiled dependencies.
- Every workspace `justfile` imports `scripts/build.just`, which detects the slot automatically. Explicit `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` environment overrides remain supported, including CI and containers. No `RUST_HOWTO_WORKTREE_NAME` variable is needed.
- Run `just build-setup` once per checkout before direct Cargo commands. On Windows it creates ignored `target` and `build` junctions to that checkout's external directories and on Linux it creates symlinks to the Linux cache directories. An ignored `.sccache` link points at the shared compiler cache. Setup refuses to replace existing files or directories. Use separate native Windows and Linux checkouts rather than sharing their platform-specific links. The root `.cargo/config.toml` supplies these paths to all independent workspaces; their local configs retain workspace-specific options. New slots created by `just worktree` are set up automatically.
- Use `sccache` as `RUSTC_WRAPPER` for all builds. Incremental compilation is disabled so Rust library compilations can be cached. When a command must disable sccache (`RUSTC_WRAPPER=''`), set `CARGO_INCREMENTAL=1` to speed up local recompilation. `SCCACHE_DIR` defaults to `D:/sccache` on Windows and `${XDG_CACHE_HOME:-$HOME/.cache}/sccache` on Linux, while respecting an existing environment override. Use sccache 0.18 or newer for the configured base-directory normalization; older versions can compile but may ignore `SCCACHE_BASEDIRS`. `SCCACHE_BASEDIRS` is the absolute current checkout root, so main and slot source paths normalize consistently; Cargo sets it even when a stale machine-wide value exists. Run `just cache-setup` once after changing these settings or starting a daemon with old base directories; it preserves existing project bases, registers all slots, and refuses to restart during Cargo/rustc builds. See [sccache's Rust guidance](https://github.com/mozilla/sccache#rust).
- Before cleaning `D:\target`, inspect registered worktrees and running Cargo/rustc processes. Preserve active outputs, reusable slot caches, and the shared sccache directory. Remove obsolete task-named outputs only after establishing that no worktree uses them. Never run a recursive cleanup through the checkout's junctions.

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
- Format Rust with the repository's nightly Rustfmt recipe. Use `dprint` for Markdown/JSON when configured, and sort changed Cargo manifests with `cargo sort -w`.
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
- Use `D:\temp` for Windows temporary files where an override is needed. Keep temporary files separate from persistent target, intermediate-build, Cargo, and sccache caches.
- Finish with `git diff --check`. Report unavailable checks and pre-existing failures without attributing them to the current change.

These Rust design and error-handling rules govern maintained tools and reusable libraries. Book-visible snippets may intentionally demonstrate `unwrap`, panics, Unicode, or a different module shape when that is the lesson; keep the example's behavior explicit and preserve the documented anchor and runner structure.
