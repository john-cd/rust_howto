# Character Sets

{{#include strings.incl.md}}

## Percent-encode a string

[![percent_encoding][c-percent_encoding-badge]][c-percent_encoding]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Encode an input string with [percent_encoding][wikipedia-percent_encoding]⮳ using the {{hi:utf8_percent_encode}}[`utf8_percent_encode`][c-percent_encoding::utf8_percent_encode]⮳ function from the `percent_encoding`{{hi:percent_encoding}} crate. Then decode using the {{hi:percent_decode}}[`percent_decode`][c-percent_encoding::percent_decode]⮳ function.

```rust,editable
{{#include ../../../deps/tests/percent-encode.rs}}
```

The encode set defines which bytes (in addition to non-ASCII and controls) need to be percent-encoded. The choice of this set depends on context. For example, `url` encodes `?` in a URL path but not in a query string.

The return value of encoding is an iterator of `&str` slices which collect into a {{hi:String}}[`String`][c-std::string::String]⮳.

## Encode a string as application/x-www-form-urlencoded

[![url][c-url-badge]][c-url]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Encodes a string into {{hi:application/x-www-form-urlencoded}}[`application/x-www-form-urlencoded`][web-application/x-www-form-urlencoded] syntax using the {{hi:form_urlencoded::byte_serialize}}[`form_urlencoded::byte_serialize`][c-form_urlencoded::byte_serialize]⮳ and subsequently decodes it with {{hi:form_urlencoded::parse}}[`form_urlencoded::parse`][c-form_urlencoded::parse]⮳. Both functions return iterators that collect into a {{hi:String}}[`String`][c-std::string::String]⮳.

```rust,editable
{{#include ../../../deps/tests/url-encode.rs}}
```

## Encode and decode hex

[![data_encoding][c-data_encoding-badge]][c-data_encoding]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

The {{hi:data_encoding}}[`data_encoding`][c-data_encoding]⮳ crate provides a `HEXUPPER::encode` method which takes a `&[u8]` and returns a {{hi:String}}[`String`][c-std::string::String]⮳ containing the {{i:hexadecimal representation}} of the data.

Similarly, a `HEXUPPER::decode` method is provided which takes a `&[u8]` and returns a `Vec<u8>` if the input data is successfully decoded.

The example below coverts `&[u8]` data to hexadecimal equivalent. Compares this value to the expected value.

```rust,editable
{{#include ../../../deps/tests/hex.rs}}
```

## Encode and decode base64

[![base64][c-base64-badge]][c-base64]  [![cat-encoding][cat-encoding-badge]][cat-encoding]

Encodes byte slice into `base64`{{hi:base64}} String using {{hi:encode}}[`encode`][c-base64::encode] and decodes it with {{hi:decode}}[`decode`][c-base64::decode].

```rust,editable
{{#include ../../../deps/tests/base64.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
