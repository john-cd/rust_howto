# Media Types

{{#include mime.incl.md}}

## Get a MIME type from a string {#get-mime-type-from-string}

[![mime][c-mime-badge]][c-mime]{{hi:mime}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

The following example shows how to parse a [`mime::Mime`][c-mime::Mime]{{hi:mime::Mime}} type from a string using the [mime][c-mime]{{hi:mime}}⮳ crate. [`mime::Mime`][c-mime::Mime]{{hi:mime::Mime}}⮳ produces a default [`mime::Mime`][c-mime::Mime]{{hi:mime::Mime}}⮳ type in an [`std::result::Result::unwrap_or`][c-std::result::Result::unwrap_or]{{hi:std::result::Result::unwrap_or}}⮳ clause.

```rust,editable
{{#include ../../../crates/ex/categories/wxyz/tests/web_programming/string.rs:example}}
```

## Get a MIME type from a filename {#get-mimetype-from-filename}

[![mime][c-mime-badge]][c-mime]{{hi:mime}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

The following example shows how to return the correct MIME{{hi:MIME type}} type from a given filename using the [`mime`][c-mime]{{hi:mime}}⮳ crate. The program will check for file extensions and match against a known list. The return value is [`mime::Mime`][c-mime::Mime]{{hi:mime::Mime}}⮳.

```rust,editable
{{#include ../../../crates/ex/categories/wxyz/tests/web_programming/filename.rs:example}}
```

## Parse the MIME type of a HTTP response {#parse-the-mime-type-of-a-http-response}

[![reqwest][c-reqwest-badge]][c-reqwest]{{hi:reqwest}} [![mime][c-mime-badge]][c-mime]{{hi:mime}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

When receiving a HTTP response{{hi:HTTP response}} from [`reqwest`][c-reqwest]{{hi:reqwest}}⮳ the [`MIME type`][mozilla-mime-type]{{hi:MIME type}}⮳ or media type may be found in the [`Content-Type`][mozilla-content-type]{{hi:Content type}}⮳ header. [`reqwest::header::HeaderMap::get`][c-reqwest::header::HeaderMap::get]{{hi:reqwest::header::HeaderMap::get}}⮳ retrieves the header{{hi:Header}} as a [`reqwest::header::HeaderValue`][c-reqwest::header::HeaderValue]{{hi:reqwest::header::HeaderValue}}⮳ which can be converted to a string. The [`mime`][c-mime]{{hi:mime}}⮳ crate can then parse that, yielding a [`mime::Mime`][c-mime::Mime]{{hi:mime::Mime}}⮳ value.

The [`mime`][c-mime]{{hi:mime}}⮳ crate also defines some commonly used MIME types.

Note that the [`reqwest::header`][c-reqwest::header]{{hi:reqwest::header}}⮳ module is exported from the [`http`][c-http]{{hi:http}}⮳ crate.

```rust,editable
{{#include ../../../crates/ex/categories/wxyz/tests/web_programming/request.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 review

</div>
