# Character Sets

{{#include string_encoding.incl.md}}

## Percent-encode a String {#percent-encoding}

[![percent-encoding][c~percent-encoding~docs~badge]][c~percent-encoding~docs]{{hi:percent-encoding}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

Encode an input string with [percent-encoding][wikipedia~percent-encoding]↗ using the [`percent-encoding`][c~percent-encoding~docs]↗{{hi:percent-encoding}} crate. Then decode using the [`percent-encoding::percent_decode`][c~percent-encoding::percent_decode~docs]↗{{hi:percent-encoding::percent_decode}} function.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/percent_encode.rs:example}}
```

The encode set defines which bytes (in addition to non-ASCII and controls) need to be percent-encoded. The choice of this set depends on context. For example, [`url`][c~url~docs]↗{{hi:url}} encodes `?` in a [URL][p~url] path but not in a query string.

The return value of [encoding][p~encoding] is an iterator of `&str` [slices][p~slices] which collect into a [`std::string::String`][c~std::string::String~docs]↗{{hi:std::string::String}}.

## Encode a String as application/x-www-form-urlencoded {#url}

[![url][c~url~docs~badge]][c~url~docs] [![url~crates.io][c~url~crates.io~badge]][c~url~crates.io] [![url~github][c~url~github~badge]][c~url~github] [![url~lib.rs][c~url~lib.rs~badge]][c~url~lib.rs]{{hi:url}}{{hi:Parser}}{{hi:url}}[![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}[![cat~parser-implementations][cat~parser-implementations~badge]][cat~parser-implementations]{{hi:Parser implementations}}[![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}

Encodes a string into [`application/x-www-form-urlencoded`][web-application/x-www-form-urlencoded]↗{{hi:application/x-www-form-urlencoded}} syntax using the [`form_urlencoded::byte_serialize`][c~form_urlencoded::byte_serialize~docs]↗{{hi:form_urlencoded::byte_serialize}} and subsequently decodes it with [`form_urlencoded::parse`][c~form_urlencoded::parse~docs]↗{{hi:form_urlencoded::parse}}. Both functions return iterators that collect into a [`std::string::String`][c~std::string::String~docs]↗{{hi:std::string::String}}.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/url_encode.rs:example}}
```

## Encode and Decode Hexadecimal {#data-encoding}

[![data-encoding][c~data-encoding~docs~badge]][c~data-encoding~docs]{{hi:Data encoding}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

The [`data-encoding`][c~data-encoding~docs]↗{{hi:data-encoding}} crate provides a `HEXUPPER::encode` method which takes a `&[u8]` and returns a [`std::string::String`][c~std::string::String~docs]↗{{hi:std::string::String}} containing the hexadecimal representation{{hi:Hexadecimal representation}} of the data.

Similarly, a [`HEXUPPER::decode`][c~data-encoding::HEXUPPER~docs]↗{{hi:HEXUPPER::decode}} method is provided which takes a `&[u8]` and returns a `Vec<u8>` if the input data is successfully decoded.

The example below coverts `&[u8]` data to hexadecimal equivalent. Compares this value to the expected value.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/hex.rs:example}}
```

## Encode and Decode with `base64` {#base64}

[![base64][c~base64~docs~badge]][c~base64~docs]{{hi:base64}} [![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}}

Encodes byte slice into [`base64`][c~base64~docs]↗{{hi:base64}} {{hi:base64}} String using [`base64::encode`][c~base64::encode~docs]↗{{hi:base64::encode}} and decodes it with [`base64::decode`][c~base64::decode~docs]↗{{hi:base64::decode}}.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/base64.rs:example}}
```

## URL Encoding {#percent-encoding}

[![percent-encoding][c~percent-encoding~docs~badge]][c~percent-encoding~docs] [![percent-encoding~crates.io][c~percent-encoding~crates.io~badge]][c~percent-encoding~crates.io] [![percent-encoding~github][c~percent-encoding~github~badge]][c~percent-encoding~github] [![percent-encoding~lib.rs][c~percent-encoding~lib.rs~badge]][c~percent-encoding~lib.rs]{{hi:percent-encoding}}

[`percent-encoding`][c~percent-encoding~docs]↗{{hi:percent-encoding}} handles URL encoding and decoding.

```rust,editable
{{#include ../../../crates/cats/encoding/examples/string_encoding/percent-encoding.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/930)
</div>
