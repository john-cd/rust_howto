## Avoid writing and reading from a same file

[![same_file-badge]][same_file] [![cat-filesystem-badge]][cat-filesystem]

Use [`same_file::Handle`] to a file that can be tested for equality with
other handles. In this example, the handles of file to be read from and
to be written to are tested for equality.

```rust,editable,no_run
{#include ../../../deps/examples/same-file.rs}
```

```bash
cargo run
```

displays the contents of the file new.txt.

```bash
cargo run >> new.txt
```

errors because the two files are same.

[`same_file::Handle`]: https://docs.rs/same-file/*/same_file/struct.Handle.html
