# Constants

{{#include global_static.incl.md}}

## Declare lazily evaluated constant

[![lazy-static][lazy-static-badge]][lazy-static]  [![cat-caching][cat-caching-badge]][cat-caching]  [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]  [![cat-memory-management][cat-memory-management-badge]][cat-memory-management]

Declares a lazily evaluated constant [`HashMap`][std::collections::HashMap] The [`HashMap`][std::collections::HashMap] will be evaluated once and stored behind a global static reference.

```rust,editable
{{#include ../../../deps/tests/lazy-constant.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
