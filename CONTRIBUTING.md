<!-- TODO Reviewed ??-??-2026 -->

# Contributing

Thanks for helping improve the Rust How-to book and its companion tools.

## Start here

Read the [contributing guide in the book](https://john-cd.com/rust_howto/appendices/contributing/index.html) for contribution types, writing style, repository structure, development setup, and guidance for chapters and examples. It also links to the project's Code of Conduct and contribution license notice.

Before starting a larger change, search the [existing issues](https://github.com/john-cd/rust_howto/issues). For a bug or feature, open or update an issue with the expected behavior and a reproducible example. Keep pull requests focused, link the issue, and list the checks you ran.

## Choose the workspace

The repository root is not one Cargo workspace. Run commands from the workspace you change. `bk/` contains the published book and its runnable examples. `later/`, `tools/`, `xmpl/`, `playground/`, and `publish/` are independent workspaces. The companion `mdbook-utils/` project has its own contributor instructions.

For changes to the published book examples, run these commands from `bk/`:

```text
just fa
just ba
just ca
just nta
```

Use `just ntt <keyword>` for a focused test. Run the matching `just` recipes from the root of any other affected workspace.

## Change and preview the book

`bk/src/SUMMARY.md` controls chapter navigation. Add an entry when you add or move a chapter. Preserve shared include and reference fragments, and put runnable Rust examples under `bk/crates/<section-or-category>/examples/<chapter>/`.

After changing book content, run `just bb` from `bk/` to build the book. In the pull request, note any service, GUI, or platform-specific checks that could not run in your environment.

## Issue priority and review labels

Use `priority:P1` through `priority:P4` labels; do not put `[P#]` prefixes in issue titles. P1 blocks publication, P2 is needed for the first edition, P3 should be done, and P4 is later or optional. Use `human-review` when work needs the owner's decision or a manual acceptance check. In `rust_howto`, every `human-review` issue must be a sub-issue of epic [#1849](https://github.com/john-cd/rust_howto/issues/1849).

Priority labels rank work. Milestones track release scope: reserve `URGENT` for current publication blockers; use `PUBLISHABLE` for work in the publication-ready batch. These axes are independent, so a P3 item can be in `PUBLISHABLE` when it belongs in that batch, and P1/P2 items can use another milestone when their release target differs. A P4 item normally belongs in `LATER PUBLICATION`.

When a project board has a `Priority` field, mirror the issue's `priority:P#` label there for sorting. Use board status and scheduling fields to track workflow; the issue label remains the priority source of truth. Update the label and board field together when priority changes.
