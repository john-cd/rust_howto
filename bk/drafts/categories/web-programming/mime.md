<!-- TODO Reviewed ??-??-2026 -->
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

The following example guesses a MIME type from the filename extension with [`mime_guess`][c~mime_guess~docs]↗{{hi:mime_guess}}. It returns [`mime::Mime`][c~mime::Mime~docs]↗{{hi:mime::Mime}} and uses `application/octet-stream` when the extension is unknown. This is an extension-based guess; it does not inspect the file's bytes.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/mime/filename.rs:example}}
```

## Parse the MIME Type of a HTTP Response {#parse-the-mime-type-of-a-http-response}

[![reqwest][c~reqwest~docs~badge]][c~reqwest~docs]{{hi:reqwest}} [![mime][c~mime~docs~badge]][c~mime~docs]{{hi:mime}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} [![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

When receiving an HTTP response from [`reqwest`][c~reqwest~docs]↗{{hi:reqwest}}, the media type is available in the [`Content-Type`][mozilla~content-type]↗{{hi:Content type}} header. The example converts the header value to text and parses it with [`mime::Mime`][c~mime::Mime~docs]↗{{hi:mime::Mime}}. It checks the response status before examining the header and reports a missing or malformed media type explicitly.

The [`mime`][c~mime~docs]↗{{hi:mime}} crate also defines some commonly used MIME types.

Note that the [`reqwest::header`][c~reqwest::header~docs]↗{{hi:reqwest::header}} module is exported from the [`http`][c~http~docs]↗{{hi:http}} crate.

```rust,editable
{{#include ../../../crates/cats/web_programming/examples/mime/request.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[requests | Make HTTP Requests]].
- [[filesystem | Filesystem]].

To identify a file from its contents instead of its extension, use a content-sniffing crate such as [`infer`](https://docs.rs/infer/latest/infer/).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
This review replaces the hand-written extension match with `mime_guess`, documents that its result comes from the filename, and adds local tests for response `Content-Type` parsing. The live HTTP example is now opt-in.
</div>
