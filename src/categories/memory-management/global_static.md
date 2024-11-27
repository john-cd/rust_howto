# Global static

{{#include global_static.incl.md}}

## Declare lazily evaluated constant {#declare-lazily-evaluated-constant}

[![lazy_static][c-lazy_static-badge]][c-lazy_static]{{hi:lazy_static}}
[![lazy_static-crates.io][c-lazy_static-crates.io-badge]][c-lazy_static-crates.io]
[![lazy_static-github][c-lazy_static-github-badge]][c-lazy_static-github]
[![lazy_static-lib.rs][c-lazy_static-lib.rs-badge]][c-lazy_static-lib.rs]
[![cat-caching][cat-caching-badge]][cat-caching]{{hi:Caching}}
[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}
[![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}}

Declares a lazily evaluated constant [`std::collections::HashMap`][c-std::collections::HashMap]{{hi:std::collections::HashMap}}⮳. The [`std::collections::HashMap`][c-std::collections::HashMap]{{hi:std::collections::HashMap}}⮳ will be evaluated once and stored behind a global static reference.

```rust,editable
{{#include ../../../deps/tests/cats/memory_management/lazy_constant.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1
</div>
