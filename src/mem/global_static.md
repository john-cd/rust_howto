# Constants

## Declare lazily evaluated constant

[![lazy-static-badge]][lazy-static]  [![cat-caching-badge]][cat-caching]  [![cat-rust-patterns-badge]][cat-rust-patterns]

Declares a lazily evaluated constant [`HashMap`][std::collections::HashMap] The [`HashMap`][std::collections::HashMap] will be evaluated once and stored behind a global static reference.

```rust,editable
{{#include ../../deps/tests/lazy-constant.rs}}
```

{{#include ../refs/link-refs.md}}
