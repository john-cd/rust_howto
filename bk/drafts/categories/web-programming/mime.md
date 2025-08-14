# Media Types

{{#include mime.incl.md}}

## Get a MIME Type from a String {#get-mime-type-from-string}

[![mime][c~mime~docs~badge]][c~mime~docs]{{hi:mime}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

The following example shows how to [parse][p~parse] a [`mime::Mime`][c~mime::Mime~docs]↗{{hi:mime::Mime}} type from a string using the [mime][c~mime~docs]↗{{hi:mime}} crate. [`mime::Mime`][c~mime::Mime~docs]↗{{hi:mime::Mime}} produces a default [`mime::Mime`][c~mime::Mime~docs]↗{{hi:mime::Mime}} type in an [`std::result::Result::unwrap_or`][c~std::result::Result::unwrap_or~docs]↗{{hi:std::result::Result::unwrap_or}} clause.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/mime/string.rs:example}}
```

## Get a MIME Type from a Filename {#get-mimetype-from-filename}

[![mime][c~mime~docs~badge]][c~mime~docs]{{hi:mime}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

The following example shows how to return the correct MIME{{hi:MIME type}} type from a given filename using the [`mime`][c~mime~docs]↗{{hi:mime}} crate. The program will check for file extensions and match against a known list. The return value is [`mime::Mime`][c~mime::Mime~docs]↗{{hi:mime::Mime}}.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/mime/filename.rs:example}}
```

## Parse the MIME Type of a HTTP Response {#parse-the-mime-type-of-a-http-response}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![mime][c~mime~docs~badge]][c~mime~docs]{{hi:mime}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

When receiving a HTTP response{{hi:HTTP response}} from [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}} the [`MIME type`][mozilla~mime-type]↗{{hi:MIME type}} or media type may be found in the [`Content-Type`][mozilla~content-type]↗{{hi:Content type}} header. [`reqwest::header::HeaderMap::get`][c~reqwest::header::HeaderMap::get~docs]↗{{hi:reqwest::header::HeaderMap::get}} retrieves the header{{hi:Header}} as a [`reqwest::header::HeaderValue`][c~reqwest::header::HeaderValue~docs]↗{{hi:reqwest::header::HeaderValue}} which can be converted to a string. The [`mime`][c~mime~docs]↗{{hi:mime}} crate can then parse that, yielding a [`mime::Mime`][c~mime::Mime~docs]↗{{hi:mime::Mime}} value.

The [`mime`][c~mime~docs]↗{{hi:mime}} crate also defines some commonly used MIME types.

Note that the [`reqwest::header`][c~reqwest::header~docs]↗{{hi:reqwest::header}} module is exported from the [`http`][c~http~docs]↗{{hi:http}} crate.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/mime/request.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/971)
</div>
