## Extract the URL origin (scheme / host / port)

[![url-badge]][url] [![cat-net-badge]][cat-net]

The [`Url`] struct exposes various methods to extract information about the URL
it represents.

```rust,editable
{#include ../../../deps/examples/origin.rs}
```

[`origin`] produces the same result.

```rust,editable
{#include ../../../deps/examples/origin2.rs}
```

[`origin`]: https://docs.rs/url/*/url/struct.Url.html#method.origin
[`Url`]: https://docs.rs/url/*/url/struct.Url.html
