<!-- TODO Reviewed ??-??-2026 -->

# Character Sets

{{#include string_encoding.incl.md}}

## Text and UTF-8 {#text-and-utf-8}

Rust `String` and `&str` values always contain valid UTF-8. A string's `.len()` is its byte length, while `.chars()` iterates Unicode scalar values (`char`), not user-perceived characters. A visible symbol may be made of multiple scalar values; use a grapheme segmentation library such as [`unicode-segmentation`][c~unicode-segmentation~docs]↗ when text editing or display needs grapheme clusters.

Converting arbitrary bytes with `String::from_utf8` checks that the bytes are valid UTF-8 and returns an error otherwise. `String::from_utf8_lossy` replaces invalid sequences with the replacement character. For UTF-16 or a legacy character encoding, decode with an encoding-specific API and choose deliberately how invalid input is handled; do not treat arbitrary bytes as a Rust string.

## Percent-encode a String {#percent-encoding}

[![percent-encoding][c~percent-encoding~docs~badge]][c~percent-encoding~docs]{{hi:percent-encoding}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

Percent encoding represents bytes as `%` followed by two hexadecimal digits. The correct encode set depends on the URL component and protocol. Do not percent-encode an entire URL with one generic set. This example defines a set for a URL fragment and decodes the result back to UTF-8.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/percent_encode.rs:example}}
```

## Encode a URL Component {#url-component-encoding}

[![percent-encoding][c~percent-encoding~docs~badge]][c~percent-encoding~docs]{{hi:percent-encoding}}

The [`percent-encoding`][c~percent-encoding~docs]↗ crate also provides common encode sets. The example encodes one component and checks that the decoded bytes form valid UTF-8. For complete URL parsing and construction, use a URL API rather than concatenating encoded pieces by hand.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/percent_encoding.rs:example}}
```

## Encode Form Data as `application/x-www-form-urlencoded` {#url}

{{#badges url}}

Form encoding uses `+` for spaces and percent-escapes other bytes as required by `application/x-www-form-urlencoded`. This is a distinct format from percent-encoding an arbitrary URL component. The `url` crate provides iterators for encoding and parsing form data.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/url_encode.rs:example}}
```

## Encode and Decode Hexadecimal {#data-encoding}

[![data-encoding][c~data-encoding~docs~badge]][c~data-encoding~docs]{{hi:Data encoding}} [![cat~encoding][cat~encoding~badge]][cat~encoding]

Hexadecimal represents each byte as two base-16 digits. The [`data-encoding`][c~data-encoding~docs]↗ crate exposes constants such as `HEXUPPER` for encoding and decoding byte sequences. Decoding can fail when the input contains invalid characters or has an invalid length.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/hex.rs:example}}
```

## Encode and Decode with `base64` {#base64}

[![base64][c~base64~docs~badge]][c~base64~docs]{{hi:base64}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

Base64 represents arbitrary bytes as ASCII text; it is not encryption or compression. The current `base64` crate API uses an `Engine`, such as `BASE64_STANDARD`, to encode a byte slice and decode the resulting text. Decoding returns bytes, which must be validated or decoded separately if they represent text.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/base64.rs:example}}
```

## Encode and Decode Hexadecimal with `hex` {#hex}

{{#badges hex encoding=}}

The [`hex`][c~hex~docs]↗{{hi:hex}} crate offers `hex::encode`, which produces lowercase hexadecimal, `hex::decode`, and `hex::decode_to_slice` for fixed-size buffers. Invalid input returns a `FromHexError`, for example for an odd length or a character that is not a hex digit. [`data-encoding`][c~data-encoding~docs]↗ (shown above) and `const-hex` are alternatives.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/hex_crate.rs:example}}
```

## Encode Forms and Path Segments with `serde_urlencoded` and `urlencoding` {#form-urlencoded}

{{#badges serde_urlencoded}} [![urlencoding][c~urlencoding~docs~badge]][c~urlencoding~docs]{{hi:urlencoding}} [![cat~encoding][cat~encoding~badge]][cat~encoding]

[`serde_urlencoded`][c~serde_urlencoded~docs]↗{{hi:serde_urlencoded}} serializes a struct to an `application/x-www-form-urlencoded` string (a query string or form body) and parses it back, into a struct or a map. Spaces become `+`. [`urlencoding`][c~urlencoding~docs]↗{{hi:urlencoding}} escapes a single path or query segment; a space becomes `%20`.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/form_urlencoded.rs:example}}
```

## Related Topics {#related-topics .skip}

- [Rust strings][p~strings].
- [Encoding and serialization][p~encoding].
- [URL parsing and construction][p~url].
- [Unique Identifiers][p~unique-identifiers].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [review](https://github.com/john-cd/rust_howto/issues/930)
</div>
