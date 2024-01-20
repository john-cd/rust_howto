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

[`percent_decode`]: https://docs.rs/percent-encoding/*/percent_encoding/fn.percent_decode.html
[`utf8_percent_encode`]: https://docs.rs/percent-encoding/*/percent_encoding/fn.utf8_percent_encode.html

[percent-encoding]: https://en.wikipedia.org/wiki/Percent-encoding
