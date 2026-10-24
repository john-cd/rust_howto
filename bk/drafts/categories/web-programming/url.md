<!-- TODO Reviewed ??-??-2026 -->

# Uniform Resource Location

{{#include url.incl.md}}

## Parse a URL from a String to a `Url` Type {#parse-a-url-from-a-string-to-a-url-type}

[![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

The [`url::Url::parse`][c~url::Url::parse~docs]↗{{hi:url::Url::parse}} method from the [`url`][c~url~docs]↗{{hi:url}} crate validates and parses a `&str` into a [`url::Url`][c~url::Url~docs]↗{{hi:url::Url}} struct. The input string may be malformed so this method returns
`Result<Url, ParseError>`.

Once the URL has been parsed, it can be used with all of the methods in the
[`url::Url`][c~url::Url~docs]↗{{hi:url::Url}} type.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/parse.rs:example}}
```

## Create a Base URL by Removing Path Segments {#create-a-base-url-by-removing-path-segments}

[![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

This recipe reduces a hierarchical URL to its origin root by clearing the path, query, and fragment. That root can be useful as a base for URL resolution; when joining relative paths, remember that a trailing slash makes the final base segment a directory.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/base.rs:example}}
```

## Create new URLs from a Base URL {#create-new-urls-from-a-base-url}

[![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

The [`url::Url::join`][c~url::Url::join~docs]↗{{hi:url::Url::join}} method resolves a relative reference against a base URL. A leading slash replaces the base path, while a path without one is appended relative to the base directory.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/new.rs:example}}
```

## Extract the URL Origin (scheme / Host / port) {#extract-the-url-origin}

[![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

The [`url::Url`][c~url::Url~docs]↗{{hi:url::Url}} struct exposes various methods to extract information about the URL it represents.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/origin.rs:example}}
```

[`url::Url::origin`][c~url::Url::origin~docs]↗{{hi:url::Url::origin}} produces the same result.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/origin1.rs:example}}
```

## Remove Fragment Identifiers and Query Pairs from a URL {#remove-fragment-identifiers-and-query-pairs}

[![url][c~url~docs~badge]][c~url~docs]{{hi:url}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

Parses [`url::Url`][c~url::Url~docs]↗{{hi:url::Url}} and slices it at [`url::Position::AfterPath`][c~url::Position::AfterPath~docs]↗{{hi:url::Position::AfterPath}} to exclude the query and fragment. Use URL APIs to encode individual query components instead of concatenating untrusted text; [`percent-encoding`][c~percent-encoding~docs]↗ provides component encoders.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/url/fragment.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[requests | Make HTTP Requests]].
- [[scraping | Extracting Links]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
This review removes a stale commented implementation, makes the URL joining example show relative-path behavior, and ensures the query/fragment example actually contains and removes both components. It adds focused examples for path, query, fragment, and port handling.
</div>
