## Get MIME type from string

[![mime-badge]][mime] [![cat-encoding-badge]][cat-encoding]

The following example shows how to parse a [`MIME`] type from a string using the
[mime] crate. [`FromStrError`] produces a default [`MIME`] type in an
`unwrap_or` clause.

```rust,editable
{#include ../../../deps/examples/string.rs}
```

[`FromStrError`]: https://docs.rs/mime/*/mime/struct.FromStrError.html
[`MIME`]: https://docs.rs/mime/*/mime/struct.Mime.html
