## Extract phone numbers from text

[![regex-badge]][regex] [![cat-text-processing-badge]][cat-text-processing]

Processes a string of text using [`Regex::captures_iter`] to capture multiple
phone numbers.  The example here is for US convention phone numbers.

```rust,editable
{#include ../../../deps/examples/phone.rs}
```

[`Regex::captures_iter`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.captures_iter
