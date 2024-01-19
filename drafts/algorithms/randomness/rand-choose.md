## Create random passwords from a set of user-defined characters

[![rand-badge]][rand] [![cat-os-badge]][cat-os]

Randomly generates a string of given length ASCII characters with custom
user-defined bytestring, with [`gen_range`].

```rust,editable
{{#include ../../deps/examples/rand-choose.rs}}
```

[`gen_range`]: https://docs.rs/rand/*/rand/trait.Rng.html#method.gen_range

{{#include ../refs/link-refs.md}}
