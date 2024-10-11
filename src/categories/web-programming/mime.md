# Media Types

{{#include mime.incl.md}}

## Get MIME type from string

[![mime][c-mime-badge]][c-mime]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

The following example shows how to parse a {{hi:MIME}}[`MIME`][c-mime::Mime] type from a string using the [mime][c-mime]⮳ crate. {{hi:FromStrError}}[`FromStrError`][c-mime::Mime]⮳ produces a default {{hi:MIME}}[`MIME`][c-mime::Mime]⮳ type in an {{hi:unwrap_or}}[`unwrap_or`][c-std::result::Result::unwrap_or]⮳ clause.

```rust,editable
{{#include ../../../deps/tests/string.rs}}
```

## Get MIME type from filename

[![mime][c-mime-badge]][c-mime]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

The following example shows how to return the correct {{i:MIME}} type from a given filename using the {{hi:mime}}[`mime`][c-mime]⮳ crate. The program will check for file extensions and match against a known list. The return value is {{hi:mime:Mime}}[`mime:Mime`][c-mime::Mime]⮳.

```rust,editable
{{#include ../../../deps/tests/filename.rs}}
```

## Parse the MIME type of a HTTP response

[![reqwest][c-reqwest-badge]][c-reqwest]  [![mime][c-mime-badge]][c-mime]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

When receiving a {{i:HTTP response}} from {{hi:reqwest}}[`reqwest`][c-reqwest]⮳ the {{hi:MIME type}}[`MIME type`][mozilla-mime-type]⮳ or media type may be found in the {{hi:Content-Type}}[`Content-Type`][mozilla-content-type]⮳ header. {{hi:reqwest::header::HeaderMap::get}}[`reqwest::header::HeaderMap::get`][c-reqwest::header::HeaderMap::get]⮳ retrieves the {{i:header}} as a {{hi:reqwest::header::HeaderValue}}[`reqwest::header::HeaderValue`][c-reqwest::header::HeaderValue]⮳ which can be converted to a string. The {{hi:mime}}[`mime`][c-mime]⮳ crate can then parse that, yielding a {{hi:mime::Mime}}[`mime::Mime`][c-mime::Mime]⮳ value.

The {{hi:mime}}[`mime`][c-mime]⮳ crate also defines some commonly used MIME types.

Note that the {{hi:reqwest::header}}[`reqwest::header`][c-reqwest::header]⮳ module is exported from the {{hi:http}}[`http`][c-http]⮳ crate.

```rust,editable,no_run
{{#include ../../../deps/tests/request.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
