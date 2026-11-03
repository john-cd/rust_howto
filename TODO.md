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

- [#2163: Epic: Finish shared formatting, lint, and hook policy validation](https://github.com/john-cd/rust_howto/issues/2163) — Local formatting/lint/teaching-example fixes; keep native-stack and human gates separate.
- [#2165: Epic: Complete dependency-update coverage and consistent CI caching](https://github.com/john-cd/rust_howto/issues/2165) — Local updater/cache/security dispositions; upstream waits remain under #2116.

### Blocked or deferred

- [#2164: Epic: Validate and finalize Cargo profiles, output paths, and linker policy](https://github.com/john-cd/rust_howto/issues/2164) — Remaining cross-platform/linker acceptance needs the corresponding hosts/CI; completed Windows measurements need no replay.
- [#2151: Validate ignored example-entry tests from #2133 (services, nextest, clippy 1.97 lints)](https://github.com/john-cd/rust_howto/issues/2151) — Real service-backed/native feature environments and the specified nextest checks.
- [#2159: Standardize devcontainer Ruby setup on Ruby 4.0](https://github.com/john-cd/rust_howto/issues/2159) — Host Ruby 3.3 removal and Ruby 4.0 resolution verification; implementation is already present.
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

- [Finish shared formatting, lint, and hook policy validation](https://github.com/john-cd/rust_howto/issues/2163).
- [Validate and finalize Cargo profiles, output paths, and linker policy](https://github.com/john-cd/rust_howto/issues/2164).
- [Complete dependency-update coverage and consistent CI caching](https://github.com/john-cd/rust_howto/issues/2165).
- [Review and reconcile repository branches and worktrees](https://github.com/john-cd/rust_howto/issues/2168).
- [Review or close PR #2161 through its follow-up](https://github.com/john-cd/rust_howto/issues/2167).

## Briefs for work without GitHub access

Keep implementation briefs self-contained here because Jules cannot read GitHub issue descriptions. Issue links provide traceability; they do not replace the scope or acceptance details.

### #2166 — Clarify two marker syntax questions

Decision: anchor selection includes every line between the matching markers, including a blank line immediately before `ANCHOR_END`; put the end marker directly after the last desired line to omit that blank line. `mdbook-utils` expands `{{#include ...}}` only and preserves `{{!crate }}` as literal text; it is not a special directive. The parser documentation and focused tests are in [mdbook-utils](https://github.com/john-cd/mdbook-utils).

Acceptance: document the chosen syntax and run the focused parser/documentation checks for any behavior change.
