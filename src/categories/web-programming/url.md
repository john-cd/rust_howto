# Uniform Resource Location

{{#include url.incl.md}}

## Parse a URL from a string to a `Url` type

[![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

The [`url::Url::parse`][c-url::Url::parse]{{hi:url::Url::parse}}⮳ method from the [`url`][c-url]{{hi:url}}⮳ crate validates and parses a `&str` into a [`url::Url`][c-url::Url]{{hi:url::Url}}⮳ struct. The input string may be malformed so this method returns
`Result<Url, ParseError>`.

Once the URL has been parsed, it can be used with all of the methods in the
[`url::Url`][c-url::Url]{{hi:url::Url}}⮳ type.

```rust,editable
{{#include ../../../deps/tests/parse.rs}}
```

## Create a base URL by removing path segments

[![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

A base URL{{hi:URL}} includes a protocol and a domain. Base URLs have no folders, files or query strings. Each of those items are stripped out of the given URL. [`url::PathSegmentsMut::clear`][c-url::PathSegmentsMut::clear]{{hi:url::PathSegmentsMut::clear}}⮳ removes paths and [`url::Url::set_query`][c-url::Url::set_query]{{hi:url::Url::set_query}}⮳ removes query string.

```rust,editable
{{#include ../../../deps/tests/base.rs}}
```

## Create new URLs from a base URL

[![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

The [`url::Url::join`][c-url::Url::join]{{hi:url::Url::join}}⮳ method creates a new URL from a base and relative path{{hi:Relative path}}.

```rust,editable
{{#include ../../../deps/tests/new.rs}}
```

## Extract the URL origin (scheme / host / port)

[![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

The [`url::Url`][c-url::Url]{{hi:url::Url}}⮳ struct exposes various methods to extract information about the URL it represents.

```rust,editable
{{#include ../../../deps/tests/origin.rs}}
```

[`url::Url::origin`][c-url::Url::origin]{{hi:url::Url::origin}}⮳ produces the same result.

```rust,editable
{{#include ../../../deps/tests/origin1.rs}}
```

## Remove fragment identifiers and query pairs from a URL

[![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

Parses [`url::Url`][c-url::Url]{{hi:url::Url}}⮳ and slices it with [`url::Position`][c-url::Position]{{hi:url::Position}}⮳ to strip unneeded URL parts.

```rust,editable
{{#include ../../../deps/tests/fragment.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
<div class="hidden">
TODO:
</div>
