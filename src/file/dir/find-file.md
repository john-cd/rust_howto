## Recursively find all files with given predicate

[![walkdir-badge]][walkdir] [![cat-filesystem-badge]][cat-filesystem]

Find JSON files modified within the last day in the current directory.
Using [`follow_links`] ensures symbolic links are followed like they were
normal directories and files.

```rust,editable,no_run
{#include ../../../deps/examples/find-file.rs}
```

[`follow_links`]: https://docs.rs/walkdir/*/walkdir/struct.WalkDir.html#method.follow_links
