# Character Sets

{{#include string_encoding.incl.md}}

## Percent-encode a string {#percent-encoding}

[![percent_encoding][c-percent_encoding-badge]][c-percent_encoding]{{hi:percent_encoding}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

Encode an input string with [percent_encoding][wikipedia-percent_encoding]⮳ using the [`percent_encoding`][c-percent_encoding]⮳{{hi:percent_encoding}} crate. Then decode using the [`percent_encoding::percent_decode`][c-percent_encoding::percent_decode]{{hi:percent_encoding::percent_decode}}⮳ function.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/strings/percent_encode.rs:example}}
```

The encode set defines which bytes (in addition to non-ASCII and controls) need to be percent-encoded. The choice of this set depends on context. For example, [`url`][c-url]⮳{{hi:url}} encodes `?` in a [URL][p-url] path but not in a query string.

The return value of [encoding][p-encoding] is an iterator of `&str` [slices][p-slices] which collect into a [`std::string::String`][c-std::string::String]{{hi:std::string::String}}⮳.

## Encode a string as application/x-www-form-urlencoded {#url}

[![url][c-url-badge]][c-url] [![url-crates.io][c-url-crates.io-badge]][c-url-crates.io] [![url-github][c-url-github-badge]][c-url-github] [![url-lib.rs][c-url-lib.rs-badge]][c-url-lib.rs]{{hi:url}}{{hi:Parser}}{{hi:url}}[![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}[![cat-parser-implementations][cat-parser-implementations-badge]][cat-parser-implementations]{{hi:Parser implementations}}[![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

Encodes a string into [`application/x-www-form-urlencoded`][web-application/x-www-form-urlencoded]{{hi:application/x-www-form-urlencoded}} syntax using the [`form_urlencoded::byte_serialize`][c-form_urlencoded::byte_serialize]{{hi:form_urlencoded::byte_serialize}}⮳ and subsequently decodes it with [`form_urlencoded::parse`][c-form_urlencoded::parse]{{hi:form_urlencoded::parse}}⮳. Both functions return iterators that collect into a [`std::string::String`][c-std::string::String]{{hi:std::string::String}}⮳.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/strings/url_encode.rs:example}}
```

## Encode and decode hexadecimal {#data-encoding}

[![data-encoding][c-data_encoding-badge]][c-data_encoding]{{hi:Data encoding}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

The [`data_encoding`][c-data_encoding]{{hi:data-encoding}}⮳ crate provides a `HEXUPPER::encode` method which takes a `&[u8]` and returns a [`std::string::String`][c-std::string::String]{{hi:std::string::String}}⮳ containing the hexadecimal representation{{hi:Hexadecimal representation}} of the data.

Similarly, a `HEXUPPER::decode` method is provided which takes a `&[u8]` and returns a `Vec<u8>` if the input data is successfully decoded.

The example below coverts `&[u8]` data to hexadecimal equivalent. Compares this value to the expected value.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/strings/hex.rs:example}}
```

## Encode and decode base64 {#base64}

[![base64][c-base64-badge]][c-base64]{{hi:base64}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}

Encodes byte slice into [`base64`][c-base64]⮳{{hi:base64}} {{hi:base64}} String using [`base64::encode`][c-base64::encode]{{hi:base64::encode}} and decodes it with [`base64::decode`][c-base64::decode]{{hi:base64::decode}}.

```rust,editable
{{#include ../../../crates/cats/encoding/tests/strings/base64.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/930)

## URL Encoding {#percent-encoding}

[![percent-encoding][c-percent_encoding-badge]][c-percent_encoding] [![percent-encoding-crates.io][c-percent_encoding-crates.io-badge]][c-percent_encoding-crates.io] [![percent-encoding-github][c-percent_encoding-github-badge]][c-percent_encoding-github] [![percent-encoding-lib.rs][c-percent_encoding-lib.rs-badge]][c-percent_encoding-lib.rs]{{hi:percent-encoding}}

[`percent-encoding`][c-percent_encoding]⮳{{hi:percent-encoding}} handles URL encoding and decoding.

{{#example percent-encoding}}
</div>
