## Create a base URL by removing path segments

[![url-badge]][url] [![cat-net-badge]][cat-net]

A base URL includes a protocol and a domain.  Base URLs have no folders,
files or query strings.  Each of those items are stripped out of the given
URL.  [`PathSegmentsMut::clear`] removes paths and [`Url::set_query`] removes
query string.

```rust,editable
{#include ../../../deps/examples/base.rs}
```

[`PathSegmentsMut::clear`]: https://docs.rs/url/*/url/struct.PathSegmentsMut.html#method.clear
[`Url::set_query`]: https://docs.rs/url/*/url/struct.Url.html#method.set_query
