<!-- TODO Reviewed 10-25-2026 -->

# TO DO List

Every open task is tracked in a GitHub issue. This file lists what is left and, for each implementable item, a self-contained brief for Jules (which has no GitHub access). Status notes go in the issue; rows stay terse.

## Notes from John (unclear; clarify before work)

- [ ] Blank line before `ANCHOR_END`? (style question for the `{{#include}}` markers)
- [ ] `{{!crate }}` (syntax idea; no issue yet)

Review items for John (`tools/`, `.github/actions`, the `.py` scripts, `LICENSE`, `bookmarks.json`) live in epic #2113. `cargo husky` is obsolete: `prek` (`prek.toml`) runs the hooks.

WAIT move code examples from `later/` to `bk/` when code and text are ready.

---

## Open issues

- [#2113](https://github.com/john-cd/rust_howto/issues/2113) — **Epic** — Human decisions, reviews and manual steps (CI billing, license, analytics, editorial reviews). Not for Jules.
- [#2114](https://github.com/john-cd/rust_howto/issues/2114) — **Epic** — Manual and live-service validation runs (Oracle, Meilisearch, S3, NATS, desktop GUI). Needs services or a display.
- [#2115](https://github.com/john-cd/rust_howto/issues/2115) — **Epic** — Work that needs macOS, FreeBSD, Windows or hardware.
- [#2116](https://github.com/john-cd/rust_howto/issues/2116) — **Epic** — Waiting on an upstream release or advisory fix (zip, zenoh, urdf-rs, cpal, egui).
- [#2117](https://github.com/john-cd/rust_howto/issues/2117) — **Epic** — Deferred recipes, example cleanup and book quality.
- [#2118](https://github.com/john-cd/rust_howto/issues/2118) — **Epic** — Tooling, CI and repository infrastructure.
- [#2145](https://github.com/john-cd/rust_howto/issues/2145) — Optional support for complete badge groups that omit the crate index term (P4).

## Briefs for Jules

General rules for every brief: work from a clean checkout of `develop` on a topic branch; keep history linear; never delete or reword `TODO`, `TODO Reviewed` or human review comments; every example has a private `main()`, a `pub fn run()` (multi-file examples) and a test that calls `main()` with real assertions; no `unwrap()` outside unwrap demos; no `allow(dead_code)`; comments end with a period. Commit as `type(scope): summary` with the issue number. Format with `just fmtall` (nightly rustfmt is pinned in `rustfmt-toolchain.toml`). Run commands from the workspace directory (`bk/`, `later/`, `tools/`), not the repo root.

### Brief E — support unindexed crate badge groups (#2145; optional)

- **State:** all badge groups representable by the current syntax have been migrated. The migration check reports zero convertible blocks. `book_tooling` commit `c625472` supports empty category titles, but `{{#badges}}` always adds the crate's index term.
- **Goal:** add a `crate=` omission form for `{{#badges}}` in `book_tooling`, then migrate complete standard groups that omit `{{hi:<crate>}}` without adding an index term.
- **Steps:** add parser and renderer tests for the no-index form, pin the `book_tooling` revision in `tools/Cargo.toml` and `tools/Cargo.lock`, and extend the idempotent migration script and tests. Preserve the existing destinations and visible labels. Leave standalone badges, incomplete or nonstandard groups, and the excluded inline-code draft unchanged; document the remaining patterns in #2145.
- **Checks:** run `cargo test -p book_tool` from `book_tooling/`, `python bk/scripts/test_migrate_badge_sets.py`, `python bk/scripts/migrate_badge_sets.py --check`, and `just bb` from `bk/`.

### Not for Jules

- #2113 needs John's decisions and reviews. The remaining #2117 bookmark curation and draft promotion are editorial. #2114 needs live services or a display; #2115 needs macOS, FreeBSD or hardware. #2116 waits for upstream releases and advisory fixes. #2118's remaining container image work needs Docker validation.

---

## Review: `dead_code` allowances and ignored tests

- No `dead_code` allowance cleanup remains. Run the outstanding prerequisite-gated ignored checks in #2114 and #2115 when their services, desktop, operating systems, or hardware are available; NATS JetStream and Windows checks have passed.
