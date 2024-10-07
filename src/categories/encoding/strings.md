# Character Sets

{{#include strings.incl.md}}

## Percent-encode a string

[![percent-encoding][percent-encoding-badge]][percent-encoding]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Encode an input string with [percent-encoding][wikipedia-percent-encoding]⮳ using the [`utf8_percent_encode`][percent_encoding::utf8_percent_encode]⮳ function from the {{i:`percent-encoding`}} crate. Then decode using the [`percent_decode`][percent_encoding::percent_decode]⮳ function.

```rust,editable
{{#include ../../../deps/tests/percent-encode.rs}}
```

The encode set defines which bytes (in addition to non-ASCII and controls) need to be percent-encoded. The choice of this set depends on context. For example, `url` encodes `?` in a URL path but not in a query string.

The return value of encoding is an iterator of `&str` slices which collect into a `String`.

## Encode a string as application/x-www-form-urlencoded

[![url][url-badge]][url]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Encodes a string into [`application/x-www-form-urlencoded`][application/x-www-form-urlencoded] syntax using the [`form_urlencoded::byte_serialize`][form_urlencoded::byte_serialize]⮳ and subsequently decodes it with [`form_urlencoded::parse`][form_urlencoded::parse]⮳. Both functions return iterators that collect into a `String`.

```rust,editable
{{#include ../../../deps/tests/url-encode.rs}}
```

## Encode and decode hex

[![data-encoding][data-encoding-badge]][data-encoding]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

The [`data_encoding`][data-encoding]⮳ crate provides a `HEXUPPER::encode` method which takes a `&[u8]` and returns a `String` containing the {{i:hexadecimal representation}} of the data.

Similarly, a `HEXUPPER::decode` method is provided which takes a `&[u8]` and returns a `Vec<u8>` if the input data is successfully decoded.

The example below coverts `&[u8]` data to hexadecimal equivalent. Compares this value to the expected value.

```rust,editable
{{#include ../../../deps/tests/hex.rs}}
```

## Encode and decode base64

[![base64][base64-badge]][base64]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Encodes byte slice into {{i:`base64`}} String using [`encode`][base64::encode] and decodes it with [`decode`][base64::decode].

```rust,editable
{{#include ../../../deps/tests/base64.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
