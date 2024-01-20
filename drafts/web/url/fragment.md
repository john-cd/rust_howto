## Remove fragment identifiers and query pairs from a URL

[![url-badge]][url] [![cat-net-badge]][cat-net]

Parses [`Url`] and slices it with [`url::Position`] to strip unneeded URL parts.

```rust,editable
{#include ../../../deps/examples/fragment.rs}
```

[`url::Position`]: https://docs.rs/url/*/url/enum.Position.html
[`Url`]: https://docs.rs/url/*/url/struct.Url.html
