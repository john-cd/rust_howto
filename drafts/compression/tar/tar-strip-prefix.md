## Decompress a tarball while removing a prefix from the paths

[![flate2-badge]][flate2] [![tar-badge]][tar] [![cat-compression-badge]][cat-compression]

Iterate over the [`Archive::entries`].  Use [`Path::strip_prefix`] to remove
the specified path prefix (`bundle/logs`).  Finally, extract the [`tar::Entry`]
via [`Entry::unpack`].

```rust,editable,no_run
{#include ../../../deps/examples/tar-strip-prefix.rs}
```

[`Archive::entries`]: https://docs.rs/tar/*/tar/struct.Archive.html#method.entries
[`Entry::unpack`]: https://docs.rs/tar/*/tar/struct.Entry.html#method.unpack
[`Path::strip_prefix`]: https://doc.rust-lang.org/std/path/struct.Path.html#method.strip_prefix
[`tar::Entry`]: https://docs.rs/tar/*/tar/struct.Entry.html
