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
- [#2145](https://github.com/john-cd/rust_howto/issues/2145) — Optional migration of remaining badge blocks; preserve exact rendered links and index terms (P4).

## Briefs for Jules

General rules for every brief: work from a clean checkout of `develop` on a topic branch; keep history linear; never delete or reword `TODO`, `TODO Reviewed` or human review comments; every example has a private `main()`, a `pub fn run()` (multi-file examples) and a test that calls `main()` with real assertions; no `unwrap()` outside unwrap demos; no `allow(dead_code)`; comments end with a period. Commit as `type(scope): summary` with the issue number. Format with `just fmtall` (nightly rustfmt is pinned in `rustfmt-toolchain.toml`). Run commands from the workspace directory (`bk/`, `later/`, `tools/`), not the repo root.

### Brief E — finish the remaining badge migration (#2145; optional)

- **State:** `tools/Cargo.toml` already pins `book_tooling` commit `c625472`, which supports empty category titles. `python bk/scripts/migrate_badge_sets.py --check` reports no remaining blocks that the current script can convert.
- **Goal:** migrate only the remaining badge blocks that `{{#badges}}` can represent without changing their meaning.
- **Steps:** review the mixed-badge, multiple-crate, and table cases listed in #2145. If needed, add the smallest `{{#badges}}` syntax that preserves destinations, labels, order, and `{{hi:...}}` terms; add renderer tests in `book_tooling`, pin the resulting revision in `tools/Cargo.toml` and `tools/Cargo.lock`, and extend the idempotent migration script and its tests. Leave unrepresentable blocks unchanged and record those patterns in #2145. Keep the excluded inline-code draft unchanged unless directive expansion can safely handle it.
- **Checks:** run `cargo test -p book_tool` from `book_tooling/`, `python bk/scripts/test_migrate_badge_sets.py`, `python bk/scripts/migrate_badge_sets.py --check`, and `just bb` from `bk/`.

### Not for Jules

- #2113 needs John's decisions and reviews. The remaining #2117 bookmark curation and draft promotion are editorial. #2114 needs live services or a display; #2115 needs macOS, FreeBSD or hardware. #2116 waits for upstream releases and advisory fixes. #2118's remaining container image work needs Docker validation.

---

## Review: `dead_code` allowances and ignored tests

- No blanket `dead_code` cleanup remains. Keep the intentional allowances for generated code, Node addon exports, and the `allow_dead_code` teaching example. Remaining expectations document intentionally uncalled paths or examples gated on services, devices, displays, contexts, or targets.
- Run the remaining prerequisite-gated ignored checks tracked in #2114 and #2115 (services, desktop, macOS/FreeBSD, and hardware). The NATS JetStream and Windows checks are complete.
- No separate follow-up is indicated by this inventory.
