# Connection pool

{{#include connection_pool.incl.md}}

## Create a connection pool {#connection-pool}

[![deadpool][c-deadpool-badge]][c-deadpool] [![deadpool-crates.io][c-deadpool-crates.io-badge]][c-deadpool-crates.io] [![deadpool-github][c-deadpool-github-badge]][c-deadpool-github] [![deadpool-lib.rs][c-deadpool-lib.rs-badge]][c-deadpool-lib.rs]{{hi:deadpool}}{{hi:Pool}}{{hi:Async}}{{hi:Database}}

`deadpool` is a simple async pool for connections and objects of any type.

```rust,editable
{{#include ../../../crates/ex/categories/d/tests/database/deadpool.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[connection_pool: expand; example (P1)](https://github.com/john-cd/rust_howto/issues/284)
</div>
