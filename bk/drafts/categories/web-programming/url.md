# Uniform Resource Location

{{#include url.incl.md}}

## Parse a URL from a String to a `Url` Type {#parse-a-url-from-a-string-to-a-url-type}

[![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

The [`url::Url::parse`][c~url::Url::parse~docs]{{hi:url::Url::parse}}↗ method from the [`url`][c~url~docs]{{hi:url}}↗ crate validates and parses a `&str` into a [`url::Url`][c~url::Url~docs]{{hi:url::Url}}↗ struct. The input string may be malformed so this method returns
`Result<Url, ParseError>`.

Once the URL has been parsed, it can be used with all of the methods in the
[`url::Url`][c~url::Url~docs]{{hi:url::Url}}↗ type.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/parse.rs:example}}
```

## Create a Base URL by Removing Path Segments {#create-a-base-url-by-removing-path-segments}

[![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

A base URL{{hi:URL}} includes a protocol and a domain. Base URLs have no folders, files or query strings. Each of those items are stripped out of the given URL. [`url::PathSegmentsMut::clear`][c~url::PathSegmentsMut::clear~docs]{{hi:url::PathSegmentsMut::clear}}↗ removes paths and [`url::Url::set_query`][c~url::Url::set_query~docs]{{hi:url::Url::set_query}}↗ removes query string.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/base.rs:example}}
```

## Create new URLs from a Base URL {#create-new-urls-from-a-base-url}

[![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

The [`url::Url::join`][c~url::Url::join~docs]{{hi:url::Url::join}}↗ method creates a new URL from a base and relative path{{hi:Relative path}}.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/new.rs:example}}
```

## Extract the URL Origin (scheme / Host / port) {#extract-the-url-origin}

[![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

The [`url::Url`][c~url::Url~docs]{{hi:url::Url}}↗ struct exposes various methods to extract information about the URL it represents.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/origin.rs:example}}
```

[`url::Url::origin`][c~url::Url::origin~docs]{{hi:url::Url::origin}}↗ produces the same result.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/origin1.rs:example}}
```

## Remove Fragment Identifiers and Query Pairs from a URL {#remove-fragment-identifiers-and-query-pairs}

[![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

Parses [`url::Url`][c~url::Url~docs]{{hi:url::Url}}↗ and slices it with [`url::Position`][c~url::Position~docs]{{hi:url::Position}}↗ to strip unneeded URL parts.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/fragment.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/973)
</div>
