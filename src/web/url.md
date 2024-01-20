# Uniform Resource Location

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

## Create a base URL by removing path segments

[![url-badge]][url] [![cat-net-badge]][cat-net]

A base URL includes a protocol and a domain.  Base URLs have no folders,
files or query strings.  Each of those items are stripped out of the given
URL.  [`PathSegmentsMut::clear`] removes paths and [`Url::set_query`] removes
query string.

```rust,editable
{#include ../../../deps/examples/base.rs}
```

## Create new URLs from a base URL

[![url-badge]][url] [![cat-net-badge]][cat-net]

The [`join`] method creates a new URL from a base and relative path.

```rust,editable
{#include ../../../deps/examples/new.rs}
```

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

## Remove fragment identifiers and query pairs from a URL

[![url-badge]][url] [![cat-net-badge]][cat-net]

Parses [`Url`] and slices it with [`url::Position`] to strip unneeded URL parts.

```rust,editable
{#include ../../../deps/examples/fragment.rs}
```

[`join`]: https://docs.rs/url/*/url/struct.Url.html#method.join
[`origin`]: https://docs.rs/url/*/url/struct.Url.html#method.origin
[`parse`]: https://docs.rs/url/*/url/struct.Url.html#method.parse
[`Url`]: https://docs.rs/url/*/url/struct.Url.html
[`PathSegmentsMut::clear`]: https://docs.rs/url/*/url/struct.PathSegmentsMut.html#method.clear
[`Url::set_query`]: https://docs.rs/url/*/url/struct.Url.html#method.set_query
[`url::Position`]: https://docs.rs/url/*/url/enum.Position.html
{{#include ../refs/link-refs.md}}
