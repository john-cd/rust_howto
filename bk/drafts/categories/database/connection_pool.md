# Connection Pool

{{#include connection_pool.incl.md}}

## Create a Connection Pool {#connection-pool}

[![deadpool][c~deadpool~docs~badge]][c~deadpool~docs] [![deadpool~crates.io][c~deadpool~crates.io~badge]][c~deadpool~crates.io] [![deadpool~repo][c~deadpool~repo~badge]][c~deadpool~repo] [![deadpool~lib.rs][c~deadpool~lib.rs~badge]][c~deadpool~lib.rs]{{hi:deadpool}}{{hi:Pool}}{{hi:Async}}{{hi:Database}}

[`deadpool`][c~deadpool~docs]↗{{hi:deadpool}} is a simple async pool for connections and objects of any type.

```rust,editable
{{#include ../../../crates/cats/database/examples/connection_pool/deadpool.rs:example}}
```

Here is an example demonstrating the use of [`deadpool`][c~deadpool~docs]↗{{hi:deadpool}} to connect to a [Postgres][p~postgres] database:

```rust,editable
{{#include ../../../crates/cats/database/examples/connection_pool/deadpool2.rs:example}}
```

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[connection_pool: expand; example](https://github.com/john-cd/rust_howto/issues/284)
</div>
