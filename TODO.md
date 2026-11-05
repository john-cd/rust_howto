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

- **Expand helper unit testing across complex example modules**:
  - While all example modules comply with `test()` invoking `main()`, large example modules (e.g., in `bk/crates/cats/database`, `bk/crates/cats/asynchronous`, and `bk/crates/cats/concurrency`) rely solely on end-to-end execution of `main()`.
  - Refactor large `main()` routines into discrete helper functions and add unit tests exercising edge cases, timeouts, and error handling for these helpers.

### 3. Examples and Benchmarks

- **Add Criterion / Divan benchmarks to performance-sensitive crates**:
  - Only four crates in `bk/crates` currently define benchmarks (`development_tools_profiling`, `memory_management`, `data_structures`, and `algorithms`).
  - Add benchmarks for high-throughput utilities in `tools/tool_lib` (e.g., `published_recipes` link extraction regexes, crate block parsing, and markdown template generation).
  - Add benchmarks in `tools/example-integrity` measuring chapter parsing, regex anchoring, and module target graph traversal across large book trees.
  - Add benchmarks in `xmpl/cancelable` measuring task submission throughput, channel contention, and cancellation latency under concurrency.

### 4. Documentation and Doc Comments

- **Document public API items and non-main helpers across book examples**:
  - Multiple non-generated book examples contain public functions or types without rustdoc comments (e.g., `bk/crates/cats/authentication/examples/session_login/axum_login.rs`, `bk/crates/cats/web_programming_http_server/examples/axum_extras/axum_server_tls.rs`, and `bk/crates/code_organization/examples/naming_conventions/main.rs`).
  - Add educational doc comments explaining parameters, return values, errors, and concurrency guarantees, adhering to the requirement that all comments end with a period.

### 5. Architecture and Workspace Dependency Isolation

- [x] - **Enforce centralized lint configuration across sub-workspaces**: (lints generated by `scripts/sync_lints.py`; `deny.toml` stays per workspace)
  - Ensure all workspaces (`bk/crates`, `later/crates`, `tools`, `xmpl`, `publish`, `playground`) inherit identical workspace-level lints and deny rules from root configurations (`deny.toml`, `clippy.toml`, `rustfmt.toml`).

### 6. Discrepancies between Documentation and Code

- [x] **Verify `bin/README.md` against `just release`**:
  - Verify that `just release` recipes build and copy all required executables into `bin/` with clear usage documentation.
