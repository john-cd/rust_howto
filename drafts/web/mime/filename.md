## Get MIME type from filename

[![mime-badge]][mime] [![cat-encoding-badge]][cat-encoding]

The following example shows how to return the correct MIME type from a given
filename using the [mime] crate.  The program will check for file extensions
and match against a known list.  The return value is [`mime:Mime`].

```rust,editable
{#include ../../../deps/examples/filename.rs}
```

[`mime:Mime`]: https://docs.rs/mime/*/mime/struct.Mime.html
