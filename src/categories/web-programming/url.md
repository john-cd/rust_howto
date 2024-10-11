# Uniform Resource Location

{{#include url.incl.md}}

## Parse a URL from a string to a `Url` type

[![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

The {{hi:parse}}[`parse`][c-url::Url::parse]⮳ method from the {{hi:url}}[`url`][c-url]⮳ crate validates and parses a `&str` into a {{hi:Url}}[`Url`][c-url::Url]⮳ struct. The input string may be malformed so this method returns
`Result<Url, ParseError>`.

Once the URL has been parsed, it can be used with all of the methods in the
{{hi:Url}}[`Url`][c-url::Url]⮳ type.

```rust,editable
{{#include ../../../deps/tests/parse.rs}}
```

## Create a base URL by removing path segments

[![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

A base {{i:URL}} includes a protocol and a domain. Base URLs have no folders, files or query strings. Each of those items are stripped out of the given URL. {{hi:PathSegmentsMut::clear}}[`PathSegmentsMut::clear`][c-url::PathSegmentsMut::clear]⮳ removes paths and {{hi:Url::set_query}}[`Url::set_query`][c-url::Url::set_query]⮳ removes query string.

```rust,editable
{{#include ../../../deps/tests/base.rs}}
```

## Create new URLs from a base URL

[![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

The {{hi:join}}[`join`][c-url::Url::join]⮳ method creates a new URL from a base and {{i:relative path}}.

```rust,editable
{{#include ../../../deps/tests/new.rs}}
```

## Extract the URL origin (scheme / host / port)

[![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

The {{hi:Url}}[`Url`][c-url::Url]⮳ struct exposes various methods to extract information about the URL it represents.

```rust,editable
{{#include ../../../deps/tests/origin.rs}}
```

{{hi:origin}}[`origin`][c-url::Url::origin]⮳ produces the same result.

```rust,editable
{{#include ../../../deps/tests/origin1.rs}}
```

## Remove fragment identifiers and query pairs from a URL

[![url][c-url-badge]][c-url]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]

Parses {{hi:Url}}[`Url`][c-url::Url]⮳ and slices it with {{hi:url::Position}}[`url::Position`][c-url::Position]⮳ to strip unneeded URL parts.

```rust,editable
{{#include ../../../deps/tests/fragment.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
