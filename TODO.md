<!-- TODO Reviewed 10-25-2026 -->

# Task tracking

<!-- recommended-issue-order:start -->

## Recommended issue order

Snapshot: November 3, 2026. Read each list from top to bottom: quick wins first,
then the remaining ready work in priority/dependency order. Quick wins are bounded
candidates, not time estimates. Recheck the live issue and current source before
starting; this index does not mark acceptance complete. Existing notes below are
context and may reference issues that have since closed.

Ready means a concrete engineering slice can start without a new owner decision or
external account/host. Where an issue mixes local work and external acceptance, only
the named local slice is ready. Blocked entries state what releases the block;
fixture preparation and investigation can continue independently.

### Ready: quick wins

No clearly bounded quick win is currently open; start with the priority list below.

### Ready: priority order

- [#2165: Epic: Complete dependency-update coverage and consistent CI caching](https://github.com/john-cd/rust_howto/issues/2165) — Local updater/cache/security dispositions; upstream waits remain under #2116.

### Blocked or deferred

- [#2164: Epic: Validate and finalize Cargo profiles, output paths, and linker policy](https://github.com/john-cd/rust_howto/issues/2164) — Remaining cross-platform/linker acceptance needs the corresponding hosts/CI; completed Windows measurements need no replay.
- [#2151: Validate ignored example-entry tests from #2133 (services, nextest, clippy 1.97 lints)](https://github.com/john-cd/rust_howto/issues/2151) — Real service-backed/native feature environments and the specified nextest checks.
- [#2160: Validate pinned Python and heavy-test service versions](https://github.com/john-cd/rust_howto/issues/2160) — Docker Linux engine, runtime/service images and Oracle registry credentials where required.
- [#2173: Record native-platform CI results after hosted runners
  start](https://github.com/john-cd/rust_howto/issues/2173) —
  Advisory macOS/FreeBSD jobs are still queued; record their results
  when GitHub assigns runners.
- [#2116: Epic: waiting on an upstream release or advisory fix](https://github.com/john-cd/rust_howto/issues/2116) — Compatible fixed upstream dependencies; keep exact release/constraint triggers in the issue.
- [#2113: Epic: human decisions, reviews, and host-dependent validation](https://github.com/john-cd/rust_howto/issues/2113) — Human reviews, accounts, native libraries and host prerequisites.

<!-- recommended-issue-order:end -->

Open work is tracked in [GitHub Issues](https://github.com/john-cd/rust_howto/issues).
Keep one topic per issue, except epics, with a problem, task list, and acceptance criteria.
Record status and validation evidence in the issue rather than duplicating checklists here.

## Blocker tracking

- [Human decisions, reviews, and host prerequisites](https://github.com/john-cd/rust_howto/issues/2113).
- [Upstream release and advisory waits](https://github.com/john-cd/rust_howto/issues/2116).

## Maintenance

- [Validate and finalize Cargo profiles, output paths, and linker policy](https://github.com/john-cd/rust_howto/issues/2164).
- [Complete dependency-update coverage and consistent CI caching](https://github.com/john-cd/rust_howto/issues/2165).

## Briefs for work without GitHub access

Keep implementation briefs self-contained here because Jules cannot read GitHub issue descriptions. Issue links provide traceability; they do not replace the scope or acceptance details.

No open briefs.

## Codebase Audit Fixes and Improvements

A comprehensive review of the monorepo identified the following fixes, improvements, and gaps across testing, documentation, architecture, and tool verification.

### 1. Unit Tests and Coverage

- **Add unit tests for CLI argument parsing across `tools/crate_indices`**:
  - Currently, `tools/crate_indices/src/cli/mod.rs` and `tools/crate_indices/src/cli/commands.rs` have 0 unit tests.
  - Add unit tests verifying subcommand resolution, default values (`--book-root`, `--file`, `--directory`), alias resolution (`c`, `a`, `p`, `l`, `u`, `e`), and error handling on invalid argument combinations.
- **Add unit tests for `tools/templ` CLI subcommands**:
  - Subcommands `badge_cmd`, `category_badge_cmd`, `category_badges_for_crate_cmd`, `config_flag`, `crate_name_arg`, `info_cmd`, and `rbe` lack unit tests for argument parsing and option propagation.
  - Add unit tests in each CLI module verifying flag combinations, defaults, and error branches.
- **Add unit tests for `publish/src/main.rs`**:
  - `publish` has 0 tests. Add unit tests verifying `Cargo.toml` parsing, fallback behavior when workspace inheritance is active, and manifest serialization.
- **Expand helper unit testing across complex example modules**:
  - While all example modules comply with `test()` invoking `main()`, large example modules (e.g., in `bk/crates/cats/database`, `bk/crates/cats/asynchronous`, and `bk/crates/cats/concurrency`) rely solely on end-to-end execution of `main()`.
  - Refactor large `main()` routines into discrete helper functions and add unit tests exercising edge cases, timeouts, and error handling for these helpers.

### 2. Integration Tests

- **Add end-to-end integration test suites for tools in `tools/`**:
  - `tools/clean`, `tools/mdbook-scrub`, and `tools/mdbook-private` contain `tests/` directories, but `tools/crate_indices`, `tools/templ`, and `tools/example-integrity` do not.
  - Implement integration tests in `tools/crate_indices/tests/cli.rs` testing CLI execution of `list_crates`, `category_page_from_metadata`, and `expand_crate_blocks` against isolated test fixtures.
  - Implement integration tests in `tools/templ/tests/cli.rs` asserting markdown badge output formatting and error reporting.
  - Implement integration tests in `tools/example-integrity/tests/cli.rs` verifying JSON schema output structure and `--fail` exit status handling on intentional broken fixture books.

### 3. Examples and Benchmarks

- **Add Criterion / Divan benchmarks to performance-sensitive crates**:
  - Only four crates in `bk/crates` currently define benchmarks (`development_tools_profiling`, `memory_management`, `data_structures`, and `algorithms`).
  - Add benchmarks for high-throughput utilities in `tools/tool_lib` (e.g., `published_recipes` link extraction regexes, crate block parsing, and markdown template generation).
  - Add benchmarks in `tools/example-integrity` measuring chapter parsing, regex anchoring, and module target graph traversal across large book trees.
  - Add benchmarks in `xmpl/cancelable` measuring task submission throughput, channel contention, and cancellation latency under concurrency.
- **Provide standalone runnable examples for reusable libraries**:
  - `tools/tool_lib` is a core library used by all tools but has no `examples/` directory showing how to construct category badges, format crate sections, or parse published recipes.
  - Add minimal example binaries under `tools/tool_lib/examples/` demonstrating programmatic use of the API.

### 4. Documentation and Doc Comments

- **Document public API items and non-main helpers across book examples**:
  - Multiple non-generated book examples contain public functions or types without rustdoc comments (e.g., `bk/crates/cats/authentication/examples/session_login/axum_login.rs`, `bk/crates/cats/web_programming_http_server/examples/axum_extras/axum_server_tls.rs`, and `bk/crates/code_organization/examples/naming_conventions/main.rs`).
  - Add educational doc comments explaining parameters, return values, errors, and concurrency guarantees, adhering to the requirement that all comments end with a period.

### 5. Architecture and Workspace Dependency Isolation

- **Isolate git dependencies in `tools/Cargo.toml`**:
  - `tools/Cargo.toml` depends on `book_tool` via git (`git = "https://github.com/john-cd/book_tooling"`, rev `1e9cc3458f19d53c102aca5da36bc7e8baeac2d3`). Because this repository requires authentication or specific network access, running standard offline builds or checking workspace crates in isolated environments fails.
  - Decouple `mdbook-scrub` from the external git dependency, or vendor the required directive parser functionality directly into `tools/tool_lib` or an internal member crate to achieve hermetic builds.
- **Enforce centralized lint configuration across sub-workspaces**:
  - Ensure all workspaces (`bk/crates`, `later/crates`, `tools`, `xmpl`, `publish`, `playground`) inherit identical workspace-level lints and deny rules from root configurations (`deny.toml`, `clippy.toml`, `rustfmt.toml`).

### 6. Discrepancies between Documentation and Code

- **Reconcile `bk/master/crates.txt` and `bk/master/crate_categories.toml` with actual book recipes**:
  - The book documents `sprs` in `bk/src/categories/mathematics/linear_algebra.md` and links it in `crates_by_category.md` and `crates_alphabetical.md`, but `sprs` was omitted from master files. Audit all crates across `bk/crates` to ensure 100% synchronization between recipe includes, master crate lists, and generated indices.
- **Verify `bin/README.md` against `just release`**:
  - Verify that `just release` recipes build and copy all required executables into `bin/` with clear usage documentation.
