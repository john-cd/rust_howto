# Media Types

{{#include index.incl.md}}

## Get MIME type from string

[![mime][mime-badge]][mime]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

The following example shows how to parse a [`MIME`][mime::Mime] type from a string using the
[mime][mime::Mime] crate. [`FromStrError`][mime::FromStrError] produces a default [`MIME`][mime::Mime] type in an
`unwrap_or` clause.

```rust,editable
{{#include ../../../../deps/tests/string.rs}}
```

## Get MIME type from filename

[![mime][mime-badge]][mime]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

The following example shows how to return the correct MIME type from a given filename using the [`mime`][mime] crate. The program will check for file extensions and match against a known list. The return value is [`mime:Mime`][mime::Mime].

```rust,editable
{{#include ../../../../deps/tests/filename.rs}}
```

## Parse the MIME type of a HTTP response

[![reqwest][reqwest-badge]][reqwest]  [![mime][mime-badge]][mime]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

When receiving a HTTP response from `reqwest` the [`MIME type`][mozilla-mime-type] or media type may be found in the [`Content-Type`][mozilla-content-type] header. [`reqwest::header::HeaderMap::get`][reqwest::header::HeaderMap::get] retrieves the header as a [`reqwest::header::HeaderValue`][reqwest::header::HeaderValue] which can be converted to a string. The `mime` crate can then parse that, yielding a [`mime::Mime`][mime::Mime] value.

The [`mime`][mime] crate also defines some commonly used MIME types.

Note that the [`reqwest::header`][reqwest::header] module is exported from the [`http`][http] crate.

```rust,editable,no_run
{{#include ../../../../deps/tests/request.rs}}
```

{{#include ../../../refs/link-refs.md}}
