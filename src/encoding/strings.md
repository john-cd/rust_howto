# Character Sets

## Percent-encode a string

[![percent-encoding-badge]][percent-encoding] [![cat-encoding-badge]][cat-encoding]

Encode an input string with [percent-encoding] using the [`utf8_percent_encode`]
function from the `percent-encoding` crate. Then decode using the [`percent_decode`]
function.

```rust,editable
{#include ../../../deps/examples/percent-encode.rs}
```

The encode set defines which bytes (in addition to non-ASCII and controls) need
to be percent-encoded. The choice of this set depends on context. For example,
`url` encodes `?` in a URL path but not in a query string.

The return value of encoding is an iterator of `&str` slices which collect into
a `String`.

## Encode a string as application/x-www-form-urlencoded

[![url-badge]][url] [![cat-encoding-badge]][cat-encoding]

Encodes a string into [application/x-www-form-urlencoded] syntax
using the [`form_urlencoded::byte_serialize`] and subsequently
decodes it with [`form_urlencoded::parse`]. Both functions return iterators
that collect into a `String`.

```rust,editable
{#include ../../../deps/examples/url-encode.rs}
```

## Encode and decode hex

[![data-encoding-badge]][data-encoding] [![cat-encoding-badge]][cat-encoding]

The [`data_encoding`] crate provides a `HEXUPPER::encode` method which
takes a `&[u8]` and returns a `String` containing the hexadecimal
representation of the data.

Similarly, a `HEXUPPER::decode` method is provided which takes a `&[u8]` and
returns a `Vec<u8>` if the input data is successfully decoded.

The example below coverts `&[u8]` data to hexadecimal equivalent.  Compares this
value to the expected value.

```rust,editable
{#include ../../../deps/examples/hex.rs}
```

## Encode and decode base64

[![base64-badge]][base64] [![cat-encoding-badge]][cat-encoding]

Encodes byte slice into `base64` String using [`encode`]
and decodes it with [`decode`].

```rust,editable
{#include ../../../deps/examples/base64.rs}
```

[`form_urlencoded::byte_serialize`]: https://docs.rs/url/*/url/form_urlencoded/fn.byte_serialize.html
[`form_urlencoded::parse`]: https://docs.rs/url/*/url/form_urlencoded/fn.parse.html
[application/x-www-form-urlencoded]: https://url.spec.whatwg.org/#application/x-www-form-urlencoded
[`data_encoding`]: https://docs.rs/data-encoding/*/data_encoding/
[`percent_decode`]: https://docs.rs/percent-encoding/*/percent_encoding/fn.percent_decode.html
[`utf8_percent_encode`]: https://docs.rs/percent-encoding/*/percent_encoding/fn.utf8_percent_encode.html
[percent-encoding]: https://en.wikipedia.org/wiki/Percent-encoding
[`decode`]: https://docs.rs/base64/*/base64/fn.decode.html
[`encode`]: https://docs.rs/base64/*/base64/fn.encode.html
{{#include ../refs/link-refs.md}}
