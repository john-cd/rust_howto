# MS SQL Server

{{#include mssql.incl.md}}

## `tiberius` {#tiberius}

[![tiberius][c~tiberius~docs~badge]][c~tiberius~docs] [![tiberius~crates.io][c~tiberius~crates.io~badge]][c~tiberius~crates.io] [![tiberius~github][c~tiberius~github~badge]][c~tiberius~github] [![tiberius~lib.rs][c~tiberius~lib.rs~badge]][c~tiberius~lib.rs]{{hi:tiberius}}{{hi:Sql}}{{hi:TDS}}{{hi:MSSQL}}

[`tiberius`][c~tiberius~docs]↗{{hi:tiberius}} is a TDS (MSSQL) driver. It is a fully asynchronous, pure Rust implementation of the TDS protocol. It is built on top of [`tokio`][c~tokio~docs]↗{{hi:tokio}} and [`futures`][c~futures~docs]↗{{hi:futures}}.

```rust,editable,noplayground
{{#include ../../../crates/cats/database/examples/mssql/tiberius.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[organize / write](https://github.com/john-cd/rust_howto/issues/1067)
</div>
