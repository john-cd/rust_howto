## Encode a string as application/x-www-form-urlencoded

[![url-badge]][url] [![cat-encoding-badge]][cat-encoding]

Encodes a string into [application/x-www-form-urlencoded] syntax
using the [`form_urlencoded::byte_serialize`] and subsequently
decodes it with [`form_urlencoded::parse`]. Both functions return iterators
that collect into a `String`.

```rust,editable
{#include ../../../deps/examples/url-encode.rs}
```

[`form_urlencoded::byte_serialize`]: https://docs.rs/url/*/url/form_urlencoded/fn.byte_serialize.html
[`form_urlencoded::parse`]: https://docs.rs/url/*/url/form_urlencoded/fn.parse.html

[application/x-www-form-urlencoded]: https://url.spec.whatwg.org/#application/x-www-form-urlencoded
