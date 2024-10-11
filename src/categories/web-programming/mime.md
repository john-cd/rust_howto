# Media Types

{{#include mime.incl.md}}

## Get MIME type from string

[![mime][c-mime-badge]][c-mime]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

The following example shows how to parse a [`mime::Mime`][c-mime::Mime]{{hi:mime::Mime}} type from a string using the [mime][c-mime]⮳ crate. [`mime::Mime`][c-mime::Mime]{{hi:mime::Mime}}⮳ produces a default [`mime::Mime`][c-mime::Mime]{{hi:mime::Mime}}⮳ type in an [`std::result::Result::unwrap_or`][c-std::result::Result::unwrap_or]{{hi:std::result::Result::unwrap_or}}⮳ clause.

```rust,editable
{{#include ../../../deps/tests/string.rs}}
```

## Get MIME type from filename

[![mime][c-mime-badge]][c-mime]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

The following example shows how to return the correct MIME{{hi:MIME type}} type from a given filename using the [`mime`][c-mime]{{hi:mime}}⮳ crate. The program will check for file extensions and match against a known list. The return value is [`mime::Mime`][c-mime::Mime]{{hi:mime::Mime}}⮳.

```rust,editable
{{#include ../../../deps/tests/filename.rs}}
```

## Parse the MIME type of a HTTP response

[![reqwest][c-reqwest-badge]][c-reqwest]  [![mime][c-mime-badge]][c-mime]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

When receiving a HTTP response{{hi:HTTP response}} from [`reqwest`][c-reqwest]{{hi:reqwest}}⮳ the [`MIME type`][mozilla-mime-type]{{hi:MIME type}}⮳ or media type may be found in the [`Content-Type`][mozilla-content-type]{{hi:Content-Type}}⮳ header. [`reqwest::header::HeaderMap::get`][c-reqwest::header::HeaderMap::get]{{hi:reqwest::header::HeaderMap::get}}⮳ retrieves the header{{hi:Header}} as a [`reqwest::header::HeaderValue`][c-reqwest::header::HeaderValue]{{hi:reqwest::header::HeaderValue}}⮳ which can be converted to a string. The [`mime`][c-mime]{{hi:mime}}⮳ crate can then parse that, yielding a [`mime::Mime`][c-mime::Mime]{{hi:mime::Mime}}⮳ value.

The [`mime`][c-mime]{{hi:mime}}⮳ crate also defines some commonly used MIME types.

Note that the [`reqwest::header`][c-reqwest::header]{{hi:reqwest::header}}⮳ module is exported from the [`http`][c-http]{{hi:http}}⮳ crate.

```rust,editable,no_run
{{#include ../../../deps/tests/request.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
