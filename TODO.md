<!-- TODO Reviewed ??-??-2026. -->

<!-- TODO Reviewed 10-25-2026 -->

# Task tracking

Open work is tracked in [GitHub Issues](https://github.com/john-cd/rust_howto/issues).
Keep one topic per issue, except epics, with a problem, task list, and acceptance criteria.
Record status and validation evidence in the issue rather than duplicating checklists here.

## Book coverage follow-ups

The November 15–16 audit compared the published book, `bk/drafts`, `later`, and the existing issues. It drew leads from [This Week in Rust #673–677](https://this-week-in-rust.org/) and checked release claims against the [Rust 1.100 announcement](https://blog.rust-lang.org/releases/1.100.0). The [coverage audit #2249](https://github.com/john-cd/rust_howto/issues/2249) tracks the source review and chapter links. The [Clippy chapter #2248](https://github.com/john-cd/rust_howto/issues/2248) is written and its workspace example is verified on Rust 1.100.0.

The published book has foundational coverage, with the largest reader-facing gaps in async, web, deployment, and data workflows. Promote focused pieces of existing material where possible. WIT components, Leptos, `Send`/`Sync`, `cargo xtask`, and OpenAPI already have examples or guides in draft or future chapters. The separate first-wave promotion issues [#2250](https://github.com/john-cd/rust_howto/issues/2250)–[#2253](https://github.com/john-cd/rust_howto/issues/2253) cover WebSockets, number formatting, image processing, and `no_std`.

- **P2** [#2254](https://github.com/john-cd/rust_howto/issues/2254) · Cover `!`, `DropGuard`, `Allocator`, and version-sensitive examples while preserving the Rust 1.98.1 baseline.
- **P2** [#2255](https://github.com/john-cd/rust_howto/issues/2255) · Document Cargo's unused-dependency lint, publish-age policy, cache layout, and trusted publishing.
- **P2** [#2256](https://github.com/john-cd/rust_howto/issues/2256) · Promote a measured compile-time diagnosis workflow using Cargo timings.
- **P2** [#2257](https://github.com/john-cd/rust_howto/issues/2257) · Bridge the published Pin chapter to draft examples for cancellation, task ownership, and `Send`.
- **P2** [#2258](https://github.com/john-cd/rust_howto/issues/2258) · Add a tested library-to-CLI example with typed errors, source chains, and useful messages.
- **P2** [#2259](https://github.com/john-cd/rust_howto/issues/2259) · Promote an Axum update endpoint with absent/null/value behavior and offline router tests.
- **P3** [#2260](https://github.com/john-cd/rust_howto/issues/2260) · Exercise bounded retries, deadlines, and backpressure with a deterministic Tower service.
- **P3** [#2261](https://github.com/john-cd/rust_howto/issues/2261) · Show explicit registration and `inventory` with stable output and duplicate handling.
- **P3** [#2262](https://github.com/john-cd/rust_howto/issues/2262) · Compare summation algorithms and explain the accuracy implications of SIMD or parallel reductions.
- **P3** [#2263](https://github.com/john-cd/rust_howto/issues/2263) · Add a Jiff example for ambiguous and nonexistent local times.
- **P3** [#2264](https://github.com/john-cd/rust_howto/issues/2264) · Add a small Creusot contract proof beside the existing verification material.
- **P3** [#2265](https://github.com/john-cd/rust_howto/issues/2265) · Show a surviving `cargo-mutants` mutation and the assertion that detects it.
- **P3** [#2266](https://github.com/john-cd/rust_howto/issues/2266) · Promote an offline data-analysis recipe from the Arrow, Polars, and DataFusion material.
- **P3** [#2267](https://github.com/john-cd/rust_howto/issues/2267) · Turn the existing AWS Lambda material into a runnable local workflow.
- **P3** [#2268](https://github.com/john-cd/rust_howto/issues/2268) · Show a local Iroh peer round trip, identity, and failure handling.

<!-- recommended-issue-order:start -->

## Ready and blocked issues

Snapshot: November 11, 2026, from live GitHub issues. Issues #2238 and #2203 were completed on November 9; #2225 was completed after validation; #2236 was closed by owner decision with native-host validation deferred; #2211 remains blocked on GitHub read credentials for the full devcontainer build.
Ready issues can start now, in priority order. Blocked issues are grouped by blocker; each group
has one tracking issue labelled `tracking`. Epics are listed where they wait on a blocker.
Recheck the live issue before starting.

### Ready

### Blocked

#### CI unavailable — tracker [#2207](https://github.com/john-cd/rust_howto/issues/2207)

- Only the tracker itself (its body lists the deferred checks).

#### Human decisions and reviews — tracker [#2239](https://github.com/john-cd/rust_howto/issues/2239)

- [#2216: Owner review of chapters, recipes and tools](https://github.com/john-cd/rust_howto/issues/2216)
- [#2215: Maintainer decisions: licensing, analytics, scope, tooling ownership](https://github.com/john-cd/rust_howto/issues/2215)

#### Waiting for upstream or dependency — tracker [#2116](https://github.com/john-cd/rust_howto/issues/2116)

- Only the tracker itself (its body lists the deferred checks).

#### Hardware, external host or account — tracker [#2240](https://github.com/john-cd/rust_howto/issues/2240)

- [#2211: Validate the Python 3.15.2 devcontainer and Maturin/PyO3 example](https://github.com/john-cd/rust_howto/issues/2211) — the locked Windows extension build and import pass with CPython 3.15.2/Maturin 1.16.0; the full devcontainer build cannot fetch `john-cd/book_tooling` because no valid GitHub read token is available.
- [#2214: Validate macOS and FreeBSD native examples](https://github.com/john-cd/rust_howto/issues/2214)
- [#2213: Validate GUI, window and hardware examples on a desktop or device](https://github.com/john-cd/rust_howto/issues/2213)
- [#2202: Run native Linux and macOS linker and workspace validation for Cargo profiles](https://github.com/john-cd/rust_howto/issues/2202)

<!-- recommended-issue-order:end -->

## Blocker tracking

- [Upstream release and advisory waits](https://github.com/john-cd/rust_howto/issues/2116).
- Host, service and human-dependent work: #2207-#2216 above.
