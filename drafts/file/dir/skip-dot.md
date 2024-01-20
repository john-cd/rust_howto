## Traverse directories while skipping dotfiles

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Uses [`filter_entry`] to descend recursively into entries passing the
`is_not_hidden` predicate thus skipping hidden files and directories.
 [`Iterator::filter`] applies to each [`WalkDir::DirEntry`] even if the parent
 is a hidden directory.

Root dir `"."` yields through [`WalkDir::depth`] usage in `is_not_hidden`
predicate.

```rust,editable,no_run
{#include ../../../deps/examples/skip-dot.rs}
```

[`filter_entry`]: https://docs.rs/walkdir/*/walkdir/struct.IntoIter.html#method.filter_entry
[`Iterator::filter`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[`WalkDir::depth`]: https://docs.rs/walkdir/*/walkdir/struct.DirEntry.html#method.depth
[`WalkDir::DirEntry`]: https://docs.rs/walkdir/*/walkdir/struct.DirEntry.html
