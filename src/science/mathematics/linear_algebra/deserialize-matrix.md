## (De)-Serialize a Matrix

[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Serialize and deserialize a matrix to and from JSON. Serialization is taken care of
by [`serde_json::to_string`] and [`serde_json::from_str`] performs deserialization.

Note that serialization followed by deserialization gives back the original matrix.

```rust
{#include ../../../deps/examples/deserialize-matrix.rs}
```

[`serde_json::to_string`]: https://docs.rs/serde_json/*/serde_json/fn.to_string.html
[`serde_json::from_str`]: https://docs.rs/serde_json/*/serde_json/fn.from_str.html
