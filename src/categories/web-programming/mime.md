# Media Types

{{#include mime.incl.md}}

## Get MIME type from string

[![mime][mime-badge]][mime]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

The following example shows how to parse a [`{{i:MIME}}`][c-mime::Mime] type from a string using the [mime][mime]⮳ crate. [`{{i:FromStrError}}`][c-mime::Mime]⮳ produces a default [`{{i:MIME}}`][c-mime::Mime]⮳ type in an [`{{i:unwrap_or}}`][c-std::result::Result::unwrap_or]⮳ clause.

```rust,editable
{{#include ../../../deps/tests/string.rs}}
```

## Get MIME type from filename

[![mime][mime-badge]][mime]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

The following example shows how to return the correct {{i:MIME}} type from a given filename using the [`{{i:mime}}`][mime]⮳ crate. The program will check for file extensions and match against a known list. The return value is [`{{i:mime:Mime}}`][c-mime::Mime]⮳.

```rust,editable
{{#include ../../../deps/tests/filename.rs}}
```

## Parse the MIME type of a HTTP response

[![reqwest][reqwest-badge]][reqwest]  [![mime][mime-badge]][mime]  [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]  [![cat-encoding][cat-encoding-badge]][cat-encoding]  [![cat-web-programming][cat-web-programming-badge]][cat-web-programming] |

When receiving a {{i:HTTP response}} from [`{{i:reqwest}}`][reqwest]⮳ the [`{{i:MIME type}}`][mozilla-mime-type]⮳ or media type may be found in the [`{{i:Content-Type}}`][mozilla-content-type]⮳ header. [`{{i:reqwest::header::HeaderMap::get}}`][reqwest::header::HeaderMap::get]⮳ retrieves the {{i:header}} as a [`{{i:reqwest::header::HeaderValue}}`][reqwest::header::HeaderValue]⮳ which can be converted to a string. The [`{{i:mime}}`][mime]⮳ crate can then parse that, yielding a [`{{i:mime::Mime}}`][c-mime::Mime]⮳ value.

The [`{{i:mime}}`][mime]⮳ crate also defines some commonly used MIME types.

Note that the [`{{i:reqwest::header}}`][c-reqwest::header]⮳ module is exported from the [`{{i:http}}`][http]⮳ crate.

```rust,editable,no_run
{{#include ../../../deps/tests/request.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
