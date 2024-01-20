## Find loops for a given path

[![same_file-badge]][same_file] [![cat-filesystem-badge]][cat-filesystem]

Use [`same_file::is_same_file`] to detect loops for a given path.
For example, a loop could be created on a Unix system via symlinks:

```bash
mkdir -p /tmp/foo/bar/baz
ln -s /tmp/foo/  /tmp/foo/bar/baz/qux
```

The following would assert that a loop exists.

```rust,editable,no_run
{#include ../../../deps/examples/loops.rs}
```

[`same_file::is_same_file`]: https://docs.rs/same-file/*/same_file/fn.is_same_file.html
