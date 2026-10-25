<!-- TODO Reviewed ??-??-2026 -->

# Git History with `gix`

{{#include git.incl.md}}

## Count Changes per File with `gix` {#git-churn}

[![gix][c~gix~docs~badge]][c~gix~docs] [![gix~crates.io][c~gix~crates.io~badge]][c~gix~crates.io] [![gix~repo][c~gix~repo~badge]][c~gix~repo]{{hi:gix}}{{hi:Git}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Git}}

[`gix`][c~gix~docs]↗{{hi:gix}} (gitoxide) is a pure-Rust implementation of Git. A tool can open a repository, walk the history and diff trees without a `git` executable or a C library. This example finds the repository that contains the crate with `gix::discover`, walks the first-parent history, diffs each commit against its parent and counts how often each path changed (the "churn" metric used to find hot spots). The first commit is compared with the empty tree, and a directory that is not a repository is reported as an error rather than a panic.

`gix` is split into many optional features. Enable only what you need, such as `revision` for history walks and `blob-diff` for tree diffs, plus one hash implementation (`sha1`). For line counts, see [`tokei`][c~tokei~docs]↗{{hi:tokei}}.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/automation/git_churn.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[xtask | `xtask` Project Checks]]

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
