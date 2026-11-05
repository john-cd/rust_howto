<!-- TODO Reviewed 10-25-2026 -->

# Task tracking

Open work is tracked in [GitHub Issues](https://github.com/john-cd/rust_howto/issues).
Keep one topic per issue, except epics, with a problem, task list, and acceptance criteria.
Record status and validation evidence in the issue rather than duplicating checklists here.

<!-- recommended-issue-order:start -->

### Ready: priority order

- [#2165: Epic: Complete dependency-update coverage and consistent CI caching](https://github.com/john-cd/rust_howto/issues/2165) — Local updater/cache/security dispositions; upstream waits remain under #2116.

### Blocked or deferred

- [#2202: Run native Linux and macOS linker and workspace validation for Cargo profiles](https://github.com/john-cd/rust_howto/issues/2202) — Needs a macOS runner and a native Linux host or CI.
- [#2203: Capture Windows linker invocation, rust-analyzer target dir, and mdbook-utils in workspace matrix](https://github.com/john-cd/rust_howto/issues/2203) — Windows-host checks plus initializing the `mdbook-utils` submodule.
- [#2151: Validate ignored example-entry tests from #2133 (services, nextest, clippy 1.97 lints)](https://github.com/john-cd/rust_howto/issues/2151) — Real service-backed/native feature environments and the specified nextest checks.
- [#2160: Validate pinned Python and heavy-test service versions](https://github.com/john-cd/rust_howto/issues/2160) — Docker Linux engine, runtime/service images and Oracle registry credentials where required.
- [#2173: Record native-platform CI results after hosted runners
  start](https://github.com/john-cd/rust_howto/issues/2173) —
  Advisory macOS/FreeBSD jobs are still queued; record their results
  when GitHub assigns runners.
- [#2116: Epic: waiting on an upstream release or advisory fix](https://github.com/john-cd/rust_howto/issues/2116) — Compatible fixed upstream dependencies; keep exact release/constraint triggers in the issue.
- [#2113: Epic: human decisions, reviews, and host-dependent validation](https://github.com/john-cd/rust_howto/issues/2113) — Human reviews, accounts, native libraries and host prerequisites.

<!-- recommended-issue-order:end -->

## Blocker tracking

- [Human decisions, reviews, and host prerequisites](https://github.com/john-cd/rust_howto/issues/2113).
- [Upstream release and advisory waits](https://github.com/john-cd/rust_howto/issues/2116).

## Maintenance

- [Native Linux and macOS Cargo profile validation](https://github.com/john-cd/rust_howto/issues/2202).
- [Windows linker, rust-analyzer, and mdbook-utils validation](https://github.com/john-cd/rust_howto/issues/2203).
- [Complete dependency-update coverage and consistent CI caching](https://github.com/john-cd/rust_howto/issues/2165).
