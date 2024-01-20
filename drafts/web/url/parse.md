## Parse a URL from a string to a `Url` type

[![url-badge]][url] [![cat-net-badge]][cat-net]

The [`parse`] method from the `url` crate validates and parses a `&str` into a
[`Url`] struct. The input string may be malformed so this method returns
`Result<Url, ParseError>`.

Once the URL has been parsed, it can be used with all of the methods in the
`Url` type.

```rust,editable
{#include ../../../deps/examples/parse.rs}
```

[`parse`]: https://docs.rs/url/*/url/struct.Url.html#method.parse
[`Url`]: https://docs.rs/url/*/url/struct.Url.html
