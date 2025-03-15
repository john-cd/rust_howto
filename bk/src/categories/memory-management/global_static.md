# Global Static

{{#include global_static.incl.md}}

## Declare Lazily Evaluated Constants {#declare-lazily-evaluated-constant}

[![lazy_static][c-lazy_static-badge]][c-lazy_static]{{hi:lazy_static}}
[![lazy_static-crates.io][c-lazy_static-crates.io-badge]][c-lazy_static-crates.io]
[![lazy_static-github][c-lazy_static-github-badge]][c-lazy_static-github]
[![lazy_static-lib.rs][c-lazy_static-lib.rs-badge]][c-lazy_static-lib.rs]
[![cat-caching][cat-caching-badge]][cat-caching]{{hi:Caching}}
[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}
[![cat-memory-management][cat-memory-management-badge]][cat-memory-management]{{hi:Memory management}}

Declares a lazily evaluated constant [`std::collections::HashMap`][c-std::collections::HashMap]{{hi:std::collections::HashMap}}⮳. The [`std::collections::HashMap`][c-std::collections::HashMap]{{hi:std::collections::HashMap}}⮳ will be evaluated once and stored behind a global static reference.

```rust,editable
{{#include ../../../crates/cats/memory_management/tests/lazy_static/lazy_constant.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[global_static: expand - `once_cell` crate and in `std`](https://github.com/john-cd/rust_howto/issues/409)

[merge global_static.md and [lazy_initialization][p-lazy-initialization].md; review lazy_constant.rs and lazy_static.rs and global_mut_state.rs](https://github.com/john-cd/rust_howto/issues/939)

Immutable Global: static (compile-time init, limited).
Lazy Init: [`lazy_static`][c-lazy_static]⮳{{hi:lazy_static}} (runtime init, simple), [`once_cell`][c-once_cell]⮳{{hi:once_cell}} (runtime init, more control).
Mutable Global: parking_lot::Mutex/RwLock (thread-safe).
Thread-Local: std::thread_local.
Atomic: std::sync::atomic.
Prefer [`once_cell`][c-once_cell]⮳{{hi:once_cell}} over lazy_static. Use mutexes/rwlocks for mutable globals. Consider alternatives to globals.
</div>
