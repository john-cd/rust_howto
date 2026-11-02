<!-- TODO Reviewed 10-25-2026 -->

# Task tracking

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

Decide whether an empty line belongs before `ANCHOR_END` in `{{#include}}` markers and whether `{{!crate }}` should be supported or documented. Record the decision in the relevant parser or documentation issue and update this file to link the decision.

Acceptance: document the chosen syntax and run the focused parser/documentation checks for any behavior change.
